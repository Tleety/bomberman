use bevy::prelude::*;
use crate::scene::Map;

pub struct MapCreationPlugin;


impl Plugin for MapCreationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    map_res: Res<Map>
){
    
    let mut x = 0.0;
    let mut y = 0.0;
    for vec in &map_res.0 {
        for value in vec {
            x += 1.0;
            if *value == 0 {
                continue;
            }
            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.95 })),
                material: materials.add(Color::rgb(240.0/256.0, 206.0/256.0, 160.0/256.0).into()),
                transform: Transform::from_xyz(x-5.5, 0.5, y-4.5),
                ..Default::default()
            });
        }
        x = 0.0;
        y += 1.0;
    }
}

