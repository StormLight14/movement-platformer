use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_rapier2d::prelude::*;

pub const ACCELERATION: f32 = 350.0;
pub const MAX_SPEED: f32 = 200.0;
const JUMP_STRENGTH: f32 = 150.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_texture = asset_server.load("player.png");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                anchor: Anchor::Center,
                ..default()
            },
            texture: player_texture,
            transform: Transform {
                translation: Vec3::new(100.0, 100.0, 10.0),
                rotation: Quat::from_rotation_z(0f32),
                ..default()
            },
            ..default()
        },
        Player,
        Name::from("Player"),
        Velocity::zero(),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Friction::new(0.1),
        Collider::cuboid(8.0, 8.0),
    ));
}

fn player_movement(
    mut query: Query<(&mut Velocity, &Friction, &mut Transform), With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut vel, friction, mut transform) in query.iter_mut() {
        let left_input = input.any_pressed([KeyCode::A, KeyCode::Left]);
        let right_input = input.any_pressed([KeyCode::D, KeyCode::Right]);

        if input.just_pressed(KeyCode::Space) {
            vel.linvel.y = JUMP_STRENGTH;
        }

        let x_axis = -(left_input as i8) + right_input as i8;

        // move player
        if x_axis != 0 {
            if vel.linvel.x + ACCELERATION * time.delta_seconds() < MAX_SPEED && x_axis > 0 {
                vel.linvel.x += ACCELERATION * time.delta_seconds();
            } else if vel.linvel.x - ACCELERATION * time.delta_seconds() > -MAX_SPEED && x_axis < 0
            {
                vel.linvel.x -= ACCELERATION * time.delta_seconds();
            } else {
                vel.linvel.x = x_axis as f32 * MAX_SPEED;
            }
        } else {
            if vel.linvel.x > 0.0 + friction.coefficient {
                vel.linvel.x -= friction.coefficient;
            } else if vel.linvel.x < 0.0 - friction.coefficient {
                vel.linvel.x += friction.coefficient;
            } else {
                vel.linvel.x = 0.0;
            }
        }
    }
}
