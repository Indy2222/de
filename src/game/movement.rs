use super::{objects::Movable, Labels};
use bevy::prelude::*;
use glam::Vec2;
use std::f32::consts::{FRAC_2_PI, PI};

const TARGET_ACCURACY: f32 = 0.1;
const MAX_ANGULAR_SPEED: f32 = PI;
const MAX_SPEED: f32 = 10.;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SendFlockEvent>().add_system_set(
            SystemSet::new()
                .with_system(process_events.after(Labels::InputUpdate))
                .with_system(move_flocks),
        );
    }
}

pub struct SendFlockEvent {
    entities: Vec<Entity>,
    target: Vec2,
}

impl SendFlockEvent {
    pub fn new(entities: Vec<Entity>, target: Vec2) -> Self {
        Self { entities, target }
    }
}

#[derive(Component)]
struct Flock {
    entities: Vec<Entity>,
    target: Vec2,
}

impl Flock {
    fn from_iterable<'a, E>(entities: E, target: Vec2) -> Self
    where
        E: 'a + IntoIterator<Item = &'a Entity>,
    {
        let entities_vec: Vec<Entity> = entities.into_iter().cloned().collect();
        Self::new(entities_vec, target)
    }

    fn new(entities: Vec<Entity>, target: Vec2) -> Self {
        Self { entities, target }
    }

    fn entities(&self) -> &[Entity] {
        self.entities.as_slice()
    }

    fn target(&self) -> Vec2 {
        self.target
    }
}

#[derive(Component, Clone, Copy)]
struct ParentFlock {
    id: Entity,
}

impl ParentFlock {
    fn new(id: Entity) -> Self {
        Self { id }
    }

    fn id(&self) -> Entity {
        self.id
    }
}

#[derive(Component, Default)]
struct Movement {
    velocity: Vec3,
}

impl Movement {
    fn velocity(&self) -> Vec3 {
        self.velocity
    }
}

#[derive(Component)]
struct MovementCapabilities {
    max_power: f32,
    drag_coefficient: f32,
}

fn process_events(mut commands: Commands, mut events: EventReader<SendFlockEvent>) {
    for event in events.iter() {
        let flock = Flock::from_iterable(&event.entities, event.target);
        let flock_id = commands.spawn().insert(flock).id();
        for &entity in &event.entities {
            commands
                .entity(entity)
                .insert(ParentFlock::new(flock_id))
                .insert(Movement::default());
        }
    }
}

fn move_flocks(
    flocks: Query<&Flock>,
    entities: Query<(&GlobalTransform, &Movement), With<ParentFlock>>,
) {
    for flock in flocks.iter() {
        let mut centroid = Vec3::ZERO;
        let mut velocity = Vec3::ZERO;

        for &entity in flock.entities() {
            let (transform, movement) = entities.get(entity).unwrap();
            centroid += transform.translation;
            velocity += movement.velocity();
        }

        centroid /= flock.entities().len() as f32;
        velocity /= flock.entities().len() as f32;
    }
}

// fn move_objects(
//     mut commands: Commands,
//     mut objects: Query<(Entity, &Target, &mut Transform)>,
//     time: Res<Time>,
// ) {
//     let time_delta = time.delta().as_secs_f32();

//     for (entity, target, mut transform) in objects.iter_mut() {
//         let target_3d = Vec3::new(target.position.x, 0., target.position.y);
//         let object_to_target = target_3d - transform.translation;

//         let forward = transform.forward();
//         let angle = forward.angle_between(object_to_target);

//         if angle > f32::EPSILON {
//             let direction = if forward.cross(object_to_target).y.is_sign_negative() {
//                 -1.
//             } else {
//                 1.
//             };
//             let angle_delta = direction * (MAX_ANGULAR_SPEED * time_delta).min(angle);
//             transform.rotate(Quat::from_rotation_y(angle_delta));
//         }

//         if angle >= FRAC_2_PI {
//             continue;
//         }

//         let delta_scalar = MAX_SPEED * time_delta;
//         let delta_vec = (delta_scalar * forward).clamp_length_max(object_to_target.dot(forward));
//         transform.translation += delta_vec;

//         if (transform.translation - target_3d).length() < TARGET_ACCURACY {
//             commands.entity(entity).remove::<Target>();
//         }
//     }
// }
