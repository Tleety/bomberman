use bevy::prelude::*;
use crate::player::Player;
use crate::game_tick_system::GameTickTimer;

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(move_player.system());
    }
}

fn move_player(
    mut query: Query<&mut Transform, With<Player>>,
    ticker: Res<GameTickTimer>,
    keys: Res<Input<KeyCode>>
){
    if !ticker.0.just_finished() {
        return; 
    }

    let mut direction = Vec3::ZERO;
    if keys.pressed(KeyCode::S) {
        direction.z -= 1.0;
    }
    if keys.pressed(KeyCode::W) {
        direction.z += 1.0;
    }
    if keys.pressed(KeyCode::A) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::D) {
        direction.x += 1.0;
    }

    for mut transform in query.iter_mut() {
        transform.translation += direction;
    }
}