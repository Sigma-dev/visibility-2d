use std::f32::consts::PI;

use bevy::{color::palettes::css, math::VectorSpace, prelude::*, render::{mesh::*, render_asset::RenderAssetUsages}, sprite::*};
use itertools::Itertools;

use crate::bevy_mesh_raycast_2d::{raycast_mesh_2d::{Line, ToLines}, IgnoreRaycasts2d, Raycast2d};

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
        let child = commands.spawn((
            MaterialMesh2dBundle {
                material: materials.add(Color::srgb(0.3, 0.3, 0.3)),
                transform: Transform::from_xyz(0., 0., -1.),
                ..default()
            },
            ViewMesh(source_entity),
            IgnoreRaycasts2d
        )).id();
    }
}

fn draw_view(
    meshes_q: Query<(&Transform, &Mesh2dHandle), With<ViewObstacle>>,
    sources_q: Query<(&Transform, &ViewSource)>,
    mut views_q: Query<(&ViewMesh, &mut Mesh2dHandle), Without<ViewObstacle>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut raycast_2d: Raycast2d,
    mut gizmos: Gizmos,
) {
    let mut positions = Vec::new();
    for (mesh_transform, mesh_handle) in meshes_q.iter() {
        let Some(mesh) = mesh_assets.get(mesh_handle.id()) else { continue; };
        let lines = mesh.to_transformed_lines(mesh_transform);
        //let Some(VertexAttributeValues::Float32x3(position_data)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else {return};
        for Line(a, b) in lines {
            let a_to_b_dir = (b - a).normalize();
            let offset = 5.; //TODO: Can probably be lower once raycast margin is fixed
            let a_to_b_offset = a_to_b_dir * offset;
            let a_to_inwards_offset = a_to_b_dir * 0.1;
            positions.push(a + a_to_inwards_offset);
            positions.push(a - a_to_b_offset);
            positions.push(b - a_to_inwards_offset);
            positions.push(b + a_to_b_offset);
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
        let red = Color::srgb(1., 0., 0.);
        for pos in &positions {
            gizmos.circle_2d(*pos, 3., red);
           // println!("{}", angle_from_front_2d(source_transform, pos));
        }

        let mut hue = 0.0;
        //gizmos.line_2d(source_position, source_transform.local_x().truncate() * 100. , Color::WHITE);
        for (p1, p2) in ray_positions.iter().tuple_windows() {
            let color = Color::hsl(hue, 1., 0.5);
            //gizmos.circle_2d(*p1, 3., red);
            //gizmos.line_2d(source_position, *p1, color);
            //gizmos.line_2d(*p1, *p2, color);
            //gizmos.line_2d(*p2, source_position, red);
            hue += 10.;
        }

        let mut new = false;
        let mesh: &mut Mesh = if view_mesh_handle.is_strong() { mesh_assets.get_mut(view_mesh_handle.id()).unwrap() } else { new = true; &mut Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default()) };
        mesh.remove_attribute(Mesh::ATTRIBUTE_POSITION);
        mesh.remove_indices();
    
        let mut indices = Vec::new();
        for index in 1..ray_positions.len() {
            indices.append(&mut vec![0, index as u32, index as u32 + 1]);
        }
        indices.append(&mut vec![0, ray_positions.len() as u32,  1]);
        let mut vertex_positions: Vec<Vec3> = vec![source_transform.translation];
        vertex_positions.extend(ray_positions.iter().map(|p| p.extend(0.)));
        //println!("{:?}", vertex_positions);
        //println!("{:?}", indices);
        mesh.insert_indices(Indices::U32(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertex_positions);
        if new {
            let clone = mesh.clone();
            view_mesh_handle.0 = mesh_assets.add(clone)
        }
    }
}

fn angle_from_front_2d(transform: &Transform, vec: &Vec2) -> f32 {
    -(vec.xy() - transform.translation.xy()).to_angle()
}