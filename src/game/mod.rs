use glam::{Quat, Vec2, Vec3};
use glfw::{Action, WindowEvent};
use std::rc::Rc;

use crate::camera::OrbitCamera;
use crate::importer::AssetImporter;
use crate::input::Input;
use crate::light::{
    components::{Attenuation, LightProperties},
    DirectionalLight, PointLight,
};
use crate::math::ray::Ray;
use crate::primitives::{Capsule, Cube, Plane, Sphere};
use crate::renderer::Renderer;
use crate::scene::collider::Collider;

use crate::scene::manager::Scene;
use crate::scene::material::{ColoredMaterial, TexturedMaterial};
use crate::scene::object::SceneObject3D;
use crate::shaders::{CubeMap, Shader, Texture};
use crate::time::Time;
use crate::ui::TextRenderer;
use crate::ui::UIManager;

pub trait RenderMode {
    fn update(&mut self, time: &Time);
    fn render(&mut self);
    fn handle_event(&mut self, event: &WindowEvent, time: &mut Time);
}
pub struct Game {
    // Scene
    scene: Scene,

    // UI
    ui_manager: UIManager,
    selected_object_id: Option<usize>,

    // Systems
    renderer: Renderer,
    input: Input,
    camera: OrbitCamera,

    // Lights
    light: DirectionalLight,
    point_lights: Vec<PointLight>,

    // State
    is_paused: bool,
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

        let text_renderer = TextRenderer::new(ui_shader);
        println!("TextRenderer initialized.");

        // Initialize Renderer
        let renderer = Renderer::new(skybox_shader, skybox_cubemap);
        println!("Renderer initialized.");

        let light = DirectionalLight::simple(Vec3::new(-0.2, -1.0, -0.3), 0.1, 0.3, 1.0, 32.0);

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

        let mut scene = Scene::new();

        // Create Scene Objects
        let center_cube = SceneObject3D::new(Box::new(cube_mesh.clone()), grass_material.clone())
            .with_name("Center Cube")
            .with_collider(Collider::new_cube(1.0));
        scene.add_object(center_cube);

        let green_cube = SceneObject3D::new(Box::new(cube_mesh.clone()), green_material.clone())
            .with_name("Green Cube")
            .with_collider(Collider::new_cube(1.0));
        scene.green_cube_id = scene.add_object(green_cube);

        let red_cube = SceneObject3D::new(Box::new(cube_mesh.clone()), red_material.clone())
            .with_name("Red Cube")
            .with_collider(Collider::new_cube(1.0));
        scene.red_cube_id = scene.add_object(red_cube);

        for i in 0..2 {
            let sphere = SceneObject3D::new(Box::new(sphere_mesh.clone()), stone_material.clone())
                .with_name(&format!("Orbiting Sphere {}", i))
                .with_collider(Collider::new_sphere(0.6));
            scene.orbiting_sphere_ids.push(sphere.id);
            scene.add_object(sphere);
        }

        for i in 0..2 {
            let capsule =
                SceneObject3D::new(Box::new(capsule_mesh.clone()), grass_material.clone())
                    .with_name(&format!("Floating Capsule {}", i))
                    .with_collider(Collider::new_box(
                        Vec3::new(-0.4, -1.0, -0.4),
                        Vec3::new(0.4, 1.0, 0.4),
                    ));
            scene.capsule_ids.push(capsule.id);
            scene.add_object(capsule);
        }

        let mut floor = SceneObject3D::new(Box::new(plane_mesh), grass_material.clone())
            .with_name("Floor")
            .with_collider(Collider::new_box(
                Vec3::new(-40.0, -0.01, -40.0),
                Vec3::new(40.0, 0.01, 40.0),
            ));
        floor.transform.position = Vec3::new(0.0, -4.0, 0.0);
        scene.add_object(floor);

        // Walls
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

        let mut w1 = SceneObject3D::new(Box::new(cube_mesh.clone()), wall_mat_x.clone())
            .with_name("Wall +X")
            .with_collider(Collider::new_cube(1.0));
        w1.transform.position = Vec3::new(half_size, -4.0 + wall_height / 2.0, 0.0);
        w1.transform.scale = Vec3::new(wall_thickness, wall_height, plane_size);
        scene.add_object(w1);

        let mut w2 = SceneObject3D::new(Box::new(cube_mesh.clone()), wall_mat_x.clone())
            .with_name("Wall -X")
            .with_collider(Collider::new_cube(1.0));
        w2.transform.position = Vec3::new(-half_size, -4.0 + wall_height / 2.0, 0.0);
        w2.transform.scale = Vec3::new(wall_thickness, wall_height, plane_size);
        scene.add_object(w2);

        let mut w3 = SceneObject3D::new(Box::new(cube_mesh.clone()), wall_mat_z.clone())
            .with_name("Wall +Z")
            .with_collider(Collider::new_cube(1.0));
        w3.transform.position = Vec3::new(0.0, -4.0 + wall_height / 2.0, half_size);
        w3.transform.scale = Vec3::new(plane_size, wall_height, wall_thickness);
        scene.add_object(w3);

        let mut w4 = SceneObject3D::new(Box::new(cube_mesh.clone()), wall_mat_z.clone())
            .with_name("Wall -Z")
            .with_collider(Collider::new_cube(1.0));
        w4.transform.position = Vec3::new(0.0, -4.0 + wall_height / 2.0, -half_size);
        w4.transform.scale = Vec3::new(plane_size, wall_height, wall_thickness);
        scene.add_object(w4);

        // Load Trees
        let tree2_model = Rc::new(
            AssetImporter::load_model("assets/resources/models/Tree2/trees9.obj")
                .expect("Failed to load tree2"),
        );
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
            scene.add_object(tree);
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
        scene.xwing_id = scene.add_object(xwing);

        // Load Statues
        let statue_model = Rc::new(
            AssetImporter::load_model("assets/resources/models/Statue/12334_statue_v1_l3.obj")
                .expect("Failed to load statue"),
        );

        let statue_configs = [
            (Vec3::new(0.0, -3.9, 20.0), 180.0f32),
            (Vec3::new(0.0, -3.9, -20.0), 0.0f32),
            (Vec3::new(20.0, -3.9, 0.0), 90.0f32),
            (Vec3::new(-20.0, -3.9, 0.0), -90.0f32),
        ];

        for (i, (pos, yaw_deg)) in statue_configs.iter().enumerate() {
            let mut s = SceneObject3D::new(Box::new(statue_model.clone()), grey_material.clone())
                .with_name(&format!("Statue {}", i))
                .with_collider(Collider::new_sphere(500.0));
            s.transform.position = *pos;
            s.transform.scale = Vec3::splat(0.01);
            s.transform.rotation = Quat::from_rotation_y(yaw_deg.to_radians())
                * Quat::from_rotation_x(-90.0f32.to_radians());
            scene.statue_ids.push(s.id);
            scene.add_object(s);
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
            point_lights.push(
                PointLight::new(
                    Vec3::ZERO, // Position will be updated in update loop
                    LightProperties::new(0.2, 2.5, 3.0, 32.0).with_color(colors[i % 4]),
                )
                .with_attenuation(Attenuation::new(1.0, 0.35, 0.44)), // Increased attenuation for more focused light
            );
        }

        // Initialize UI Manager
        let ui_manager = UIManager::new(text_renderer, ui_rect_shader);
        println!("UIManager initialized.");

        Self {
            scene,

            ui_manager,
            selected_object_id: None,

            renderer,
            input: Input::new(),
            camera: OrbitCamera::new(),
            light,

            point_lights: point_lights.to_vec(),
            is_paused: false,
        }
    }

    fn cast_ray(&self, ray: &Ray) -> Option<usize> {
        self.scene.cast_ray(ray)
    }

    fn apply_transform_delta(&mut self, id: usize, delta: Vec3) {
        if let Some(obj) = self.scene.get_object_mut(id) {
            obj.transform.position += delta;
        }
    }

    fn check_intersection(&self, ray: &Ray) {
        self.scene.check_intersection(ray);
    }
}

impl RenderMode for Game {
    fn update(&mut self, time: &Time) {
        let current_time = time.time();
        let delta_time = time.delta_time;

        self.camera.update(&self.input, delta_time);

        self.input.reset_delta();

        // Animated Objects logic
        for obj in &mut self.scene.objects {
            if obj.id == self.scene.green_cube_id {
                obj.transform.position = Vec3::new(0.0, 2.0, 0.0);
                obj.transform.rotation = Quat::from_rotation_y(current_time);
            } else if obj.id == self.scene.red_cube_id {
                obj.transform.position = Vec3::new(0.0, -2.0, 0.0);
                obj.transform.rotation = Quat::from_rotation_y(-current_time);
            } else if let Some(i) = self
                .scene
                .orbiting_sphere_ids
                .iter()
                .position(|&id| id == obj.id)
            {
                let configs = [(2.5, 1.2), (4.0, 0.8)];
                let (radius, speed) = configs[i];
                let x = (current_time * speed).cos() * radius;
                let z = (current_time * speed).sin() * radius;
                obj.transform.position = Vec3::new(x, 0.0, z);
            } else if let Some(i) = self.scene.capsule_ids.iter().position(|&id| id == obj.id) {
                let tilt = 45.0f32.to_radians();
                let tilt_quat = Quat::from_rotation_z(tilt);
                let offset = i as f32 * std::f32::consts::PI;
                let angle = current_time * 0.7 + offset;
                let orbit_pos = Vec3::new(angle.cos() * 4.0, 0.0, angle.sin() * 4.0);
                let tilted_pos = tilt_quat.mul_vec3(orbit_pos);

                obj.transform.position = tilted_pos;
                obj.transform.rotation =
                    Quat::from_rotation_y(current_time) * Quat::from_rotation_x(tilt);
            } else if obj.id == self.scene.xwing_id {
                // X-Wing: Slight oscillation
                obj.transform.position.y = 50.0 + (current_time * 0.5).sin() * 2.0;
            }
        }

        // Update Point Lights based on statue positions
        for i in 0..4 {
            let s_id = self.scene.statue_ids[i];
            if let Some(statue) = self.scene.objects.iter().find(|o| o.id == s_id) {
                let statue_pos = statue.transform.position;
                let light_radius = 2.0;
                let light_speed = 1.0;
                let light_angle =
                    current_time * light_speed + (i as f32 * std::f32::consts::PI / 2.0);

                let light_x = statue_pos.x + light_angle.cos() * light_radius;
                let light_z = statue_pos.z + light_angle.sin() * light_radius;
                let light_y = statue_pos.y + 1.5;

                self.point_lights[i].position = Vec3::new(light_x, light_y, light_z);
            }
        }
    }

    fn render(&mut self) {
        self.renderer
            .render(&self.scene, &self.camera, &self.light, &self.point_lights);
        self.ui_manager
            .render(&self.scene, self.selected_object_id, self.is_paused);
    }

    fn handle_event(&mut self, event: &WindowEvent, time: &mut Time) {
        self.input.handle_event(event);

        if let WindowEvent::MouseButton(glfw::MouseButtonLeft, Action::Press, _) = event {
            let (mx, my) = (self.input.mouse.pos.x, self.input.mouse.pos.y);

            // Pause Button
            if self.ui_manager.pause_button.is_clicked(mx, my, 720.0) {
                time.toggle_pause();
                self.is_paused = time.is_paused;
                return;
            }

            // Inspector Interaction
            if self.selected_object_id.is_some() {
                let delta = self.ui_manager.inspector.check_clicks(mx, my, 720.0);
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
