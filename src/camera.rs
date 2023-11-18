use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_parallax::*;
use bevy_rapier2d::prelude::*;

use crate::{
    player::{Player, ACCELERATION, MAX_SPEED},
    VIEW_HEIGHT, VIEW_WIDTH,
};

//const CAMERA_MAX_SPEED: f32 = MAX_SPEED;
//const CAMERA_ACCELERATION: f32 = ACCELERATION * 0.9;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ParallaxPlugin)
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follow)
            .add_systems(Update, move_background);
    }
}

fn spawn_camera(mut commands: Commands, mut create_parallax: EventWriter<CreateParallaxEvent>) {
    let mut camera = Camera2dBundle::default();
    camera.camera_2d.clear_color = ClearColorConfig::Custom(Color::BLACK);

    // camera has 0, 0 in bottom left
    camera.transform.translation += Vec3::new(VIEW_WIDTH / 2.0, VIEW_HEIGHT / 2.0, 0.0);

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: VIEW_WIDTH,
        min_height: VIEW_HEIGHT,
    };

    let spawn_camera = commands
        .spawn((camera, Velocity::zero(), ParallaxCameraComponent::default()))
        .id();

    create_parallax.send(CreateParallaxEvent {
        layers_data: vec![
            /*LayerData {
                speed: LayerSpeed::Bidirectional(0.9, 0.9),
                repeat: LayerRepeat::horizontally(RepeatStrategy::Same),
                path: "bg/cyberpunk_back.png".to_string(),
                tile_size: Vec2::new(96.0, 160.0),
                cols: 1,
                rows: 1,
                scale: 1.0,
                z: 0.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Bidirectional(0.6, 0.8),
                repeat: LayerRepeat::horizontally(RepeatStrategy::Same),
                path: "bg/cyberpunk_middle.png".to_string(),
                tile_size: Vec2::new(144.0, 160.0),
                cols: 1,
                rows: 1,
                scale: 1.0,
                z: 1.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Bidirectional(0.1, 0.3),
                repeat: LayerRepeat::both(RepeatStrategy::Mirror),
                path: "bg/cyberpunk_front.png".to_string(),
                tile_size: Vec2::new(272.0, 160.0),
                cols: 1,
                rows: 1,
                scale: 1.0,
                z: 2.0,
                ..default()
            },
            */
            // Mountains
            LayerData {
                speed: LayerSpeed::Bidirectional(0.9, 0.9),
                repeat: LayerRepeat::horizontally(RepeatStrategy::Same),
                path: "bg/mountains1.png".to_string(),
                tile_size: Vec2::new(2000.0, 1000.0),
                cols: 1,
                rows: 1,
                scale: 1.0,
                z: 0.0,
                ..default()
            },
            LayerData {
                position: Vec2::new(0.0, 200.0),
                speed: LayerSpeed::Bidirectional(0.2, 0.2),
                repeat: LayerRepeat::horizontally(RepeatStrategy::Same),
                path: "bg/clouds1.png".to_string(),
                tile_size: Vec2::new(1500.0, 50.0),
                cols: 5,
                rows: 1,
                scale: 1.0,
                z: 1.0,
                ..default()
            },
        ],
        camera: spawn_camera,
    });
}

fn camera_follow(
    mut camera_query: Query<(&mut Transform, &mut Velocity), With<Camera2d>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let (mut camera_transform, _camera_velocity) = camera_query.single_mut();
    let player_transform = player_query.single();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

pub fn move_background(
    keyboard_input: Res<Input<KeyCode>>,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
    mut camera_query: Query<(Entity, &mut Transform), With<Camera>>,
    player_query: Query<&Velocity, (With<Player>, Without<Camera>)>,
    time: Res<Time>,
) {
    let (camera, mut camera_transform) = camera_query.get_single_mut().unwrap();
    let player_velocity = player_query.single();

    let mut direction = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        direction += Vec2::new(1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        direction += Vec2::new(-1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        direction += Vec2::new(0.0, 1.0);
    }
    if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        direction += Vec2::new(0.0, -1.0);
    }
    if keyboard_input.pressed(KeyCode::E) {
        camera_transform.rotate_z(0.1);
    }
    if keyboard_input.pressed(KeyCode::Q) {
        camera_transform.rotate_z(-0.1);
    }

    move_event_writer.send(ParallaxMoveEvent {
        camera_move_speed: Vec2::new(player_velocity.linvel.x * time.delta_seconds(), 0.0),
        camera,
    });
}
