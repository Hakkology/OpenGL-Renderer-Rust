use glam::{Mat4, Vec3, Quat};
use glfw::{Action, WindowEvent};
use std::rc::Rc;

use crate::camera::OrbitCamera;
use crate::input::Input;
use crate::light::{DirectionalLight, Light, PointLight};
use crate::primitives::{Capsule, Cube, Skybox, Sphere};
use crate::shaders::{CubeMap, Shader, Texture};
use crate::shadow::ShadowMap;
use crate::time::Time;
use crate::scene::object::SceneObject3D;
use crate::scene::material::{ColoredMaterial, TexturedMaterial};
use crate::scene::context::RenderContext;
use crate::ui::{Button, TextRenderer};
use crate::ui::inspector::Inspector;
use crate::math::ray::Ray;
use crate::scene::collider::Collider;

pub trait RenderMode {
    fn update(&mut self, time: &Time);
    fn render(&mut self);
    fn handle_event(&mut self, event: &WindowEvent, time: &mut Time);
}
pub struct Game {
    // Scene Objects
    center_cube: SceneObject3D<Rc<Cube>>,
    green_cube: SceneObject3D<Rc<Cube>>,
    red_cube: SceneObject3D<Rc<Cube>>,
    orbiting_spheres: Vec<SceneObject3D<Rc<Sphere>>>,
    capsules: Vec<SceneObject3D<Rc<Capsule>>>,
    
    skybox: Skybox,

    // Shaders necessary for dynamic UI elements
    ui_rect_shader: Rc<Shader>,
    skybox_shader: Rc<Shader>,

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
    light_space_matrix: Mat4,
    selected_object_id: Option<usize>,
    inspector: Inspector,
}

impl Game {
    pub fn new() -> Self {
        println!("Initializing Game...");
        // Load shaders
        let colored_shader = Rc::new(
            Shader::from_files("assets/shaders/lit.vert", "assets/shaders/lit_color.frag")
                .expect("Failed to create colored shader"),
        );
        println!("Colored shader loaded.");

        let textured_shader = Rc::new(
            Shader::from_files(
                "assets/shaders/lit.vert",
                "assets/shaders/lit_textured.frag",
            )
            .expect("Failed to create textured shader"),
        );
        println!("Textured shader loaded.");

        let ui_shader = Rc::new(
            Shader::from_files("assets/shaders/ui.vert", "assets/shaders/ui_text.frag")
                .expect("Failed to create UI text shader"),
        );
        println!("UI shader loaded.");

        let ui_rect_shader = Rc::new(
            Shader::from_files("assets/shaders/ui.vert", "assets/shaders/ui_color.frag")
                .expect("Failed to create UI rect shader"),
        );
        println!("UI rect shader loaded.");

        let skybox_shader = Rc::new(
            Shader::from_files("assets/shaders/skybox.vert", "assets/shaders/skybox.frag")
                .expect("Failed to create skybox shader"),
        );
        println!("Skybox shader loaded.");

        let texture = Rc::new(
            Texture::from_file("assets/resources/textures/Poliigon_GrassPatchyGround_4585_BaseColor.jpg")
                .expect("Failed to load texture")
        );
        println!("Texture loaded.");

        let sphere_texture = Rc::new(
            Texture::from_file("assets/resources/textures/StoneBricks_1K.tiff")
                .expect("Failed to load sphere texture")
        );
        println!("Sphere texture loaded.");

        let skybox_cubemap =
            CubeMap::from_cross_file("assets/resources/textures/Cubemap_Sky_22-512x512.png")
                .expect("Failed to load skybox cubemap");
        println!("Skybox cubemap loaded.");

        let text_renderer = TextRenderer::new(ui_shader.clone());
        println!("TextRenderer initialized.");

        // Shadow map (2048x2048 resolution)
        let shadow_map = ShadowMap::new(2048, 2048);
        println!("Shadow map initialized.");

        let light = DirectionalLight::simple(Vec3::new(-0.2, -1.0, -0.3), 0.1, 0.5, 1.0, 32.0);

        // Create Shared Meshes
        let cube_mesh = Rc::new(Cube::new(1.0));
        let sphere_mesh = Rc::new(Sphere::new(0.6, 32, 32));
        let capsule_mesh = Rc::new(Capsule::new(0.4, 1.2, 32, 16, 16));

        // Create Materials
        let grass_material = Rc::new(TexturedMaterial {
            shader: textured_shader.clone(),
            texture: texture.clone(),
            is_lit: true,
            receive_shadows: true,
        });
        
        // Stone material - Let's make it receive no shadows as a test/demo? 
        // No, user just wants the ABILITY. Let's keep defaults but exposing them.
        let stone_material = Rc::new(TexturedMaterial {
            shader: textured_shader.clone(),
            texture: sphere_texture.clone(),
            is_lit: true,
            receive_shadows: true,
        });

        let green_material = Rc::new(ColoredMaterial {
            shader: colored_shader.clone(),
            color: Vec3::new(0.5, 0.8, 0.2),
            is_lit: true,
            receive_shadows: true,
        });

        // Red material - Unlit demo?
        // Let's make the red cube Unlit for demonstration if user wants.
        // But for now let's set them all enabled.
        let red_material = Rc::new(ColoredMaterial {
            shader: colored_shader.clone(),
            color: Vec3::new(1.0, 0.0, 0.0),
            is_lit: true,
            receive_shadows: true,
        });

        // Create Scene Objects
        let center_cube = SceneObject3D::new(cube_mesh.clone(), grass_material.clone())
            .with_name("Center Cube")
            .with_collider(Collider::new_cube(1.0));
        let green_cube = SceneObject3D::new(cube_mesh.clone(), green_material.clone())
            .with_name("Green Cube")
            .with_collider(Collider::new_cube(1.0));
        let red_cube = SceneObject3D::new(cube_mesh.clone(), red_material.clone())
            .with_name("Red Cube")
            .with_collider(Collider::new_cube(1.0));
        
        let mut orbiting_spheres = Vec::new();
        for i in 0..2 {
             orbiting_spheres.push(SceneObject3D::new(sphere_mesh.clone(), stone_material.clone())
                .with_name(&format!("Orbiting Sphere {}", i))
                .with_collider(Collider::new_sphere(0.6)));
        }

        let mut capsules = Vec::new();
        for i in 0..2 {
             capsules.push(SceneObject3D::new(capsule_mesh.clone(), grass_material.clone())
                .with_name(&format!("Floating Capsule {}", i))
                .with_collider(Collider::new_box(Vec3::new(-0.4, -1.0, -0.4), Vec3::new(0.4, 1.0, 0.4))));
        }

        Self {
            center_cube,
            green_cube,
            red_cube,
            orbiting_spheres,
            capsules,
            
            skybox: Skybox::new(),
            ui_rect_shader,
            skybox_shader,
            skybox_cubemap,
            
            text_renderer: TextRenderer::new(ui_shader),
            pause_button: Button::new("Pause", 1170.0, 660.0, 100.0, 40.0),
            
            input: Input::new(),
            camera: OrbitCamera::new(),
            shadow_map,
            light,
            point_light: PointLight::simple(Vec3::new(3.0, 3.0, 3.0), 0.05, 0.8, 1.0, 32.0),
            
            light_space_matrix: Mat4::IDENTITY,
            selected_object_id: None,
            inspector: Inspector::new(1070.0, 500.0),
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

    fn render_shadow_pass(&self) {
        self.shadow_map.begin_pass();
        self.shadow_map
            .set_light_space_matrix(&self.light_space_matrix);

        // Draw objects to depth map
        self.center_cube.render_depth(&self.shadow_map.shader);
        self.green_cube.render_depth(&self.shadow_map.shader);
        self.red_cube.render_depth(&self.shadow_map.shader);

        for obj in &self.orbiting_spheres {
            obj.render_depth(&self.shadow_map.shader);
        }
        
        for obj in &self.capsules {
            obj.render_depth(&self.shadow_map.shader);
        }

        self.shadow_map.end_pass(1280, 720);
    }

    fn render_objects(&self, projection: &Mat4, view: &Mat4) {
        let context = RenderContext {
            projection: *projection,
            view: *view,
            view_pos: self.camera.position,
            light: &self.light,
            point_light: &self.point_light,
            shadow_map: &self.shadow_map,
            light_space_matrix: self.light_space_matrix,
        };
        
        self.center_cube.render(&context);
        self.green_cube.render(&context);
        self.red_cube.render(&context);

        for obj in &self.orbiting_spheres {
             obj.render(&context);
        }
        
        for obj in &self.capsules {
            obj.render(&context);
        }
    }

    fn render_ui(&self) {
        self.text_renderer.render_rect(
            &self.ui_rect_shader,
            10.0,
            660.0, // Adjusted for 720p (720 - 60)
            180.0,
            50.0,
            glam::Vec4::new(0.0, 0.0, 0.0, 0.5),
            1280.0,
            720.0,
        );
        self.text_renderer.render_text(
            "Hakkology",
            20.0,
            665.0,
            32.0,
            Vec3::new(1.0, 1.0, 1.0),
            1280.0,
            720.0,
        );
        self.pause_button.draw(&self.text_renderer, &self.ui_rect_shader, 1280.0, 720.0);

        if let Some(id) = self.selected_object_id {
             let mut found = None;
             if self.center_cube.id == id { found = Some((&self.center_cube.name, self.center_cube.transform.position)); }
             else if self.green_cube.id == id { found = Some((&self.green_cube.name, self.green_cube.transform.position)); }
             else if self.red_cube.id == id { found = Some((&self.red_cube.name, self.red_cube.transform.position)); }
             else {
                 for obj in &self.orbiting_spheres { if obj.id == id { found = Some((&obj.name, obj.transform.position)); break; } }
                 if found.is_none() { for obj in &self.capsules { if obj.id == id { found = Some((&obj.name, obj.transform.position)); break; } } }
             }

             if let Some((name, pos)) = found {
                 self.inspector.draw(&self.text_renderer, &self.ui_rect_shader, 1280.0, 720.0, name, pos);
             }
        }
    }

    fn cast_ray(&self, ray: &Ray) -> Option<usize> {
        let mut min_dist = f32::MAX;
        let mut hit_id = None;
        let mut check = |dist: f32, id: usize| {
            if dist < min_dist {
                min_dist = dist;
                hit_id = Some(id);
            }
        };

        if let Some(dist) = self.center_cube.collider.as_ref().and_then(|c| c.intersect(ray, &self.center_cube.transform)) { check(dist, self.center_cube.id); }
        if let Some(dist) = self.green_cube.collider.as_ref().and_then(|c| c.intersect(ray, &self.green_cube.transform)) { check(dist, self.green_cube.id); }
        if let Some(dist) = self.red_cube.collider.as_ref().and_then(|c| c.intersect(ray, &self.red_cube.transform)) { check(dist, self.red_cube.id); }
        
        for obj in &self.orbiting_spheres { if let Some(dist) = obj.collider.as_ref().and_then(|c| c.intersect(ray, &obj.transform)) { check(dist, obj.id); } }
        for obj in &self.capsules { if let Some(dist) = obj.collider.as_ref().and_then(|c| c.intersect(ray, &obj.transform)) { check(dist, obj.id); } }
        
        hit_id
    }

    fn apply_transform_delta(&mut self, id: usize, delta: Vec3) {
         if self.center_cube.id == id { self.center_cube.transform.position += delta; }
         else if self.green_cube.id == id { self.green_cube.transform.position += delta; }
         else if self.red_cube.id == id { self.red_cube.transform.position += delta; }
         else {
             for obj in &mut self.orbiting_spheres { if obj.id == id { obj.transform.position += delta; return; } }
             for obj in &mut self.capsules { if obj.id == id { obj.transform.position += delta; return; } }
         }
    }


    fn check_intersection(&self, ray: &Ray) {
        let mut min_dist = f32::MAX;
        let mut hit_object: Option<(String, usize)> = None;

        // Helper closure to check intersection and update nearest
        let mut check = |dist: f32, name: &str, id: usize| {
            if dist < min_dist {
                min_dist = dist;
                hit_object = Some((name.to_string(), id));
            }
        };

        if let Some(dist) = self.center_cube.collider.as_ref().and_then(|c| c.intersect(ray, &self.center_cube.transform)) {
            check(dist, &self.center_cube.name, self.center_cube.id);
        }
        if let Some(dist) = self.green_cube.collider.as_ref().and_then(|c| c.intersect(ray, &self.green_cube.transform)) {
             check(dist, &self.green_cube.name, self.green_cube.id);
        }
         if let Some(dist) = self.red_cube.collider.as_ref().and_then(|c| c.intersect(ray, &self.red_cube.transform)) {
             check(dist, &self.red_cube.name, self.red_cube.id);
        }
        
        for p in &self.orbiting_spheres {
            if let Some(dist) = p.collider.as_ref().and_then(|c| c.intersect(ray, &p.transform)) {
                check(dist, &p.name, p.id);
            }
        }
        
        for p in &self.capsules {
             if let Some(dist) = p.collider.as_ref().and_then(|c| c.intersect(ray, &p.transform)) {
                 check(dist, &p.name, p.id);
             }
        }
        
        if let Some((name, id)) = hit_object {
             println!("Raycast Hit: '{}' (ID: {}) at distance {:.2}", name, id, min_dist);
        } else {
             println!("Raycast Miss");
        }
    }
}

impl RenderMode for Game {
    fn update(&mut self, time: &Time) {
        // self.time += delta_time; // No longer manual accumulation
        let current_time = time.time();
        let delta_time = time.delta_time;
        
        self.camera.update(&self.input, delta_time);

        // Update light space matrix for shadows
        self.light_space_matrix =
            self.shadow_map
                .light_space_matrix(self.light.direction, Vec3::ZERO, 10.0);

        self.input.reset_delta();

        // Animated Objects logic
        // Orbiting Spheres
        let configs = [(2.5, 1.2), (4.0, 0.8)];
        for (i, (radius, speed)) in configs.iter().enumerate() {
            if i < self.orbiting_spheres.len() {
                 let x = (current_time * speed).cos() * radius;
                 let z = (current_time * speed).sin() * radius;
                 self.orbiting_spheres[i].transform.position = Vec3::new(x, 0.0, z);
            }
        }
        
        // Green Cube
        self.green_cube.transform.position = Vec3::new(0.0, 2.0, 0.0);
        self.green_cube.transform.rotation = Quat::from_rotation_y(current_time);
        
        // Red Cube
        self.red_cube.transform.position = Vec3::new(0.0, -2.0, 0.0);
        self.red_cube.transform.rotation = Quat::from_rotation_y(-current_time);
        
        // Capsules
        let tilt = 45.0f32.to_radians();
        let tilt_quat = Quat::from_rotation_z(tilt);
        
        for i in 0..2 {
             if i >= self.capsules.len() { break; }
             let offset = i as f32 * std::f32::consts::PI;
             let angle = current_time * 0.7 + offset;
             let orbit_pos = Vec3::new(angle.cos() * 4.0, 0.0, angle.sin() * 4.0);
             let tilted_pos = tilt_quat.mul_vec3(orbit_pos);
             
             self.capsules[i].transform.position = tilted_pos;
             self.capsules[i].transform.rotation = Quat::from_rotation_y(current_time) * Quat::from_rotation_x(tilt);
        }
    }

    fn render(&mut self) {
        // Shadow pass
        self.render_shadow_pass();

        let projection = self.camera.projection_matrix(1280.0 / 720.0);
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
            
            // 1. Pause Button
            if self.pause_button.is_clicked(mx, my, 720.0) {
                time.toggle_pause();
                self.pause_button.text = if time.is_paused { "Resume".to_string() } else { "Pause".to_string() };
                return;
            }

            // 2. Inspector Interaction
            if self.selected_object_id.is_some() {
                let delta = self.inspector.check_clicks(mx, my, 720.0);
                if delta != Vec3::ZERO {
                    if let Some(id) = self.selected_object_id {
                        self.apply_transform_delta(id, delta);
                    }
                    return;
                }
            }

            // 3. Scene Selection (Raycast)
            let ray = self.camera.screen_point_to_ray(mx, my, 1280.0, 720.0);
            self.selected_object_id = self.cast_ray(&ray);
            self.check_intersection(&ray); // For debug log
        }
    }
}
