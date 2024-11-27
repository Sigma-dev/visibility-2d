use bevy::{prelude::*, sprite::*};
use bevy_mesh_raycast_2d::IgnoreRaycasts2d;
use bevy_view_cone::*;
use movable_2d::Movable2d;
use rotator_2d::Rotator2d;

mod movable_2d;
mod rotator_2d;
mod bevy_mesh_raycast_2d;
mod bevy_view_cone;

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, bevy_mesh_raycast_2d::plugin, bevy_view_cone::plugin, movable_2d::plugin, rotator_2d::plugin))
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
            transform: Transform::from_xyz(0., 0., 0.),
            material: materials.add(Color::WHITE),
            ..default()
        },
        IgnoreRaycasts2d,
        ViewSource::new(2000.),
        Movable2d::new(50.)
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
        ViewObstacle,
        Rotator2d::new(50.)
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Triangle2d { vertices: [Vec2::new(-50., -50.), Vec2::new(0., 50.), Vec2::new(50., -50.)] })),
            transform: Transform::from_xyz(50., 0., 0.),
            material: materials.add(Color::WHITE),
            ..default()
        },
        ViewObstacle,
        Rotator2d::new(30.)
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Capsule2d::new(15., 25.))),
            transform: Transform::from_xyz(50., -100., 0.),
            material: materials.add(Color::WHITE),
            ..default()
        },
        ViewObstacle,
        Rotator2d::new(-20.)
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 20. })),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(
                50.0,
                -10.0,
                0.0,
            ),
            ..default()
        },
        ViewObstacle,
        Rotator2d::new(10.)
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(RegularPolygon::new(40., 8))),
            transform: Transform::from_xyz(0., 200., 0.),
            material: materials.add(Color::WHITE),
            ..default()
        },
        ViewObstacle,
        Rotator2d::new(-40.)
    ));

    let size = 300.;

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle { half_size: Vec2::new(1., size) })),
            transform: Transform::from_xyz(size, 0., 0.),
            material: materials.add(Color::WHITE),
            ..default()
        },
        ViewObstacle,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle { half_size: Vec2::new(1., size) })),
            transform: Transform::from_xyz(-size, 0., 0.),
            material: materials.add(Color::WHITE),
            ..default()
        },
        ViewObstacle,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle { half_size: Vec2::new(size, 1.) })),
            transform: Transform::from_xyz(0., size, 0.),
            material: materials.add(Color::WHITE),
            ..default()
        },
        ViewObstacle,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle { half_size: Vec2::new(size, 1.) })),
            transform: Transform::from_xyz(0., -size, 0.),
            material: materials.add(Color::WHITE),
            ..default()
        },
        ViewObstacle,
    ));
}