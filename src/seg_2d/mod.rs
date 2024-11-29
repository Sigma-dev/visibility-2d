use bevy::prelude::*;

#[derive(Debug)]
pub struct Seg2d(pub Vec2, pub Vec2);

impl PartialEq for Seg2d {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

impl Eq for Seg2d {}

impl Seg2d {
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
        self.0.distance(self.1)
    }

    pub fn transformed(&self, t: &Transform) -> Seg2d {
        Seg2d(t.transform_point(self.0.extend(0.)).truncate(), t.transform_point(self.1.extend(0.)).truncate())
    }
}

use std::{f32::consts::PI, hash::{Hash, Hasher}};

impl Hash for Seg2d {
    fn hash<H: Hasher>(&self, state: &mut H) {
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