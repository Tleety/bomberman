use bevy::prelude::*;

pub struct GameTickPlugin;
pub struct GameTickTimer(pub Timer);

impl Plugin for GameTickPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .insert_resource(GameTickTimer(Timer::from_seconds(0.5, true)))
        .add_system(tick_system.system());
    }
}

fn tick_system(
    mut timer: ResMut<GameTickTimer>,
    time: Res<Time>
){
    timer.0.tick(time.delta());
}