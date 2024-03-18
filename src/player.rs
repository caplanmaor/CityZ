use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

pub struct PlayerPlugin;

const ROTATION_SENSITIVITY_PLAYER: f32 = 0.001;
const ROTATION_SENSITIVITY_CAMERA: f32 = 0.001;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(PreUpdate, player_movement);
        app.add_systems(Update, camera_orbit_control);
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerCamera;

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    time: Res<Time>,
    mut player_q: Query<&mut Transform, With<Player>>,
) {
    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.read() {
        delta += event.delta;
    }

    let movement_speed = 6.0; 

    for mut transform in player_q.iter_mut() {
        // Handle rotation based on mouse movement
        transform.rotation *= Quat::from_rotation_y(-delta.x * ROTATION_SENSITIVITY_PLAYER);

        // Initialize direction vector
        let mut direction = Vec3::ZERO;

        // Adjust direction based on keyboard input
        if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
            direction -= Vec3::Z; // Moving backward
        }
        if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
            direction += Vec3::Z; // Moving forward
        }
        if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
            direction -= Vec3::X; // Moving left
        }
        if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
            direction += Vec3::X; // Moving right
        }

        // Normalize the direction to have a consistent movement speed
        let movement = if direction != Vec3::ZERO {
            transform.rotation * direction.normalize() * movement_speed * time.delta_seconds()
        } else {
            Vec3::ZERO
        };

        // Apply movement
        transform.translation += movement;
    }
}


fn spawn_player(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let player = (PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        }, 
        Player
    );
    commands.spawn(player)
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                transform: Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            })
            .insert(PlayerCamera);
        });
}

// TODO: make the camera rotate around the player
fn camera_orbit_control(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        for event in mouse_motion_events.read() {
            camera_transform.rotate(Quat::from_rotation_x(-event.delta.y * ROTATION_SENSITIVITY_CAMERA));
        }
    }
}
