use bevy::{prelude::*, sprite::*};
use bevy_mesh_raycast_2d::IgnoreRaycasts2d;
use bevy_view_cone::*;

mod bevy_mesh_raycast_2d;
mod bevy_view_cone;

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, bevy_mesh_raycast_2d::plugin, bevy_view_cone::plugin))
    .add_systems(Startup, setup)
    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(
        Camera2dBundle {
            ..default()
        }
    );

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 10. })),
            material: materials.add(Color::WHITE),
            ..default()
        },
        IgnoreRaycasts2d,
        ViewSource::new(500.),
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle { half_size: Vec2::splat(50.) })),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(
                -200.0,
                0.0,
                0.0,
            ),
            ..default()
        },
        ViewObstacle
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle { half_size: Vec2::splat(50.) })),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(
                200.0,
                100.0,
                0.0,
            ),
            ..default()
        },
        ViewObstacle
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 20. })),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(
                200.0,
                -100.0,
                0.0,
            ),
            ..default()
        },
        ViewObstacle
    ));

     commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle { half_size: Vec2::splat(10000.) })),
            material: materials.add(Color::srgba(0., 0., 0., 0.)),
            ..default()
        },
        ViewObstacle,
    )); 

}