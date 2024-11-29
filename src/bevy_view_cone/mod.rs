use std::f32::consts::PI;

use bevy::{prelude::*, render::{mesh::*, render_asset::RenderAssetUsages}, sprite::*};
use crate::bevy_mesh_raycast_2d::{raycast_mesh_2d::RaycastMesh2d, IgnoreRaycasts2d, Raycast2d};
use crate::seg_2d::Seg2d;

#[derive(Component)]
pub struct ViewObstacle;

#[derive(Component)]
pub struct ViewMesh(Entity);

impl ViewMesh {
    pub fn get(&self) -> Entity {
        self.0
    }
}

#[derive(Component)]
pub struct ViewSource {
    view_distance: f32,
}

impl ViewSource {
    pub fn new(view_distance: f32) -> ViewSource {
        ViewSource { view_distance }
    }
}

pub fn plugin(app: &mut App) {
    app
    .add_systems(Update, (add_view, draw_view));
}

fn add_view(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    sources_q: Query<Entity, Added<ViewSource>>,
) {
    for source_entity in sources_q.iter() {
        commands.spawn((
            MaterialMesh2dBundle {
                material: materials.add(Color::srgb(0.3, 0.3, 0.3)),
                transform: Transform::from_xyz(0., 0., -1.),
                ..default()
            },
            ViewMesh(source_entity),
            IgnoreRaycasts2d
        ));
    }
}

fn draw_view(
    meshes_q: Query<&RaycastMesh2d, With<ViewObstacle>>,
    sources_q: Query<(&Transform, &ViewSource)>,
    mut views_q: Query<(&ViewMesh, &mut Mesh2dHandle), Without<ViewObstacle>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut raycast_2d: Raycast2d,
) {
    let mut positions = Vec::new();
    for raycast_mesh in meshes_q.iter() {
        let lines = raycast_mesh.get_transformed_lines();
        for Seg2d(a, b) in lines {
            let a_to_b_dir = (*b - *a).normalize();
            let a_to_outwards_offset = -a_to_b_dir * 1.;
            let a_to_inwards_offset = a_to_b_dir * 0.75;
            positions.push(*a + a_to_outwards_offset);
            positions.push(*a + a_to_inwards_offset);
            positions.push(*b - a_to_outwards_offset);
            positions.push(*b - a_to_inwards_offset);
        }
    }
    for (view_mesh_source, mut view_mesh_handle) in views_q.iter_mut() {
        let (source_transform, source) = sources_q.get(view_mesh_source.get()).unwrap();
        let source_position = source_transform.translation.truncate();
        let mut ray_positions: Vec<Vec2> = positions.iter().map(|p| {
            let direction = Dir2::new(*p - source_position).unwrap();
            raycast_2d.cast_ray(Ray2d { origin: source_position, direction }).first().map_or(direction.as_vec2() * source.view_distance,|h| h.1.position)
        }).collect();

        ray_positions.sort_by(|p1, p2| angle_from_front_2d(source_transform, p1).partial_cmp(&angle_from_front_2d(source_transform, p2)).unwrap());

        let mut new = false;
        let mesh: &mut Mesh = if view_mesh_handle.is_strong() { mesh_assets.get_mut(view_mesh_handle.id()).unwrap() } else { new = true; &mut Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default()) };
    
        let mut indices = Vec::new();
        for index in 1..ray_positions.len() {
            indices.append(&mut vec![0, index as u32, index as u32 + 1]);
        }
        indices.append(&mut vec![0, ray_positions.len() as u32,  1]);
        let mut vertex_positions: Vec<Vec3> = vec![source_transform.translation];
        vertex_positions.extend(ray_positions.iter().map(|p| p.extend(0.)));
        mesh.insert_indices(Indices::U32(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertex_positions);
        if new {
            let clone = mesh.clone();
            view_mesh_handle.0 = mesh_assets.add(clone)
        }
    }
}

fn angle_from_front_2d(transform: &Transform, vec: &Vec2) -> f32 {
    let mut angle = (*vec - transform.translation.xy()).to_angle();
    if angle < 0. {
        angle = 2. * PI - angle.abs()
    }
    angle
}