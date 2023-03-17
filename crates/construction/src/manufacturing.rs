use std::{collections::VecDeque, time::Duration};

use ahash::AHashMap;
use bevy::prelude::*;
use de_core::{
    baseset::GameSet,
    cleanup::DespawnOnGameExit,
    gamestate::GameState,
    gconfig::GameConfig,
    objects::{ActiveObjectType, ObjectType, UnitType, PLAYER_MAX_UNITS},
    player::Player,
    projection::{ToAltitude, ToFlat},
    state::AppState,
};
use de_objects::{IchnographyCache, ObjectCache};
use de_pathing::{PathQueryProps, PathTarget, UpdateEntityPath};
use de_spawner::{ObjectCounter, SpawnBundle};

const MANUFACTURING_TIME: Duration = Duration::from_secs(2);

pub(crate) struct ManufacturingPlugin;

impl Plugin for ManufacturingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnqueueAssemblyEvent>()
            .add_system(
                configure
                    .in_base_set(GameSet::PostUpdate)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_system(
                enqueue
                    .in_base_set(GameSet::Update)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_system(
                produce
                    .in_base_set(GameSet::PreUpdate)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

/// Send this event to enqueue a unit to be manufactured by a factory.
pub struct EnqueueAssemblyEvent {
    factory: Entity,
    unit: UnitType,
}

impl EnqueueAssemblyEvent {
    /// # Arguments
    ///
    /// `factory` - the building to produce the unit.
    ///
    /// `unit` - unit to be produced.
    pub fn new(factory: Entity, unit: UnitType) -> Self {
        Self { factory, unit }
    }

    fn factory(&self) -> Entity {
        self.factory
    }

    fn unit(&self) -> UnitType {
        self.unit
    }
}

/// An assembly line attached to every building and capable of production of
/// any units.
#[derive(Component, Default)]
struct AssemblyLine {
    queue: VecDeque<ProductionItem>,
}

impl AssemblyLine {
    /// Put another unit into the manufacturing queue.
    fn enqueue(&mut self, unit: UnitType) {
        self.queue.push_back(ProductionItem::new(unit));
    }

    /// In case the assembly line is stopped, restart the production.
    ///
    /// # Arguments
    ///
    /// * `time` - elapsed time since a fixed point in time in the past.
    fn restart(&mut self, time: Duration) {
        if let Some(item) = self.queue.front_mut().filter(|item| !item.is_active()) {
            item.restart(time);
        }
    }

    /// In case the assembly line is actively manufacturing some units, stop
    /// it.
    ///
    /// # Arguments
    ///
    /// * `time` - elapsed time since a fixed point in time in the past.
    fn stop(&mut self, time: Duration) {
        if let Some(item) = self.queue.front_mut().filter(|item| item.is_active()) {
            item.stop(time);
        }
    }

    /// Update the production line.
    ///
    /// This method should be called repeatedly and during every tick until it
    /// returns None. The returned values correspond to finished units.
    ///
    /// # Arguments
    ///
    /// * `time` - elapsed time since a fixed point in time in the past.
    fn produce(&mut self, time: Duration) -> Option<UnitType> {
        if let Some(time_past) = self.queue.front().and_then(|item| item.finished(time)) {
            let item = self.queue.pop_front().unwrap();

            if item.is_active() {
                if let Some(next) = self.queue.front_mut() {
                    next.restart(time - time_past);
                }
            }

            Some(item.unit())
        } else {
            None
        }
    }
}

/// A single unit being manufactured / enqueued for manufacturing in an
/// assembly line.
struct ProductionItem {
    /// Total accumulated production time of the item until the last
    /// stop/restart to the manufacturing.
    accumulated: Duration,
    /// Time elapsed since a fixed point in the past until when manufacturing
    /// of the unit was restarted for the last time.
    restarted: Option<Duration>,
    unit: UnitType,
}

impl ProductionItem {
    fn new(unit: UnitType) -> Self {
        Self {
            accumulated: Duration::ZERO,
            restarted: None,
            unit,
        }
    }

    fn unit(&self) -> UnitType {
        self.unit
    }

    /// Returns true if the unit is actively manufactured.
    fn is_active(&self) -> bool {
        self.restarted.is_some()
    }

    /// Restarts (stops and starts) manufacturing of the unit.
    fn restart(&mut self, time: Duration) {
        self.stop(time);
        self.restarted = Some(time);
    }

    /// Stops manufacturing of the unit if it is currently being manufactured.
    ///
    /// Total accumulated manufacturing time is clipped to the time it takes to
    /// produce the unit.
    fn stop(&mut self, time: Duration) {
        if let Some(last) = self.restarted {
            self.accumulated += time - last;
            if self.accumulated > MANUFACTURING_TIME {
                self.accumulated = MANUFACTURING_TIME;
            }
        }
        self.restarted = None;
    }

    /// Returns None if the unit is not yet finished. Otherwise, it returns for
    /// how long it has been finished.
    fn finished(&self, time: Duration) -> Option<Duration> {
        let progress = self.progress(time);
        if progress >= MANUFACTURING_TIME {
            Some(progress - MANUFACTURING_TIME)
        } else {
            None
        }
    }

    /// Returns for how long cumulatively the unit has been manufactured.
    fn progress(&self, time: Duration) -> Duration {
        self.accumulated
            + self
                .restarted
                .map_or(Duration::ZERO, |restarted| time - restarted)
    }
}

fn configure(mut commands: Commands, new: Query<(Entity, &ObjectType), Added<ObjectType>>) {
    for (entity, &object_type) in new.iter() {
        if matches!(
            object_type,
            ObjectType::Active(ActiveObjectType::Building(_))
        ) {
            commands.entity(entity).insert(AssemblyLine::default());
        }
    }
}

fn enqueue(mut events: EventReader<EnqueueAssemblyEvent>, mut lines: Query<&mut AssemblyLine>) {
    for event in events.iter() {
        let Ok(mut line) = lines.get_mut(event.factory()) else { continue };
        info!(
            "Enqueueing manufacturing of {} in {:?}.",
            event.unit(),
            event.factory()
        );
        line.enqueue(event.unit());
    }
}

fn produce(
    mut commands: Commands,
    time: Res<Time>,
    cache: Res<ObjectCache>,
    conf: Res<GameConfig>,
    counter: Res<ObjectCounter>,
    mut path_events: EventWriter<UpdateEntityPath>,
    mut factories: Query<(Entity, &ObjectType, &Transform, &Player, &mut AssemblyLine)>,
) {
    let mut counts: AHashMap<Player, u32> = AHashMap::new();
    for player in conf.players() {
        let count = counter.player(player).unwrap().unit_count();
        counts.insert(player, count);
    }

    for (factory, &factory_object_type, transform, &player, mut assembly) in factories.iter_mut() {
        let player_count = counts.get_mut(&player).unwrap();
        if *player_count < PLAYER_MAX_UNITS {
            assembly.restart(time.elapsed());
        }

        loop {
            if *player_count >= PLAYER_MAX_UNITS {
                assembly.stop(time.elapsed());
                break;
            }

            let Some(unit_type) = assembly.produce(time.elapsed()) else { break };
            *player_count += 1;

            info!(
                "Manufacturing of {unit_type} in {:?} just finished.",
                factory
            );

            let unit_object_type = ObjectType::Active(ActiveObjectType::Unit(unit_type));

            let local_aabb = cache.get_ichnography(factory_object_type).local_aabb();
            let center: Vec2 = local_aabb.center().into();
            let direction = Vec2::new(local_aabb.half_extents().x + 40., 0.);
            let target = transform
                .transform_point((center + direction).to_msl())
                .to_flat();
            let center = transform.transform_point(center.to_msl());

            let unit = commands
                .spawn((
                    SpawnBundle::new(unit_object_type, Transform::from_translation(center)),
                    player,
                    DespawnOnGameExit,
                ))
                .id();
            path_events.send(UpdateEntityPath::new(
                unit,
                PathTarget::new(target, PathQueryProps::new(0., f32::INFINITY), false),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assembly_line() {
        let mut line = AssemblyLine::default();

        line.restart(Duration::from_secs(0));
        assert!(line.produce(Duration::from_secs(20)).is_none());

        line.enqueue(UnitType::Attacker);
        line.enqueue(UnitType::Attacker);
        line.restart(Duration::from_secs(20));
        assert!(line.produce(Duration::from_secs(21)).is_none());
        assert_eq!(
            line.produce(Duration::from_secs(23)).unwrap(),
            UnitType::Attacker
        );
        assert!(line.produce(Duration::from_secs(23)).is_none());
        assert_eq!(
            line.produce(Duration::from_secs(24)).unwrap(),
            UnitType::Attacker
        );
        assert!(line.produce(Duration::from_secs(30)).is_none());

        line.enqueue(UnitType::Attacker);
        line.enqueue(UnitType::Attacker);
        line.restart(Duration::from_secs(50));
        assert!(line.produce(Duration::from_secs(51)).is_none());
        line.stop(Duration::from_secs(51));
        line.restart(Duration::from_secs(60));
        assert!(line.produce(Duration::from_secs_f32(60.5)).is_none());
        assert_eq!(
            line.produce(Duration::from_secs(61)).unwrap(),
            UnitType::Attacker
        );
        assert_eq!(
            line.produce(Duration::from_secs(63)).unwrap(),
            UnitType::Attacker
        );
        assert!(line.produce(Duration::from_secs(90)).is_none());
    }
}