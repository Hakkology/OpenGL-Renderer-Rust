use glam::Vec3;

pub struct DirectionalLight {
    pub direction: Vec3,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub color: Vec3,
}

impl DirectionalLight {
    pub fn new(direction: Vec3, ambient: f32, diffuse: f32, specular: f32, shininess: f32) -> Self {
        DirectionalLight {
            direction: direction.normalize(),
            ambient,
            diffuse,
            specular,
            shininess,
            color: Vec3::ONE, // Varsayılan beyaz ışık
        }
    }
}
