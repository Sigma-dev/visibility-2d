use std::f32::consts::PI;

use bevy::{prelude::*, render::mesh::VertexAttributeValues, sprite::Mesh2dHandle, utils::HashSet};
use itertools::Itertools;

use super::{IgnoreRaycasts2d, IntersectionData2d};

#[derive(Debug)]
pub struct Line(pub Vec2, pub Vec2);

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        // Compare lines considering that start and end points can be swapped
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

impl Eq for Line {}

impl Line {
    fn canonical(&self) -> (Vec2, Vec2) {
        if self.0.length_squared() < self.1.length_squared() {
            (self.0.clone(), self.1.clone())
        } else {
            (self.1.clone(), self.0.clone())
        }
    }

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

    pub fn transformed(&self, t: &Transform) -> Line {
        Line(t.transform_point(self.0.extend(0.)).truncate(), t.transform_point(self.1.extend(0.)).truncate())
    }

    /*
    pub fn intersection(&self, ray: Ray2d) -> Option<IntersectionData2d> {
        let epsilon = 1e-6; // Small tolerance for floating-point imprecision

        let a = self.0; // Segment start
        let b = self.1; // Segment end
        let dir_seg = b - a; // Direction of the segment
        let dir_ray = *ray.direction; // Direction of the ray (assumed normalized)

        let denominator = dir_seg.perp_dot(dir_ray); // Perpendicular dot product
        if denominator.abs() < f32::EPSILON {
            // Parallel lines (no intersection)
            return None;
        }

        let t = (ray.origin - a).perp_dot(dir_ray) / denominator;
        let u = (ray.origin - a).perp_dot(dir_seg) / denominator;

        if t >= -epsilon && t <= 1.0 + epsilon && u >= -epsilon {
            // Adjust bounds with epsilon to handle edge cases
            let position = a + t * dir_seg; // Point of intersection

            // Additional check for endpoint alignment
            if t < 0.0 {
                if (position - a).length_squared() > epsilon * epsilon {
                    return None; // Too far from segment start
                }
            } else if t > 1.0 {
                if (position - b).length_squared() > epsilon * epsilon {
                    return None; // Too far from segment end
                }
            }

            Some(IntersectionData2d {
                position,
                normal: dir_seg.perp().normalize(), // Perpendicular to the segment
                distance: ray.origin.distance(position),
            })
        } else {
            None
        }
    } */

    pub fn intersection(&self, ray: Ray2d) -> Option<IntersectionData2d> {
        let origin = self.get_middle();
        let normal = self.get_closest_normal(ray);
        let Some(distance) = ray.intersect_plane(origin, Plane2d::new(normal)) else { return None };
        let position = ray.origin + *ray.direction * distance;
        let dist = origin.distance(position);
        if dist > (self.length() / 2.) { 
            return None;
        }; //TODO: Without the margin, some rays go through for some reason
        //println!("{} {}", line.length());
        Some(IntersectionData2d { position, normal, distance })
    }
}

use std::hash::{Hash, Hasher};

impl Hash for Line {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Ensure consistent hashing regardless of point order
        let (p1, p2) = if self.0.x < self.1.x || (self.0.x == self.1.x && self.0.y < self.1.y) {
            (&self.0, &self.1)
        } else {
            (&self.1, &self.0)
        };
        p1.x.to_bits().hash(state);
        p1.y.to_bits().hash(state);
        p2.x.to_bits().hash(state);
        p2.y.to_bits().hash(state);
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
        remove_duplicates(lines)
    }
    
    fn to_transformed_lines(&self, transform: &Transform) -> Vec<Line> {
        let lines = self.to_lines();
        lines.iter().map(|l| l.transformed(transform)).collect()
    }
}

#[derive(Component)]
pub struct RaycastMesh2d {
    lines: Vec<Line>,
    transformed_lines: Vec<Line>,
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
    ) -> &Vec<Line> {
        &self.transformed_lines
    }
}

fn remove_duplicates(lines: Vec<Line>) -> Vec<Line> {
    let len = lines.len();
    let set: HashSet<Line> = lines.into_iter().collect();
    println!("{} {}", len, set.len());

    set.into_iter().collect()
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