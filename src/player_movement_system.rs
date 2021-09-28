use bevy::prelude::*;
use crate::player::Player;

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(move_player.system());
    }
}

fn move_player(mut query: Query<&mut Transform, With<Player>>){
    for mut transform in query.iter_mut() {
        transform.translation.x += 1.0;
    }
}