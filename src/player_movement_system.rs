use bevy::prelude::*;

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(move_player.system());
    }
}

fn move_player(){
}