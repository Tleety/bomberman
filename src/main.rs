use bevy::prelude::*;

mod map_creation_system;
mod scene;
mod player;
mod player_movement_system;

fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(scene::ScenePlugin)
    .add_plugin(map_creation_system::MapCreationPlugin)
    .add_plugin(player_movement_system::PlayerMovementPlugin)
    .run();
}
