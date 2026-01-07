use crate::config::{rendering as render_cfg, window as win_cfg};
use crate::light::{DirectionalLight, PointLight};
use crate::primitives::Skybox;
use crate::scene::context::RenderContext;
use crate::scene::manager::Scene;
use crate::shaders::{CubeMap, Shader};
use crate::shadow::{PointShadowMap, ShadowMap};
use glam::{Mat4, Vec3};
use std::rc::Rc;

pub struct Renderer {
    pub skybox: Skybox,
    pub skybox_shader: Rc<Shader>,
    pub skybox_cubemap: Rc<CubeMap>,
    pub shadow_map: ShadowMap,
    pub point_shadow_maps: Vec<PointShadowMap>,
    pub light_space_matrix: Mat4,
    pub frame_count: u64,
}

impl Renderer {
    pub fn new(skybox_shader: Rc<Shader>, skybox_cubemap: Rc<CubeMap>) -> Self {
        let shadow_map = ShadowMap::new(render_cfg::SHADOW_MAP_SIZE, render_cfg::SHADOW_MAP_SIZE);

        let mut point_shadow_maps = Vec::new();
        for _ in 0..render_cfg::MAX_POINT_LIGHTS {
            point_shadow_maps.push(PointShadowMap::new(render_cfg::POINT_SHADOW_SIZE));
        }

        Self {
            skybox: Skybox::new(),
            skybox_shader,
            skybox_cubemap,
            shadow_map,
            point_shadow_maps,
            light_space_matrix: Mat4::IDENTITY,
            frame_count: 0,
        }
    }

    pub fn render(
        &mut self,
        scene: &Scene,
        camera: &crate::camera::OrbitCamera,
        light: &DirectionalLight,
        point_lights: &[PointLight],
    ) {
        // 1. Shadow Passes
        self.render_shadow_pass(scene, light);
        self.render_point_shadow_pass(scene, point_lights);

        // 2. Main Render Setup
        let aspect = win_cfg::WIDTH as f32 / win_cfg::HEIGHT as f32;
        let projection = camera.projection_matrix(aspect);
        let view = camera.view_matrix();

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // 3. Render Skybox
        self.render_skybox(&projection, &view);

        // 4. Render Scene Objects
        let context = RenderContext {
            projection,
            view,
            view_pos: camera.position,
            light,
            point_lights,
            shadow_map: &self.shadow_map,
            point_shadow_maps: &self.point_shadow_maps,
            far_plane: render_cfg::SHADOW_FAR_PLANE,
            light_space_matrix: self.light_space_matrix,
        };

        for obj in &scene.objects {
            obj.render(&context);
        }

        self.frame_count += 1;
    }

    fn render_skybox(&self, projection: &Mat4, view: &Mat4) {
        unsafe {
            gl::DepthFunc(gl::LEQUAL);
        }
        self.skybox_shader.use_program();

        // Remove translation from view matrix for skybox
        let mut skybox_view = *view;
        skybox_view.w_axis = glam::Vec4::W;

        self.skybox_shader
            .set_mat4("projection", &projection.to_cols_array());
        self.skybox_shader
            .set_mat4("view", &skybox_view.to_cols_array());
        self.skybox_shader.set_int("skybox", 0);

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.skybox_cubemap.id);
        }
        self.skybox.draw();
        unsafe {
            gl::DepthFunc(gl::LESS);
        }
    }

    fn render_shadow_pass(&mut self, scene: &Scene, light: &DirectionalLight) {
        self.light_space_matrix =
            self.shadow_map
                .light_space_matrix(light.direction, Vec3::ZERO, 35.0);

        self.shadow_map.begin_pass();
        self.shadow_map
            .set_light_space_matrix(&self.light_space_matrix);

        for obj in &scene.objects {
            obj.render_depth(&self.shadow_map.shader);
        }
        self.shadow_map.end_pass(win_cfg::WIDTH, win_cfg::HEIGHT);
    }

    fn render_point_shadow_pass(&mut self, scene: &Scene, point_lights: &[PointLight]) {
        let far_plane = render_cfg::SHADOW_FAR_PLANE;
        let light_idx = (self.frame_count % 2) as usize;

        for i in 0..2 {
            let idx = light_idx + i * 2;
            if let Some(pl) = point_lights.get(idx) {
                if let Some(psm) = self.point_shadow_maps.get(idx) {
                    psm.begin_pass(pl.position, far_plane);
                    for obj in &scene.objects {
                        obj.render_depth(&psm.shader);
                    }
                    psm.end_pass(win_cfg::WIDTH, win_cfg::HEIGHT);
                }
            }
        }
    }
}
