use bevy::prelude::*;

pub struct MapCreationPlugin;

impl Plugin for MapCreationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
){
    let map = vec![
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 1, 1, 0],
        vec![0, 1, 1, 1, 0],
        vec![0, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1]];
    
    let mut x = 0.0;
    let mut y = 0.0;
    for vec in map {
        for value in vec {
            x += 1.0;
            if value == 0 {
                continue;
            }
            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.1, 0.6, 0.3).into()),
                transform: Transform::from_xyz(x-3.0, 0.5, y-2.0),
                ..Default::default()
            });
        }
        x = 0.0;
        y += 1.0;
    }
}

