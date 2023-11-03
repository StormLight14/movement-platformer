use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_parallax::ParallaxPlugin;
use bevy_rapier2d::prelude::*;

pub const VIEW_WIDTH: f32 = 640.0;
pub const VIEW_HEIGHT: f32 = 360.0;
pub const GRAVITY_SPEED: f32 = 400.0;

use camera::CameraPlugin;
use player::PlayerPlugin;

mod camera;
mod player;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Kirby Platformer".into(),
                        resolution: (1280.0, 720.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((CameraPlugin, PlayerPlugin))
        .add_systems(Startup, setup_physics)
        .run();
}

fn setup_physics(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::new(0.0, -GRAVITY_SPEED);
    commands
        .spawn(Collider::cuboid(VIEW_WIDTH, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
}
