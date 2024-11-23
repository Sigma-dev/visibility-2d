use std::f32::consts::PI;

use bevy::{prelude::*, render::{mesh::*, render_asset::RenderAssetUsages}, sprite::*};

use crate::bevy_mesh_raycast_2d::{IgnoreRaycasts2d, Raycast2d};

#[derive(Component)]
pub struct ViewObstacle;

#[derive(Component)]
pub struct ViewMesh;

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
            ViewMesh,
            IgnoreRaycasts2d
        )).id();
        commands.get_entity(source_entity).unwrap().add_child(child);
    }
}

fn draw_view(
    meshes_q: Query<(&Transform, &Mesh2dHandle), With<ViewObstacle>>,
    sources_q: Query<(&Transform, &ViewSource)>,
    mut views_q: Query<(&Parent, &mut Mesh2dHandle), (With<ViewMesh>, Without<ViewObstacle>)>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut raycast_2d: Raycast2d,
) {
    let mut positions = Vec::new();
    for (mesh_transform, mesh_handle) in meshes_q.iter() {
        let Some(mesh) = mesh_assets.get(mesh_handle.id()) else { continue; };
        let Some(VertexAttributeValues::Float32x3(position_data)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else {return};
        let mut mesh_positions: Vec<Vec2> = position_data.iter().map(|d| Vec2::new(d[0], d[1]) + mesh_transform.translation.truncate()).collect();
        positions.append(&mut mesh_positions);
    }
    for (view_mesh_parent, mut view_mesh_handle) in views_q.iter_mut() {
        let (source_transform, source) = sources_q.get(view_mesh_parent.get()).unwrap();
        let source_position = source_transform.translation.truncate();
        positions.sort_by(|p1, p2| angle_from_front_2d(source_transform, p1).partial_cmp(&angle_from_front_2d(source_transform, p2)).unwrap());
        let ray_targets: Vec<Vec2> = positions.iter().flat_map(|p| {
            let perpendicular = Vec2::new(-p.y, p.x).normalize() * 0.001;
            vec![*p - perpendicular, *p, *p + perpendicular].into_iter()
        }).collect();
        let ray_positions: Vec<Vec2> = ray_targets.iter().map(|p| {
            let direction = Dir2::new(*p - source_position).unwrap();
            raycast_2d.cast_ray(Ray2d { origin: source_position, direction }).first().map_or(direction.as_vec2() * source.view_distance,|h| h.1.position)
        }).collect();

        let mut new = false;
        let mesh: &mut Mesh = if view_mesh_handle.is_strong() { mesh_assets.get_mut(view_mesh_handle.id()).unwrap() } else { new = true; &mut Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default()) };

        let mut indices = Vec::new();
        for index in 1..ray_positions.len() + 1 {
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
    let forward = transform.local_x().truncate();
    let mut angle = forward.angle_between(*vec);
    if angle < 0. {
        angle = 2. * PI - -angle
    }
    angle
}