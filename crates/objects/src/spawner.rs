use bevy::{
    ecs::system::EntityCommands,
    prelude::{
        shape::{Box, Cube},
        *,
    },
    render::mesh::{Indices, PrimitiveTopology},
};
use de_core::{
    events::ResendEventPlugin,
    gconfig::GameConfig,
    objects::{ActiveObjectType, MovableSolid, ObjectType, Playable, StaticSolid},
    state::GameState,
};
use de_map::description::{ActiveObject, InnerObject, Object};
use iyes_loopless::prelude::*;

use crate::{cache::ObjectCache, ColliderCache};

pub(crate) struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEvent>()
            .add_plugin(ResendEventPlugin::<SpawnEvent>::default())
            .add_system(spawn.run_in_state(GameState::Playing));
    }
}

pub struct SpawnEvent {
    object: Object,
}

impl SpawnEvent {
    pub fn new(object: Object) -> Self {
        Self { object }
    }
}

fn spawn(
    mut commands: Commands,
    game_config: Res<GameConfig>,
    cache: Res<ObjectCache>,
    mut events: EventReader<SpawnEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in events.iter() {
        let child = commands.spawn().id();
        let object = &event.object;

        let transform = object.placement().to_transform();
        let global_transform = GlobalTransform::from(transform);
        let mut entity_commands = commands.spawn_bundle((global_transform, transform));

        let object_type = match object.inner() {
            InnerObject::Active(object) => {
                spawn_active(game_config.as_ref(), &mut entity_commands, object);
                ObjectType::Active(object.object_type())
            }
            InnerObject::Inactive(object) => {
                info!("Spawning inactive object {}", object.object_type());
                entity_commands.insert(StaticSolid);
                ObjectType::Inactive(object.object_type())
            }
        };

        entity_commands.add_child(child);

        entity_commands.insert(object_type).with_children(|parent| {
            parent.spawn_scene(cache.get(object_type).scene());
        });

        let mut entity_commands = commands.entity(child);
        let shape = &cache.get_collider(object_type).shape;
        let indices = Indices::U32(shape.indices().iter().flatten().cloned().collect());
        let positions: Vec<[f32; 3]> = shape.vertices().iter().map(|v| [v.x, v.y, v.z]).collect();
        let uvs: Vec<[f32; 2]> = positions.iter().map(|v| [0.5, 0.5]).collect();
        let normals: Vec<[f32; 3]> = shape
            .pseudo_normals()
            .unwrap()
            .vertices_pseudo_normal
            .iter()
            .map(|v| [v.x, v.y, v.z])
            .collect();

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(indices));

        let extents = shape.local_aabb().extents();
        let box_shape = Box::new(extents.x, extents.y, extents.z);
        let mesh = meshes.add(mesh);
        entity_commands.insert_bundle(PbrBundle {
            mesh,
            material: materials.add(
                Color::Rgba {
                    red: 0.5,
                    green: 0.5,
                    blue: 0.1,
                    alpha: 0.9,
                }
                .into(),
            ),
            ..Default::default()
        });
    }
}

fn spawn_active(game_config: &GameConfig, commands: &mut EntityCommands, object: &ActiveObject) {
    info!("Spawning active object {}", object.object_type());

    commands.insert(object.player());
    if object.player() == game_config.player() {
        commands.insert(Playable);
    }

    if object.object_type() == ActiveObjectType::Attacker {
        commands.insert(MovableSolid);
    } else {
        commands.insert(StaticSolid);
    }
}
