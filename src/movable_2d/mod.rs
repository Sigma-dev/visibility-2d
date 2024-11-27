use bevy::prelude::*;

#[derive(Component)]
pub struct Movable2d {
    speed: f32,
}

impl Movable2d {
    pub fn new(speed: f32) -> Movable2d {
        Movable2d { speed }
    }
}

pub fn plugin(app: &mut App) {
    app
    .add_systems(Update, handle_movables);
}

fn handle_movables(
    time: Res<Time>,
    inputs: Res<ButtonInput<KeyCode>>,
    mut movables_q: Query<(&mut Transform, &Movable2d)>,
) {
    for (mut transform, movable) in movables_q.iter_mut() {
        if inputs.pressed(KeyCode::KeyD) {
            transform.translation.x += movable.speed * time.delta_seconds()
        }
        if inputs.pressed(KeyCode::KeyA) {
            transform.translation.x -= movable.speed * time.delta_seconds()
        }
        if inputs.pressed(KeyCode::KeyW) {
            transform.translation.y += movable.speed * time.delta_seconds()
        }
        if inputs.pressed(KeyCode::KeyS) {
            transform.translation.y -= movable.speed * time.delta_seconds()
        }
    }
}