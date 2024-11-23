use std::f32::consts::PI;

use bevy::{prelude::*, render::mesh::VertexAttributeValues, sprite::Mesh2dHandle};
use itertools::Itertools;

use super::{IgnoreRaycasts2d, IntersectionData2d};

#[derive(Debug)]
struct Line(Vec2, Vec2);

impl Line {
    pub fn get_middle(&self) -> Vec2 {
        (self.0 + self.1) / 2.
    }

    pub fn get_normals(&self) -> (Vec2, Vec2) {
        let dx = self.1.x - self.0.x;
        let dy = self.1.y - self.0.y;
        (Vec2::new(-dy, dx), Vec2::new(dy, -dx))
    }
    
    pub fn get_closest_normal(&self, ray: Ray2d) -> Vec2 {
        let normals = self.get_normals();
        if normals.0.angle_between(*ray.direction).abs() > PI / 2. {
            normals.0
        } else {
            normals.1
        }
    }

    pub fn length(&self) -> f32 {
        return self.0.distance(self.1)
    }
}

#[derive(Component)]
pub struct RaycastMesh2d {
    lines: Vec<Line>
}

impl RaycastMesh2d {
    pub fn from_mesh(mesh: &Mesh) -> RaycastMesh2d {
        let Some(VertexAttributeValues::Float32x3(position_data)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else { panic!("Couldn't get vertices") };
        let positions: Vec<Vec2> = position_data.iter().map(|d| Vec2::new(d[0], d[1])).collect();
        let Some(indices) = mesh.indices() else { panic!("Couldn't get vertices") };
        let mut lines = Vec::new();
        for (p1, p2, p3) in indices.iter().tuples().map(|(i1, i2, i3)| (positions[i1], positions[i2], positions[i3])) {
            lines.push(Line(p1, p2));
            lines.push(Line(p2, p3));
            lines.push(Line(p3, p1));
        };
        RaycastMesh2d { lines }
    }

    pub fn get_intersections(
        &self,
        ray: Ray2d
    ) -> Vec<IntersectionData2d> {
        let mut intersections = Vec::new();
        for line in &self.lines {
            let origin = line.get_middle();
            let normal = line.get_closest_normal(ray);
            if let Some(distance) = ray.intersect_plane(origin, Plane2d::new(normal)) {
                let position = ray.origin + *ray.direction * distance;
                if origin.distance(position) >= line.length() / 2. { continue; };
                //println!("{} {}", line.length());
                intersections.push(IntersectionData2d { position, normal, distance });
            }
        }
        intersections
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
