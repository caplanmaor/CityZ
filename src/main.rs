use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).add_systems(Startup, (spawn_floor, spawn_camera, spawn_light, spawn_gltf)).run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        // looking at the floor straight down
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });
}

fn spawn_light(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });
}

fn spawn_gltf(
    mut commands: Commands,
    ass: Res<AssetServer>,
) {
    commands.spawn(SceneBundle {
        scene: ass.load("scene.gltf#Scene0"),
        transform: Transform::from_xyz(0.0, 1.0, 0.0).with_scale(Vec3::new(5.0,5.0,5.0)),
        ..Default::default()
    });
}

fn spawn_floor(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Plane3d::default().mesh().size(50.0, 50.0))),
        // type annotation is needed here
        material: materials.add(Color::SILVER),
        ..default()
    });
}