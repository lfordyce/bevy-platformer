mod state;
mod debug;
mod system;
mod components;

use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_ecs_ldtk::prelude::*;

use heron::*;
use crate::state::loading::GameAssets;

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .insert_resource(ClearColor(Color::rgb(0.133, 0.122, 0.192)))
        .insert_resource(WindowDescriptor {
            title: "Platformer!".to_string(),
            ..Default::default()
        })
        .add_state(state::State::Loading)
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior:
            LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color:
            SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .insert_resource(LevelSelection::Index(0))
        .insert_resource(Gravity::from(Vec2::new(0.0, -600.0))) // Define the gravity
        .add_loading_state(
            LoadingState::new(state::State::Loading)
                .continue_to_state(state::State::InGame)
                .with_collection::<GameAssets>(),
        )
        .add_system_set(SystemSet::on_enter(state::State::InGame).with_system(system::setup))
        .add_system_set(
            SystemSet::on_update(state::State::InGame)
                .with_system(system::spawn_wall_collision)
                .with_system(system::pause_physics_during_load)
                .with_system(system::camera_fit_inside_current_level)
                .with_system(system::spawn_ground_sensor)
                .with_system(system::ground_detection)
                .with_system(system::player_added)
                .with_system(log_collisions)
        )
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .register_ldtk_entity::<components::PlayerBundle>("Player")
        .run();
}

fn log_collisions(mut events: EventReader<CollisionEvent>) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(d1, d2) => {
                println!("Collision started between {:?} and {:?}", d1, d2)
            }
            CollisionEvent::Stopped(d1, d2) => {
                println!("Collision stopped between {:?} and {:?}", d1, d2)
            }
        }
    }
}
