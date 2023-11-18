use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_ecs_ldtk::prelude::*;
use bevy_parallax::ParallaxPlugin;
use bevy_rapier2d::prelude::*;

pub const VIEW_WIDTH: f32 = 640.0;
pub const VIEW_HEIGHT: f32 = 360.0;
pub const MAX_GRAVITY_SPEED: f32 = 400.0;
pub const GRAVITY_ACCELERATION: f32 = 350.0;
pub const TILE_SIZE: f32 = 16.0;

use camera::CameraPlugin;
use player::PlayerPlugin;
use tiles::TilesPlugin;

mod camera;
mod player;
mod tiles;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Movement Platformer".into(),
                        resolution: (1280.0, 720.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::Index(0))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((TilesPlugin, CameraPlugin, PlayerPlugin))
        .add_systems(Startup, setup_physics)
        .run();
}

fn setup_physics(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::new(0.0, -MAX_GRAVITY_SPEED);
    commands
        .spawn(Collider::cuboid(VIEW_WIDTH, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
}
