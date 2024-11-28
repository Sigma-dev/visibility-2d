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
        let mut move_dir = Vec2::ZERO;
        if inputs.pressed(KeyCode::KeyD) {
            move_dir.x += 1. 
        }
        if inputs.pressed(KeyCode::KeyA) {
            move_dir.x -= 1.
        }
        if inputs.pressed(KeyCode::KeyW) {
            move_dir.y += 1.
        }
        if inputs.pressed(KeyCode::KeyS) {
            move_dir.y -= 1.
        }
        if move_dir == Vec2::ZERO {
            continue;
        }
        move_dir = move_dir.normalize();
        transform.translation += move_dir.extend(0.) * movable.speed * time.delta_seconds();
    }
}