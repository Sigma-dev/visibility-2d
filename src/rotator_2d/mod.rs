use bevy::prelude::*;

#[derive(Component)]
pub struct Rotator2d {
    speed: f32,
}

impl Rotator2d {
    pub fn new(speed: f32) -> Rotator2d {
        Rotator2d { speed }
    }
}

pub fn plugin(app: &mut App) {
    app
    .add_systems(Update, handle_rotators);
}

fn handle_rotators(
    time: Res<Time>,
    mut rotators_q: Query<(&mut Transform, &Rotator2d)>,
) {
    for (mut transform, rotator) in rotators_q.iter_mut() {
        transform.rotate_z(rotator.speed.to_radians() * time.delta_seconds() * 1.); //rotation doesn't work because the lines arn't yet transformed maybe ?
    }
}