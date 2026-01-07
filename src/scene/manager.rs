use crate::math::ray::Ray;
use crate::scene::object::SceneObject3D;

pub struct Scene {
    pub objects: Vec<SceneObject3D>,
    // Special object IDs for animation/logic (keeping them for convenience)
    pub green_cube_id: usize,
    pub red_cube_id: usize,
    pub orbiting_sphere_ids: Vec<usize>,
    pub capsule_ids: Vec<usize>,
    pub statue_ids: Vec<usize>,
    pub xwing_id: usize,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            green_cube_id: 0,
            red_cube_id: 0,
            orbiting_sphere_ids: Vec::new(),
            capsule_ids: Vec::new(),
            statue_ids: Vec::new(),
            xwing_id: 0,
        }
    }

    pub fn add_object(&mut self, obj: SceneObject3D) -> usize {
        let id = obj.id;
        self.objects.push(obj);
        id
    }

    pub fn get_object_mut(&mut self, id: usize) -> Option<&mut SceneObject3D> {
        self.objects.iter_mut().find(|obj| obj.id == id)
    }

    pub fn cast_ray(&self, ray: &Ray) -> Option<usize> {
        let mut min_dist = f32::MAX;
        let mut hit_id = None;

        for obj in &self.objects {
            if let Some(dist) = obj
                .collider
                .as_ref()
                .and_then(|c| c.intersect(ray, &obj.transform))
            {
                if dist < min_dist {
                    min_dist = dist;
                    hit_id = Some(obj.id);
                }
            }
        }

        hit_id
    }

    pub fn check_intersection(&self, ray: &Ray) {
        let mut min_dist = f32::MAX;
        let mut hit_object: Option<(String, usize)> = None;

        for obj in &self.objects {
            if let Some(dist) = obj
                .collider
                .as_ref()
                .and_then(|c| c.intersect(ray, &obj.transform))
            {
                if dist < min_dist {
                    min_dist = dist;
                    hit_object = Some((obj.name.clone(), obj.id));
                }
            }
        }

        if let Some((name, id)) = hit_object {
            println!(
                "Raycast Hit: '{}' (ID: {}) at distance {:.2}",
                name, id, min_dist
            );
        } else {
            println!("Raycast Miss");
        }
    }
}
