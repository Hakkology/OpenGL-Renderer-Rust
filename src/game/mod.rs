use glam::{Mat4, Vec3};
use glfw::{Action, WindowEvent};
use std::rc::Rc;

use crate::camera::OrbitCamera;
use crate::input::Input;
use crate::light::{DirectionalLight, Light, PointLight};
use crate::primitives::{Capsule, Cube, Skybox, Sphere};
use crate::shaders::{CubeMap, Shader, Texture};
use crate::shadow::ShadowMap;
use crate::time::Time;
use crate::ui::{Button, TextRenderer};

pub trait RenderMode {
    fn update(&mut self, delta_time: f32);
    fn render(&mut self);
    fn handle_event(&mut self, event: &WindowEvent, time: &mut Time);
}

pub struct Game {
    // Primitives
    cube: Cube,
    sphere: Sphere,
    capsule: Capsule,
    skybox: Skybox,

    // Shaders
    colored_shader: Rc<Shader>,
    textured_shader: Rc<Shader>,
    ui_shader: Rc<Shader>,
    ui_rect_shader: Rc<Shader>,
    skybox_shader: Rc<Shader>,

    // Textures
    texture: Texture,
    skybox_cubemap: CubeMap,

    // UI
    text_renderer: TextRenderer,
    pause_button: Button,

    // Systems
    input: Input,
    camera: OrbitCamera,
    shadow_map: ShadowMap,

    // Lights
    light: DirectionalLight,
    point_light: PointLight,

    // State
    time: f32,
    light_space_matrix: Mat4,
}

impl Game {
    pub fn new() -> Self {
        // Load shaders
        let colored_shader = Rc::new(
            Shader::from_files("assets/shaders/lit.vert", "assets/shaders/lit_color.frag")
                .expect("Failed to create colored shader"),
        );

        let textured_shader = Rc::new(
            Shader::from_files(
                "assets/shaders/lit.vert",
                "assets/shaders/lit_textured.frag",
            )
            .expect("Failed to create textured shader"),
        );

        let ui_shader = Rc::new(
            Shader::from_files("assets/shaders/ui.vert", "assets/shaders/ui_text.frag")
                .expect("Failed to create UI text shader"),
        );

        let ui_rect_shader = Rc::new(
            Shader::from_files("assets/shaders/ui.vert", "assets/shaders/ui_color.frag")
                .expect("Failed to create UI rect shader"),
        );

        let skybox_shader = Rc::new(
            Shader::from_files("assets/shaders/skybox.vert", "assets/shaders/skybox.frag")
                .expect("Failed to create skybox shader"),
        );

        // Load textures
        let texture =
            Texture::from_file("assets/resources/textures/photo-wall-texture-pattern.jpg")
                .expect("Failed to load texture");

        let skybox_cubemap =
            CubeMap::from_cross_file("assets/resources/textures/Cubemap_Sky_22-512x512.png")
                .expect("Failed to load skybox cubemap");

        let text_renderer = TextRenderer::new(ui_shader.clone());

        // Shadow map (2048x2048 resolution)
        let shadow_map = ShadowMap::new(2048, 2048);

        let light = DirectionalLight::simple(Vec3::new(-0.2, -1.0, -0.3), 0.1, 0.5, 1.0, 32.0);

        Self {
            cube: Cube::new(1.0),
            sphere: Sphere::new(0.6, 32, 32),
            capsule: Capsule::new(0.4, 1.2, 32, 16, 16),
            skybox: Skybox::new(),
            colored_shader,
            textured_shader,
            ui_shader,
            ui_rect_shader,
            skybox_shader,
            texture,
            skybox_cubemap,
            text_renderer,
            pause_button: Button::new("Pause", 640.0, 20.0, 140.0, 40.0),
            input: Input::new(),
            camera: OrbitCamera::new(),
            shadow_map,
            light,
            point_light: PointLight::simple(Vec3::new(3.0, 3.0, 3.0), 0.05, 0.8, 1.0, 32.0),
            time: 0.0,
            light_space_matrix: Mat4::IDENTITY,
        }
    }

    fn apply_lights(&self, shader: &Shader) {
        self.light.apply_to_shader(shader, self.camera.position);
        self.point_light
            .apply_to_shader(shader, self.camera.position);
        shader.set_mat4("lightSpaceMatrix", &self.light_space_matrix.to_cols_array());
        self.shadow_map.bind_shadow_map(5);
        shader.set_int("shadowMap", 5);
    }

    fn set_mvp(&self, shader: &Shader, projection: &Mat4, view: &Mat4, model: &Mat4) {
        shader.set_mat4("projection", &projection.to_cols_array());
        shader.set_mat4("view", &view.to_cols_array());
        shader.set_mat4("model", &model.to_cols_array());
    }

    fn render_skybox(&self, projection: &Mat4) {
        self.skybox_shader.use_program();
        self.skybox_shader
            .set_mat4("projection", &projection.to_cols_array());
        self.skybox_shader
            .set_mat4("view", &self.camera.skybox_view_matrix().to_cols_array());
        self.skybox_cubemap.bind(0);
        self.skybox_shader.set_int("skybox", 0);
        self.skybox.draw();
    }

    fn get_object_models(&self) -> Vec<Mat4> {
        let mut models = Vec::new();

        // Center cube
        models.push(Mat4::IDENTITY);

        // Orbiting spheres
        for (radius, speed) in [(2.5, 1.2), (4.0, 0.8)] {
            let x = (self.time * speed).cos() * radius;
            let z = (self.time * speed).sin() * radius;
            models.push(Mat4::from_translation(Vec3::new(x, 0.0, z)));
        }

        // Green cube
        models.push(
            Mat4::from_translation(Vec3::new(0.0, 2.0, 0.0)) * Mat4::from_rotation_y(self.time),
        );

        // Red cube
        models.push(
            Mat4::from_translation(Vec3::new(0.0, -2.0, 0.0)) * Mat4::from_rotation_y(-self.time),
        );

        // Capsules
        let tilt = 45.0f32.to_radians();
        let tilt_mat = Mat4::from_rotation_z(tilt);
        for i in 0..2 {
            let offset = i as f32 * std::f32::consts::PI;
            let angle = self.time * 0.7 + offset;
            let orbit_pos = Vec3::new(angle.cos() * 4.0, 0.0, angle.sin() * 4.0);
            let tilted_pos = tilt_mat.transform_point3(orbit_pos);
            models.push(
                Mat4::from_translation(tilted_pos)
                    * Mat4::from_rotation_y(self.time)
                    * Mat4::from_rotation_x(tilt),
            );
        }

        models
    }

    fn render_shadow_pass(&self) {
        self.shadow_map.begin_pass();
        self.shadow_map
            .set_light_space_matrix(&self.light_space_matrix);

        let models = self.get_object_models();

        // Render all objects to depth
        for model in &models {
            self.shadow_map.set_model(model);
            if model == &models[0] || model == &models[1] || model == &models[2] {
                self.cube.draw();
            } else if models.len() > 3 && (model == &models[3] || model == &models[4]) {
                self.cube.draw();
            }
        }

        // Simple approach: just draw all primitives for shadow
        for model in &models[..3] {
            self.shadow_map.set_model(model);
            self.cube.draw();
        }

        self.shadow_map.end_pass(800, 600);
    }

    fn render_objects(&self, projection: &Mat4, view: &Mat4) {
        // Center textured cube
        self.textured_shader.use_program();
        self.texture.bind(0);
        self.textured_shader.set_int("u_Texture", 0);
        self.apply_lights(&self.textured_shader);
        self.set_mvp(&self.textured_shader, projection, view, &Mat4::IDENTITY);
        self.cube.draw();

        // Orbiting spheres
        for (radius, speed) in [(2.5, 1.2), (4.0, 0.8)] {
            let x = (self.time * speed).cos() * radius;
            let z = (self.time * speed).sin() * radius;
            let model = Mat4::from_translation(Vec3::new(x, 0.0, z));
            self.set_mvp(&self.textured_shader, projection, view, &model);
            self.sphere.draw();
        }

        // Colored cubes
        self.colored_shader.use_program();
        self.apply_lights(&self.colored_shader);

        // Green cube (top)
        self.colored_shader.set_vec3("objectColor", 0.5, 0.8, 0.2);
        let model =
            Mat4::from_translation(Vec3::new(0.0, 2.0, 0.0)) * Mat4::from_rotation_y(self.time);
        self.set_mvp(&self.colored_shader, projection, view, &model);
        self.cube.draw();

        // Red cube (bottom)
        self.colored_shader.set_vec3("objectColor", 1.0, 0.0, 0.0);
        let model =
            Mat4::from_translation(Vec3::new(0.0, -2.0, 0.0)) * Mat4::from_rotation_y(-self.time);
        self.set_mvp(&self.colored_shader, projection, view, &model);
        self.cube.draw();

        // Orbiting capsules (tilted 45°)
        self.textured_shader.use_program();
        self.texture.bind(0);
        self.apply_lights(&self.textured_shader);
        let tilt = 45.0f32.to_radians();
        let tilt_mat = Mat4::from_rotation_z(tilt);

        for i in 0..2 {
            let offset = i as f32 * std::f32::consts::PI;
            let angle = self.time * 0.7 + offset;
            let orbit_pos = Vec3::new(angle.cos() * 4.0, 0.0, angle.sin() * 4.0);
            let tilted_pos = tilt_mat.transform_point3(orbit_pos);
            let model = Mat4::from_translation(tilted_pos)
                * Mat4::from_rotation_y(self.time)
                * Mat4::from_rotation_x(tilt);
            self.set_mvp(&self.textured_shader, projection, view, &model);
            self.capsule.draw();
        }
    }

    fn render_ui(&self) {
        self.text_renderer.render_rect(
            &self.ui_rect_shader,
            10.0,
            540.0,
            180.0,
            50.0,
            glam::Vec4::new(0.0, 0.0, 0.0, 0.5),
            800.0,
            600.0,
        );
        self.text_renderer.render_text(
            "Hakkology",
            20.0,
            545.0,
            32.0,
            Vec3::new(1.0, 1.0, 1.0),
            800.0,
            600.0,
        );
        self.pause_button
            .draw(&self.text_renderer, &self.ui_rect_shader, 800.0, 600.0);
    }
}

impl RenderMode for Game {
    fn update(&mut self, delta_time: f32) {
        self.time += delta_time;
        self.camera.update(&self.input, delta_time);

        // Update light space matrix for shadows
        self.light_space_matrix =
            self.shadow_map
                .light_space_matrix(self.light.direction, Vec3::ZERO, 10.0);

        self.input.reset_delta();
    }

    fn render(&mut self) {
        // Shadow pass
        self.render_shadow_pass();

        let projection = self.camera.projection_matrix(800.0 / 600.0);
        let view = self.camera.view_matrix();

        // Clear after shadow pass
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        self.render_skybox(&projection);
        self.render_objects(&projection, &view);
        self.render_ui();
    }

    fn handle_event(&mut self, event: &WindowEvent, time: &mut Time) {
        self.input.handle_event(event);

        if let WindowEvent::MouseButton(glfw::MouseButtonLeft, Action::Press, _) = event {
            let (mx, my) = (self.input.mouse.pos.x, self.input.mouse.pos.y);
            if self.pause_button.is_clicked(mx, my, 600.0) {
                time.toggle_pause();
                self.pause_button.text = if time.is_paused {
                    "Resume".to_string()
                } else {
                    "Pause".to_string()
                };
            }
        }
    }
}
