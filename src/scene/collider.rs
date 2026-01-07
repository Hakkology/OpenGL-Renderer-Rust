use crate::math::ray::Ray;
use crate::scene::transform::Transform;
use glam::Vec3;

#[derive(Debug, Clone)]
pub enum ColliderShape {
    Sphere { radius: f32 },
    Box { min: Vec3, max: Vec3 },
    // Add Capsule later
}

#[derive(Debug, Clone)]
pub struct Collider {
    pub shape: ColliderShape,
    pub enabled: bool,
}

impl Collider {
    pub fn new_sphere(radius: f32) -> Self {
        Self {
            shape: ColliderShape::Sphere { radius },
            enabled: true,
        }
    }

    pub fn new_box(min: Vec3, max: Vec3) -> Self {
        Self {
            shape: ColliderShape::Box { min, max },
            enabled: true,
        }
    }

    /// Helper to create a box collider for a cube of given size centered at origin.
    pub fn new_cube(size: f32) -> Self {
        let half = size / 2.0;
        Self::new_box(Vec3::splat(-half), Vec3::splat(half))
    }

    /// Check for intersection with a ray.
    /// Returns the distance along the ray (t) if intersected.
    pub fn intersect(&self, ray: &Ray, transform: &Transform) -> Option<f32> {
        if !self.enabled {
            return None;
        }

        let model_matrix = transform.to_matrix();
        let inverse_model = model_matrix.inverse();

        let local_origin = inverse_model.transform_point3(ray.origin);
        let local_direction = inverse_model.transform_vector3(ray.direction);

        let local_ray = Ray {
            origin: local_origin,
            direction: local_direction,
        };

        match self.shape {
            ColliderShape::Sphere { radius } => local_ray.intersect_sphere(Vec3::ZERO, radius),
            ColliderShape::Box { min, max } => local_ray.intersect_aabb(min, max),
        }
    }
}
