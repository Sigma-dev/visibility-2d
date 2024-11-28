use bevy::{ecs::system::{lifetimeless::Read, SystemParam}, prelude::*};
use raycast_mesh_2d::{build_raycastable_meshes, RaycastMesh2d};

pub mod raycast_mesh_2d;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, build_raycastable_meshes);
}

#[derive(Component)]
pub struct IgnoreRaycasts2d;

#[derive(Debug, Clone, Reflect)]
pub struct IntersectionData2d {
    pub position: Vec2,
    pub normal: Vec2,
    pub distance: f32,
}

impl IntersectionData2d {
    pub fn with_position(&self, position: Vec2) -> IntersectionData2d {
        IntersectionData2d { position: position, normal: self.normal, distance: self.distance }
    }
}

#[derive(SystemParam)]
pub struct Raycast2d<'w, 's> {
    #[doc(hidden)]
    pub mesh_query: Query<
        'w,
        's,
        (
            Entity,
            Read<RaycastMesh2d>,
            Read<GlobalTransform>,
        ),
    >,
}

impl<'w, 's> Raycast2d<'w, 's> {
    pub fn cast_ray(
        &mut self,
        ray: Ray2d,
    ) -> Vec<(Entity, IntersectionData2d)> {
        let mut hits = Vec::new();
        for (entity, mesh, gt) in self.mesh_query.iter() {
            //let mut local_ray = ray.clone();
            //local_ray.origin -= gt.translation().truncate();
            hits.extend(mesh.get_intersections(ray, &gt.compute_transform()).iter().map(|i| (entity, i.with_position(i.position))));
        }
        hits.sort_by(|h1, h2| h1.1.distance.partial_cmp(&h2.1.distance).unwrap());
        hits
    }
}