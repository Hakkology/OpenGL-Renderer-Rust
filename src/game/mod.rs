use glam::{Mat4, Quat, Vec2, Vec3};
use glfw::{Action, WindowEvent};
use std::rc::Rc;

use crate::camera::OrbitCamera;
use crate::importer::AssetImporter;
use crate::input::Input;
use crate::light::{components::LightProperties, DirectionalLight, PointLight};
use crate::math::ray::Ray;
use crate::primitives::{Capsule, Cube, Plane, Skybox, Sphere};
use crate::scene::collider::Collider;
use crate::scene::context::RenderContext;
use crate::scene::material::{ColoredMaterial, TexturedMaterial};
use crate::scene::object::SceneObject3D;
use crate::shaders::{CubeMap, Shader, Texture};
use crate::shadow::ShadowMap;
use crate::time::Time;
use crate::ui::inspector::Inspector;
use crate::ui::{Button, TextRenderer};

pub trait RenderMode {
    fn update(&mut self, time: &Time);
    fn render(&mut self);
    fn handle_event(&mut self, event: &WindowEvent, time: &mut Time);
}
pub struct Game {
    // Scene Objects
    center_cube: SceneObject3D,
    green_cube: SceneObject3D,
    red_cube: SceneObject3D,
    orbiting_spheres: Vec<SceneObject3D>,
    capsules: Vec<SceneObject3D>,
    floor: SceneObject3D,
    walls: Vec<SceneObject3D>,
    trees: Vec<SceneObject3D>,
    xwing: SceneObject3D,
    statues: Vec<SceneObject3D>,

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
    point_shadow_maps: Vec<crate::shadow::PointShadowMap>,

    // Lights
    light: DirectionalLight,
    point_lights: Vec<PointLight>,

    // State
    light_space_matrix: Mat4,
    selected_object_id: Option<usize>,
    inspector: Inspector,
    frame_count: u64,
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
            Texture::from_file(
                "assets/resources/textures/Poliigon_GrassPatchyGround_4585_BaseColor.jpg",
            )
            .expect("Failed to load texture"),
        );
        println!("Texture loaded.");

        let sphere_texture = Rc::new(
            Texture::from_file("assets/resources/textures/StoneBricks_1K.tiff")
                .expect("Failed to load sphere texture"),
        );
        println!("Sphere texture loaded.");

        let skybox_cubemap =
            CubeMap::from_cross_file("assets/resources/textures/Cubemap_Sky_22-512x512.png")
                .expect("Failed to load skybox cubemap");
        println!("Skybox cubemap loaded.");

        let text_renderer = TextRenderer::new(ui_shader.clone());
        println!("TextRenderer initialized.");

        // Shadow map (2048x2048 resolution)
        let shadow_map = crate::shadow::ShadowMap::new(2048, 2048);
        println!("Directional shadow map initialized.");

        let light = DirectionalLight::simple(Vec3::new(-0.2, -1.0, -0.3), 0.1, 0.3, 1.0, 32.0);

        // Point shadow maps (one for each light)
        let mut point_shadow_maps = Vec::new();
        for _ in 0..4 {
            point_shadow_maps.push(crate::shadow::PointShadowMap::new(512));
        }
        println!("Point shadow maps initialized with 512x512 resolution.");

        // Create Shared Meshes
        let cube_mesh = Rc::new(Cube::new(1.0));
        let sphere_mesh = Rc::new(Sphere::new(0.6, 32, 32));
        let capsule_mesh = Rc::new(Capsule::new(0.4, 1.2, 32, 16, 16));
        let plane_mesh = Rc::new(Plane::new(80.0));

        // Create Materials
        let grass_material = Rc::new(TexturedMaterial {
            shader: textured_shader.clone(),
            texture: texture.clone(),
            is_lit: true,
            is_repeated: false,
            uv_scale: Vec2::ONE,
            receive_shadows: true,
        });

        let stone_material = Rc::new(TexturedMaterial {
            shader: textured_shader.clone(),
            texture: sphere_texture.clone(),
            is_lit: true,
            is_repeated: false,
            uv_scale: Vec2::ONE,
            receive_shadows: true,
        });

        let green_material = Rc::new(ColoredMaterial {
            shader: colored_shader.clone(),
            color: Vec3::new(0.5, 0.8, 0.2),
            is_lit: true,
            receive_shadows: true,
        });

        let red_material = Rc::new(ColoredMaterial {
            shader: colored_shader.clone(),
            color: Vec3::new(1.0, 0.0, 0.0),
            is_lit: true,
            receive_shadows: true,
        });

        // Create Scene Objects
        let center_cube = SceneObject3D::new(Box::new(cube_mesh.clone()), grass_material.clone())
            .with_name("Center Cube")
            .with_collider(Collider::new_cube(1.0));
        let green_cube = SceneObject3D::new(Box::new(cube_mesh.clone()), green_material.clone())
            .with_name("Green Cube")
            .with_collider(Collider::new_cube(1.0));
        let red_cube = SceneObject3D::new(Box::new(cube_mesh.clone()), red_material.clone())
            .with_name("Red Cube")
            .with_collider(Collider::new_cube(1.0));

        let mut orbiting_spheres = Vec::new();
        for i in 0..2 {
            orbiting_spheres.push(
                SceneObject3D::new(Box::new(sphere_mesh.clone()), stone_material.clone())
                    .with_name(&format!("Orbiting Sphere {}", i))
                    .with_collider(Collider::new_sphere(0.6)),
            );
        }

        let mut capsules = Vec::new();
        for i in 0..2 {
            capsules.push(
                SceneObject3D::new(Box::new(capsule_mesh.clone()), grass_material.clone())
                    .with_name(&format!("Floating Capsule {}", i))
                    .with_collider(Collider::new_box(
                        Vec3::new(-0.4, -1.0, -0.4),
                        Vec3::new(0.4, 1.0, 0.4),
                    )),
            );
        }

        let mut floor = SceneObject3D::new(Box::new(plane_mesh), grass_material.clone())
            .with_name("Floor")
            .with_collider(Collider::new_box(
                Vec3::new(-40.0, -0.01, -40.0),
                Vec3::new(40.0, 0.01, 40.0),
            ));
        floor.transform.position = Vec3::new(0.0, -4.0, 0.0);

        // Walls
        let mut walls = Vec::new();
        let wall_height = 8.0;
        let plane_size = 80.0;
        let half_size = plane_size / 2.0;
        let wall_thickness = 1.0;

        let wall_mat_x = Rc::new(TexturedMaterial {
            shader: textured_shader.clone(),
            texture: sphere_texture.clone(),
            is_lit: true,
            is_repeated: true,
            uv_scale: Vec2::new(wall_height / 8.0, plane_size / 8.0),
            receive_shadows: true,
        });

        let wall_mat_z = Rc::new(TexturedMaterial {
            shader: textured_shader.clone(),
            texture: sphere_texture.clone(),
            is_lit: true,
            is_repeated: true,
            uv_scale: Vec2::new(plane_size / 8.0, wall_height / 8.0),
            receive_shadows: true,
        });

        // +X Wall (Inner face normal is -X)
        let mut w1 = SceneObject3D::new(Box::new(cube_mesh.clone()), wall_mat_x.clone())
            .with_name("Wall +X")
            .with_collider(Collider::new_cube(1.0));
        w1.transform.position = Vec3::new(half_size, -4.0 + wall_height / 2.0, 0.0);
        w1.transform.scale = Vec3::new(wall_thickness, wall_height, plane_size);
        walls.push(w1);

        // -X Wall (Inner face normal is +X)
        let mut w2 = SceneObject3D::new(Box::new(cube_mesh.clone()), wall_mat_x.clone())
            .with_name("Wall -X")
            .with_collider(Collider::new_cube(1.0));
        w2.transform.position = Vec3::new(-half_size, -4.0 + wall_height / 2.0, 0.0);
        w2.transform.scale = Vec3::new(wall_thickness, wall_height, plane_size);
        walls.push(w2);

        // +Z Wall (Inner face normal is -Z)
        let mut w3 = SceneObject3D::new(Box::new(cube_mesh.clone()), wall_mat_z.clone())
            .with_name("Wall +Z")
            .with_collider(Collider::new_cube(1.0));
        w3.transform.position = Vec3::new(0.0, -4.0 + wall_height / 2.0, half_size);
        w3.transform.scale = Vec3::new(plane_size, wall_height, wall_thickness);
        walls.push(w3);

        // -Z Wall (Inner face normal is +Z)
        let mut w4 = SceneObject3D::new(Box::new(cube_mesh.clone()), wall_mat_z.clone())
            .with_name("Wall -Z")
            .with_collider(Collider::new_cube(1.0));
        w4.transform.position = Vec3::new(0.0, -4.0 + wall_height / 2.0, -half_size);
        w4.transform.scale = Vec3::new(plane_size, wall_height, wall_thickness);
        walls.push(w4);

        // Load Trees
        let tree2_model = Rc::new(
            AssetImporter::load_model("assets/resources/models/Tree2/trees9.obj")
                .expect("Failed to load tree2"),
        );

        let mut trees = Vec::new();
        let tree_positions = [Vec3::new(-8.0, -4.0, -8.0), Vec3::new(8.0, -4.0, 8.0)];

        for (i, pos) in tree_positions.iter().enumerate() {
            let mut tree =
                SceneObject3D::new(Box::new(tree2_model.clone()), green_material.clone())
                    .with_name(&format!("Tree {}", i))
                    .with_collider(Collider::new_box(
                        Vec3::new(-0.5, 0.0, -0.5),
                        Vec3::new(0.5, 3.0, 0.5),
                    ));

            tree.transform.position = *pos;
            tree.transform.scale = Vec3::splat(0.8);
            trees.push(tree);
        }

        // Load X-Wing
        let xwing_model = Rc::new(
            AssetImporter::load_model("assets/resources/models/xwing/x-wing.obj")
                .expect("Failed to load xwing"),
        );

        let grey_material = Rc::new(ColoredMaterial {
            shader: colored_shader.clone(),
            color: Vec3::new(0.7, 0.7, 0.7),
            is_lit: true,
            receive_shadows: true,
        });

        let mut xwing = SceneObject3D::new(Box::new(xwing_model.clone()), grey_material.clone())
            .with_name("X-Wing")
            .with_collider(Collider::new_sphere(2.0));
        xwing.transform.position = Vec3::new(0.0, 50.0, 10.0);
        xwing.transform.scale = Vec3::splat(1.0);

        // Load Statues
        let statue_model = Rc::new(
            AssetImporter::load_model("assets/resources/models/Statue/12334_statue_v1_l3.obj")
                .expect("Failed to load statue"),
        );

        let mut statues = Vec::new();
        let statue_configs = [
            (Vec3::new(0.0, -3.9, 20.0), 180.0f32), // +Z, facing -Z (180 deg Y)
            (Vec3::new(0.0, -3.9, -20.0), 0.0f32),  // -Z, facing +Z (0 deg Y)
            (Vec3::new(20.0, -3.9, 0.0), 90.0f32),  // +X, facing -X (90 deg Y)
            (Vec3::new(-20.0, -3.9, 0.0), -90.0f32), // -X, facing +X (-90 deg Y)
        ];

        for (i, (pos, yaw_deg)) in statue_configs.iter().enumerate() {
            let mut s = SceneObject3D::new(Box::new(statue_model.clone()), grey_material.clone())
                .with_name(&format!("Statue {}", i))
                .with_collider(Collider::new_sphere(500.0));
            s.transform.position = *pos;
            s.transform.scale = Vec3::splat(0.01);
            s.transform.rotation = Quat::from_rotation_y(yaw_deg.to_radians())
                * Quat::from_rotation_x(-90.0f32.to_radians());
            statues.push(s);
        }

        // Initialize 4 Point Lights
        let mut point_lights = Vec::new();
        let colors = [
            Vec3::new(1.0, 0.0, 0.0), // Red
            Vec3::new(0.0, 1.0, 0.0), // Green
            Vec3::new(0.0, 0.0, 1.0), // Blue
            Vec3::new(1.0, 0.5, 0.0), // Orange
        ];

        for i in 0..4 {
            point_lights.push(PointLight::new(
                Vec3::ZERO, // Position will be updated in update loop
                LightProperties::new(0.2, 2.5, 3.0, 32.0).with_color(colors[i % 4]),
            ));
        }

        Self {
            center_cube,
            green_cube,
            red_cube,
            orbiting_spheres,
            capsules,
            floor,
            walls,
            trees,
            xwing,
            statues,

            skybox: Skybox::new(),
            ui_rect_shader,
            skybox_shader,
            skybox_cubemap,

            text_renderer: TextRenderer::new(ui_shader),
            pause_button: Button::new("Pause", 1170.0, 660.0, 100.0, 40.0),

            input: Input::new(),
            camera: OrbitCamera::new(),
            shadow_map,
            point_shadow_maps,
            light,

            point_lights,
            light_space_matrix: Mat4::IDENTITY,
            selected_object_id: None,
            inspector: Inspector::new(1070.0, 500.0),
            frame_count: 0,
        }
    }

    fn each_object<F>(&self, mut f: F)
    where
        F: FnMut(&SceneObject3D),
    {
        f(&self.center_cube);
        f(&self.green_cube);
        f(&self.red_cube);
        f(&self.floor);
        f(&self.xwing);
        for obj in &self.orbiting_spheres {
            f(obj);
        }
        for obj in &self.capsules {
            f(obj);
        }
        for obj in &self.walls {
            f(obj);
        }
        for obj in &self.trees {
            f(obj);
        }
        for obj in &self.statues {
            f(obj);
        }
    }

    fn each_object_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut SceneObject3D),
    {
        f(&mut self.center_cube);
        f(&mut self.green_cube);
        f(&mut self.red_cube);
        f(&mut self.floor);
        f(&mut self.xwing);
        for obj in &mut self.orbiting_spheres {
            f(obj);
        }
        for obj in &mut self.capsules {
            f(obj);
        }
        for obj in &mut self.walls {
            f(obj);
        }
        for obj in &mut self.trees {
            f(obj);
        }
        for obj in &mut self.statues {
            f(obj);
        }
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

        self.each_object(|obj| obj.render_depth(&self.shadow_map.shader));
        self.shadow_map.end_pass(1280, 720);
    }

    fn render_point_shadow_pass(&mut self) {
        let far_plane = 25.0;
        for i in 0..2 {
            let offset = (self.frame_count % 2) as usize * 2;
            let light_idx = offset + i;

            if let Some(pl) = self.point_lights.get(light_idx) {
                if let Some(psm) = self.point_shadow_maps.get(light_idx) {
                    psm.begin_pass(pl.position, far_plane);
                    self.each_object(|obj| obj.render_depth(&psm.shader));
                    psm.end_pass(1280, 720);
                }
            }
        }
    }

    fn render_objects(&self, projection: &Mat4, view: &Mat4) {
        let context = RenderContext {
            projection: *projection,
            view: *view,
            view_pos: self.camera.position,
            light: &self.light,
            point_lights: &self.point_lights,
            shadow_map: &self.shadow_map,
            point_shadow_maps: &self.point_shadow_maps,
            far_plane: 25.0,
            light_space_matrix: self.light_space_matrix,
        };

        self.each_object(|obj| obj.render(&context));
    }

    fn render_ui(&self) {
        self.text_renderer.render_rect(
            &self.ui_rect_shader,
            10.0,
            660.0,
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
        self.pause_button
            .draw(&self.text_renderer, &self.ui_rect_shader, 1280.0, 720.0);

        if let Some(id) = self.selected_object_id {
            let mut found = None;
            self.each_object(|obj| {
                if obj.id == id {
                    found = Some((obj.name.clone(), obj.transform.position));
                }
            });

            if let Some((name, pos)) = found {
                self.inspector.draw(
                    &self.text_renderer,
                    &self.ui_rect_shader,
                    1280.0,
                    720.0,
                    &name,
                    pos,
                );
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

        // Unified Raycasting
        self.each_object(|obj| {
            if let Some(dist) = obj
                .collider
                .as_ref()
                .and_then(|c| c.intersect(ray, &obj.transform))
            {
                check(dist, obj.id);
            }
        });

        hit_id
    }

    fn apply_transform_delta(&mut self, id: usize, delta: Vec3) {
        self.each_object_mut(|obj| {
            if obj.id == id {
                obj.transform.position += delta;
            }
        });
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

        self.each_object(|obj| {
            if let Some(dist) = obj
                .collider
                .as_ref()
                .and_then(|c| c.intersect(ray, &obj.transform))
            {
                check(dist, &obj.name, obj.id);
            }
        });

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

impl RenderMode for Game {
    fn update(&mut self, time: &Time) {
        // self.time += delta_time; // No longer manual accumulation
        let current_time = time.time();
        let delta_time = time.delta_time;

        self.camera.update(&self.input, delta_time);

        // Update light space matrix for shadows
        self.light_space_matrix =
            self.shadow_map
                .light_space_matrix(self.light.direction, Vec3::ZERO, 35.0);

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
            if i >= self.capsules.len() {
                break;
            }
            let offset = i as f32 * std::f32::consts::PI;
            let angle = current_time * 0.7 + offset;
            let orbit_pos = Vec3::new(angle.cos() * 4.0, 0.0, angle.sin() * 4.0);
            let tilted_pos = tilt_quat.mul_vec3(orbit_pos);

            self.capsules[i].transform.position = tilted_pos;
            self.capsules[i].transform.rotation =
                Quat::from_rotation_y(current_time) * Quat::from_rotation_x(tilt);
        }

        // Statue Orbiting Light Logic (4 lights around 4 statues)
        for i in 0..4 {
            if i >= self.statues.len() || i >= self.point_lights.len() {
                break;
            }

            let statue_pos = self.statues[i].transform.position;
            let light_radius = 2.0;
            let light_speed = 1.0;
            let light_angle = current_time * light_speed + (i as f32 * std::f32::consts::PI / 2.0); // Offset phases

            let light_x = statue_pos.x + light_angle.cos() * light_radius;
            let light_z = statue_pos.z + light_angle.sin() * light_radius;
            let light_y = statue_pos.y + 1.5;

            self.point_lights[i].position = Vec3::new(light_x, light_y, light_z);
        }

        self.frame_count += 1;
    }

    fn render(&mut self) {
        // Shadow passes
        self.render_shadow_pass();
        self.render_point_shadow_pass();

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

            // Pause Button
            if self.pause_button.is_clicked(mx, my, 720.0) {
                time.toggle_pause();
                self.pause_button.text = if time.is_paused {
                    "Resume".to_string()
                } else {
                    "Pause".to_string()
                };
                return;
            }

            // Inspector Interaction
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
