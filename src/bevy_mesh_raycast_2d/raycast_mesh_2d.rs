use bevy::{prelude::*, render::mesh::VertexAttributeValues, sprite::Mesh2dHandle, utils::HashSet};
use itertools::Itertools;

use super::{IgnoreRaycasts2d, IntersectionData2d};
use crate::seg_2d::Seg2d;

pub trait ToSeg2ds {
    fn to_lines(&self) -> Vec<Seg2d>;
}

impl ToSeg2ds for Mesh {
    fn to_lines(&self) -> Vec<Seg2d> {
        let Some(VertexAttributeValues::Float32x3(position_data)) = self.attribute(Mesh::ATTRIBUTE_POSITION) else { panic!("Couldn't get vertices") };
        let positions: Vec<Vec2> = position_data.iter().map(|d| Vec2::new(d[0], d[1])).collect();
        let Some(indices) = self.indices() else { panic!("Couldn't get vertices") };
        let mut lines = Vec::new();
        for (p1, p2, p3) in indices.iter().tuples().map(|(i1, i2, i3)| (positions[i1], positions[i2], positions[i3])) {
            lines.push(Seg2d(p1, p2));
            lines.push(Seg2d(p2, p3));
            lines.push(Seg2d(p3, p1));
        };
        remove_duplicates(lines)
    }
}

fn remove_duplicates(lines: Vec<Seg2d>) -> Vec<Seg2d> {
    let len = lines.len();
    let set: HashSet<Seg2d> = lines.into_iter().collect();
    println!("{} {}", len, set.len());

    set.into_iter().collect()
}

#[derive(Component)]
pub struct RaycastMesh2d {
    lines: Vec<Seg2d>,
    transformed_lines: Vec<Seg2d>,
}

impl RaycastMesh2d {
    pub fn from_mesh(mesh: &Mesh) -> RaycastMesh2d {
        RaycastMesh2d { lines: mesh.to_lines(), transformed_lines: Vec::new() }
    }

    pub fn get_intersections(
        &self,
        ray: Ray2d,
        transform: &Transform
    ) -> Vec<IntersectionData2d> {
        let mut intersections = Vec::new();
        for local_line in &self.lines {
            let line = local_line.transformed(transform); //TODO: Generate the transformed lines in PreUpdate every frame
            if let Some(intersection) = line.intersection(ray) {
                intersections.push(intersection);
            }
        }
        intersections
    }

    pub fn get_transformed_lines(
        &self,
    ) -> &Vec<Seg2d> {
        &self.transformed_lines
    }
}

pub fn build_raycastable_meshes(
    mut commands: Commands,
    meshes_q: Query<(Entity, &Mesh2dHandle), (Without<RaycastMesh2d>, Without<IgnoreRaycasts2d>)>,
    mesh_assets: Res<Assets<Mesh>>,
) {
    for (entity, mesh_handle) in meshes_q.iter() {
        let Some(mesh) = mesh_assets.get(mesh_handle.id()) else { continue; };
        let Some (mut e) = commands.get_entity(entity) else { continue; };
        e.insert(RaycastMesh2d::from_mesh(mesh));
    }
}

pub fn update_transformed_raycastables(
    mut meshes_q: Query<(&Transform, &mut RaycastMesh2d)>,
) {
    for (transform, mut raycast_mesh) in meshes_q.iter_mut() {
        raycast_mesh.transformed_lines = raycast_mesh.lines.iter().map(|l| l.transformed(transform)).collect();
    }
}

pub trait Intersectionable2d {
    fn intersection(&self, ray: Ray2d) -> Option<IntersectionData2d>;
}

impl Intersectionable2d for Seg2d {
    fn intersection(&self, ray: Ray2d) -> Option<IntersectionData2d> {
        let origin = self.get_middle();
        let normal = self.get_closest_normal(ray);
        let Some(distance) = ray.intersect_plane(origin, Plane2d::new(normal)) else { return None };
        let position = ray.origin + *ray.direction * distance;
        let dist = origin.distance(position);
        if dist > (self.length() / 2.) { 
            return None;
        };
        Some(IntersectionData2d { position, normal, distance })
    }
}