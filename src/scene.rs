use bevy::prelude::*;

pub struct ScenePlugin;

pub struct Map(pub std::vec::Vec<std::vec::Vec<i32>>);

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let map = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 0, 0, 1, 1, 0, 1, 0, 0, 1],
        vec![1, 0, 1, 1, 0, 1, 1, 1, 0, 1],
        vec![1, 1, 1, 0, 1, 1, 0, 1, 1, 1],
        vec![1, 0, 1, 1, 1, 1, 1, 0, 1, 1],
        vec![1, 1, 0, 1, 1, 1, 1, 1, 0, 1],
        vec![1, 1, 1, 0, 1, 1, 0, 1, 1, 1],
        vec![1, 0, 1, 1, 1, 0, 1, 1, 0, 1],
        vec![1, 0, 0, 1, 0, 1, 1, 0, 0, 1],
        vec![1, 1, 1, 0, 1, 0, 1, 0, 1, 1]];

        app
        .insert_resource(Map(map))
        .add_startup_system(setup.system());
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
        material: materials.add(Color::rgb(83.0/256.0, 77.0/256.0, 65.0/256.0).into()),
        ..Default::default()
    });

    //light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    //camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(3.0, 15.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}