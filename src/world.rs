use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
       app.add_systems(Startup, (spawn_light, spawn_gltf, spawn_floor));
    }
}

fn spawn_light(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 50_000_000.,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 64.0, 8.0),
        ..default()
    });
}

fn spawn_gltf(
    mut commands: Commands,
    ass: Res<AssetServer>,
) {
    commands.spawn(SceneBundle {
        scene: ass.load("scene.gltf#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(1.0,1.0,1.0)),
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