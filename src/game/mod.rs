use std::rc::Rc;
use glam::{Mat4, Vec3};
use glfw::{Action, Key, WindowEvent};

use crate::primitives::{Cube, Sphere, Capsule, Skybox};
use crate::shaders::{Shader, VertexShaderKind, FragmentShaderKind, Texture, CubeMap};
use crate::light::DirectionalLight;
use crate::ui::{TextRenderer, Button};
use crate::input::Input;

use crate::time::Time;

pub trait RenderMode {
    fn update(&mut self, delta_time: f32);
    fn render(&self);
    fn handle_event(&mut self, event: &WindowEvent, time: &mut Time);
}

pub struct Game {
    // Assets & Primitives
    cube: Cube,
    sphere: Sphere,
    capsule: Capsule,
    colored_shader: Rc<Shader>,
    textured_shader: Rc<Shader>,
    ui_shader: Rc<Shader>,
    ui_rect_shader: Rc<Shader>,
    texture: Texture,
    text_renderer: TextRenderer,
    pause_button: Button,
    skybox: Skybox,
    skybox_shader: Rc<Shader>,
    skybox_cubemap: CubeMap,

    // State
    input: Input,
    light: DirectionalLight,
    camera_pos: Vec3,
    yaw: f32,
    pitch: f32,
    camera_distance: f32,
    time: f32,
}

impl Game {
    pub fn new() -> Self {
        let colored_shader = Rc::new(Shader::new(
            VertexShaderKind::Lit, 
            FragmentShaderKind::LitDirectional
        ).expect("Failed to create colored shader"));

        let textured_shader = Rc::new(Shader::new(
            VertexShaderKind::Lit,
            FragmentShaderKind::LitTextured
        ).expect("Failed to create textured shader"));

        let ui_shader = Rc::new(Shader::new(
            VertexShaderKind::UI,
            FragmentShaderKind::UIText
        ).expect("Failed to create UI text shader"));

        let ui_rect_shader = Rc::new(Shader::new(
            VertexShaderKind::UI,
            FragmentShaderKind::UIColor
        ).expect("Failed to create UI rect shader"));

        let texture = Texture::from_file("assets/resources/textures/photo-wall-texture-pattern.jpg")
            .expect("Failed to load texture");

        let text_renderer = TextRenderer::new(ui_shader.clone());

        let skybox_shader = Rc::new(Shader::new(
            VertexShaderKind::Skybox,
            FragmentShaderKind::Skybox
        ).expect("Failed to create skybox shader"));

        let skybox_cubemap = CubeMap::from_cross_file("assets/resources/textures/Cubemap_Sky_22-512x512.png")
            .expect("Failed to load skybox cubemap");

        Self {
            cube: Cube::new(1.0),
            sphere: Sphere::new(0.6, 32, 32),
            capsule: Capsule::new(0.4, 1.2, 32, 16, 16),
            colored_shader,
            textured_shader,
            ui_shader,
            ui_rect_shader,
            texture,
            text_renderer,
            pause_button: Button::new("Pause", 640.0, 20.0, 140.0, 40.0),
            skybox: Skybox::new(),
            skybox_shader,
            skybox_cubemap,
            input: Input::new(),
            light: DirectionalLight::new(
                Vec3::new(-0.2, -1.0, -0.3),
                0.1,    // ambient
                0.5,    // diffuse
                1.0,    // specular
                32.0    // shininess
            ),
            camera_pos: Vec3::new(0.0, 0.0, 10.0),
            yaw: -90.0,
            pitch: 0.0,
            camera_distance: 7.0,
            time: 0.0,
        }
    }

    fn set_light_uniforms(&self, shader: &Shader) {
        let light = &self.light;
        shader.set_vec3("lightDir", light.direction.x, light.direction.y, light.direction.z);
        shader.set_vec3("lightColor", light.color.x, light.color.y, light.color.z);
        shader.set_float("ambientStrength", light.ambient);
        shader.set_float("diffuseStrength", light.diffuse);
        shader.set_float("specularStrength", light.specular);
        shader.set_float("shininess", light.shininess);
        shader.set_vec3("viewPos", self.camera_pos.x, self.camera_pos.y, self.camera_pos.z);
    }

    fn shader_set_mvp(&self, shader: &Shader, p: &Mat4, v: &Mat4, m: &Mat4) {
        shader.set_mat4("projection", &p.to_cols_array());
        shader.set_mat4("view", &v.to_cols_array());
        shader.set_mat4("model", &m.to_cols_array());
    }
}

impl RenderMode for Game {
    fn update(&mut self, delta_time: f32) {
        self.time += delta_time;

        if self.input.is_mouse_button_pressed(glfw::MouseButtonLeft) {
            let sensitivity = 50.0;
            self.yaw += self.input.mouse.delta.x * sensitivity * delta_time;
            self.pitch -= self.input.mouse.delta.y * sensitivity * delta_time; 
            
            if self.pitch > 89.0 { self.pitch = 89.0; }
            if self.pitch < -89.0 { self.pitch = -89.0; }
        }

        // Zoom update
        self.camera_distance -= self.input.mouse.scroll_y * 0.5;
        if self.camera_distance < 2.0 { self.camera_distance = 2.0; }
        if self.camera_distance > 20.0 { self.camera_distance = 20.0; }

        let radius = self.camera_distance;
        self.camera_pos.x = radius * self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        self.camera_pos.y = radius * self.pitch.to_radians().sin();
        self.camera_pos.z = radius * self.yaw.to_radians().sin() * self.pitch.to_radians().cos();
        
        self.input.reset_delta();
    }

    fn render(&self) {
        let projection = Mat4::perspective_rh_gl(45.0f32.to_radians(), 800.0/600.0, 0.1, 100.0);
        let view = Mat4::look_at_rh(self.camera_pos, Vec3::ZERO, Vec3::Y);

        // Render Skybox first
        self.skybox_shader.use_program();
        // Remove translation from view matrix for skybox
        let view_no_translation = Mat4::from_mat3(glam::Mat3::from_mat4(view));
        self.skybox_shader.set_mat4("projection", &projection.to_cols_array());
        self.skybox_shader.set_mat4("view", &view_no_translation.to_cols_array());
        self.skybox_cubemap.bind(0);
        self.skybox_shader.set_int("skybox", 0);
        self.skybox.draw();

        // 1. Render Textured Cube (CENTER)
        self.textured_shader.use_program();
        self.texture.bind(0);
        self.textured_shader.set_int("u_Texture", 0);
        self.set_light_uniforms(&self.textured_shader);

        let model_tex_cube = Mat4::IDENTITY;
        self.shader_set_mvp(&self.textured_shader, &projection, &view, &model_tex_cube);
        self.cube.draw();

        // 2. Render Orbiting Spheres
        self.textured_shader.use_program();
        self.texture.bind(0);
        
        // Inner Sphere (Distance 2.5)
        let orbit_radius1 = 2.5;
        let speed1 = 1.2;
        let sphere_x1 = (self.time * speed1).cos() * orbit_radius1;
        let sphere_z1 = (self.time * speed1).sin() * orbit_radius1;
        let model_sphere1 = Mat4::from_translation(Vec3::new(sphere_x1, 0.0, sphere_z1));
        self.shader_set_mvp(&self.textured_shader, &projection, &view, &model_sphere1);
        self.sphere.draw();

        // Outer Sphere (Distance 4.0)
        let orbit_radius2 = 4.0;
        let speed2 = 0.8;
        let sphere_x2 = (self.time * speed2).cos() * orbit_radius2;
        let sphere_z2 = (self.time * speed2).sin() * orbit_radius2;
        let model_sphere2 = Mat4::from_translation(Vec3::new(sphere_x2, 0.0, sphere_z2));
        self.shader_set_mvp(&self.textured_shader, &projection, &view, &model_sphere2);
        self.sphere.draw();

        // 3. Render Green Cube (UP) (Self-rotating)
        self.colored_shader.use_program();
        self.set_light_uniforms(&self.colored_shader);
        self.colored_shader.set_vec3("objectColor", 0.5, 0.8, 0.2);
        let model_green = Mat4::from_translation(Vec3::new(0.0, 2.0, 0.0)) * Mat4::from_rotation_y(self.time);
        self.shader_set_mvp(&self.colored_shader, &projection, &view, &model_green);
        self.cube.draw();

        // 4. Render Red Cube (DOWN) (Self-rotating)
        self.colored_shader.set_vec3("objectColor", 1.0, 0.0, 0.0);
        let model_red = Mat4::from_translation(Vec3::new(0.0, -2.0, 0.0)) * Mat4::from_rotation_y(-self.time);
        self.shader_set_mvp(&self.colored_shader, &projection, &view, &model_red);
        self.cube.draw();

        // 5. Render Orbiting Capsules (Tilted 45 degrees, self-rotating)
        self.textured_shader.use_program();
        self.texture.bind(0);
        
        let orbit_radius_capsule = 4.0;
        let capsule_speed = 0.7;
        let tilt_angle = 45.0f32.to_radians();
        let tilt_mat = Mat4::from_rotation_z(tilt_angle);

        for i in 0..2 {
            let offset = i as f32 * std::f32::consts::PI; // Opposite sides
            let angle = self.time * capsule_speed + offset;
            
            // Base orbit on XZ plane
            let orbit_pos = Vec3::new(angle.cos() * orbit_radius_capsule, 0.0, angle.sin() * orbit_radius_capsule);
            // Apply 45 degree tilt
            let tilted_pos = tilt_mat.transform_point3(orbit_pos);
            
            let model_capsule = Mat4::from_translation(tilted_pos) 
                * Mat4::from_rotation_y(self.time) // Self rotation
                * Mat4::from_rotation_x(tilt_angle); // Align with orbit tilt or just extra flair
                
            self.shader_set_mvp(&self.textured_shader, &projection, &view, &model_capsule);
            self.capsule.draw();
        }

        // 5. UI
        self.text_renderer.render_rect(&self.ui_rect_shader, 10.0, 540.0, 180.0, 50.0, glam::Vec4::new(0.0, 0.0, 0.0, 0.5), 800.0, 600.0);
        self.text_renderer.render_text("Hakkology", 20.0, 545.0, 32.0, Vec3::new(1.0, 1.0, 1.0), 800.0, 600.0);

        // Draw Pause Button
        self.pause_button.draw(&self.text_renderer, &self.ui_rect_shader, 800.0, 600.0);
    }

    fn handle_event(&mut self, event: &WindowEvent, time: &mut Time) {
        self.input.handle_event(event);

        match event {
            WindowEvent::MouseButton(glfw::MouseButtonLeft, Action::Press, _) => {
                let mx = self.input.mouse.pos.x;
                let my = self.input.mouse.pos.y;
                if self.pause_button.is_clicked(mx, my, 600.0) {
                    time.toggle_pause();
                    self.pause_button.text = if time.is_paused { "Resume".to_string() } else { "Pause".to_string() };
                }
            }
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                // escape is handled globally too
            }
            _ => {}
        }
    }
}
