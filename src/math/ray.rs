use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    /// Returns the distance to the intersection point, if any.
    pub fn intersect_sphere(&self, center: Vec3, radius: f32) -> Option<f32> {
        let oc = self.origin - center;
        let a = self.direction.length_squared();
        let b = 2.0 * oc.dot(self.direction);
        let c = oc.length_squared() - radius * radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        if t > 0.0 {
            Some(t)
        } else {
            // Check if secondary intersection is valid (ray could be inside)
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            if t2 > 0.0 {
                Some(t2)
            } else {
                None
            }
        }
    }

    /// Returns the distance to the intersection point, if any.
    /// min and max are the corners of the AABB.
    pub fn intersect_aabb(&self, min: Vec3, max: Vec3) -> Option<f32> {
        let mut t_min = 0.0_f32;
        let mut t_max = f32::MAX;

        for i in 0..3 {
            let inv_d = 1.0 / self.direction[i];
            let mut t0 = (min[i] - self.origin[i]) * inv_d;
            let mut t1 = (max[i] - self.origin[i]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = t_min.max(t0);
            t_max = t_max.min(t1);

            if t_max <= t_min {
                return None;
            }
        }

        Some(t_min)
    }
}
