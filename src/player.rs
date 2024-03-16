use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, player_movement);
    }
}

#[derive(Component)]
struct Player;

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_q: Query<&mut Transform, With<Player>>,
) {
    for mut transform in player_q.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keys.pressed(KeyCode::ArrowUp) {
            let player_forward = transform.rotation * Vec3::Z;
            direction += player_forward;
        }

        if keys.pressed(KeyCode::ArrowDown) {
            let player_backward = transform.rotation * -Vec3::Z;
            direction += player_backward;
        }

        if keys.pressed(KeyCode::ArrowLeft) {
            let player_left = transform.rotation * -Vec3::X;
            direction += player_left;
        }

        if keys.pressed(KeyCode::ArrowRight) {
            let player_right = transform.rotation * Vec3::X;
            direction += player_right;
        }

        // Normalize the direction to have a consistent movement speed
        let movement_speed = 10.0; // Adjust the speed as necessary
        let movement = direction.normalize_or_zero() * movement_speed * time.delta_seconds();

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
    commands.spawn(player);

}