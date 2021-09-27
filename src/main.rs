use bevy::prelude::*;

mod map_creation_system;
mod scene;

fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(scene::ScenePlugin)
    .add_plugin(map_creation_system::MapCreationPlugin)
    .run();
}
