use std::f32::consts::PI;

use bevy::{prelude::*, render::mesh::VertexAttributeValues, sprite::Mesh2dHandle};
use itertools::Itertools;

use super::{IgnoreRaycasts2d, IntersectionData2d};

#[derive(Debug)]
pub struct Line(pub Vec2, pub Vec2);

impl Line {
    pub fn transformed(&self, t: &Transform) -> Line {
        Line(t.transform_point(self.0.extend(0.)).truncate(), t.transform_point(self.1.extend(0.)).truncate())
    }

    pub fn intersection(&self, ray: Ray2d) -> Option<IntersectionData2d> {
        let origin = self.get_middle();
        let normal = self.get_closest_normal(ray);
        let Some(distance) = ray.intersect_plane(origin, Plane2d::new(normal)) else { return None };
        let position = ray.origin + *ray.direction * distance;
        let dist = origin.distance(position);
        if dist > (self.length() / 2.) + 5. { 
            return None;
        }; //TODO: Without the margin, some rays go through for some reason
        //println!("{} {}", line.length());
        Some(IntersectionData2d { position, normal, distance })
    }
}

pub trait ToLines {
    fn to_lines(&self) -> Vec<Line>;

    fn to_transformed_lines(&self, transform: &Transform) -> Vec<Line>;
}

impl ToLines for Mesh {
    fn to_lines(&self) -> Vec<Line> {
        let Some(VertexAttributeValues::Float32x3(position_data)) = self.attribute(Mesh::ATTRIBUTE_POSITION) else { panic!("Couldn't get vertices") };
        let positions: Vec<Vec2> = position_data.iter().map(|d| Vec2::new(d[0], d[1])).collect();
        let Some(indices) = self.indices() else { panic!("Couldn't get vertices") };
        let mut lines = Vec::new();
        for (p1, p2, p3) in indices.iter().tuples().map(|(i1, i2, i3)| (positions[i1], positions[i2], positions[i3])) {
            lines.push(Line(p1, p2));
            lines.push(Line(p2, p3));
            lines.push(Line(p3, p1));
        };
        lines
    }
    
    fn to_transformed_lines(&self, transform: &Transform) -> Vec<Line> {
        let lines = self.to_lines();
        lines.iter().map(|l| l.transformed(transform)).collect()
    }
}

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
        RaycastMesh2d { lines: mesh.to_lines() }
    }

    pub fn get_intersections(
        &self,
        ray: Ray2d,
        transform: &Transform
    ) -> Vec<IntersectionData2d> {
        let mut intersections = Vec::new();
        for local_line in &self.lines {
            let line = local_line.transformed(transform);
            if let Some(intersection) = line.intersection(ray) {
                intersections.push(intersection);
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
