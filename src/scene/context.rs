use crate::light::{DirectionalLight, Light, PointLight};
use crate::shaders::Shader;
use crate::shadow::ShadowMap;
use glam::{Mat4, Vec3};

pub struct RenderContext<'a> {
    pub projection: Mat4,
    pub view: Mat4,
    pub view_pos: Vec3,
    pub light: &'a DirectionalLight,
    pub point_lights: &'a [PointLight],
    pub shadow_map: &'a ShadowMap,
    pub point_shadow_maps: &'a [crate::shadow::PointShadowMap],
    pub far_plane: f32,
    pub light_space_matrix: Mat4,
}

impl<'a> RenderContext<'a> {
    pub fn apply_lighting(&self, shader: &Shader) {
        self.light.apply_to_shader(shader, self.view_pos);
        for (i, pl) in self.point_lights.iter().enumerate() {
            if i >= 4 {
                break;
            }
            pl.apply_to_shader_indexed(shader, i, self.view_pos);
        }

        // We also need to set lightSpaceMatrix and bind shadow map
        shader.set_mat4("lightSpaceMatrix", &self.light_space_matrix.to_cols_array());

        // Assuming shadow map is always bound to unit 5
        self.shadow_map.bind_shadow_map(5);
        shader.set_int("shadowMap", 5);

        for (i, psm) in self.point_shadow_maps.iter().enumerate() {
            if i >= 4 {
                break;
            }
            psm.bind_cubemap(6 + i as u32);
            shader.set_int(&format!("pointShadowMaps[{}]", i), (6 + i) as i32);
        }
        shader.set_float("farPlane", self.far_plane);
        shader.set_vec3("viewPos", self.view_pos.x, self.view_pos.y, self.view_pos.z);
    }
}
