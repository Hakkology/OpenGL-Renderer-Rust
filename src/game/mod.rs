use glam::{Quat, Vec2, Vec3};
use glfw::{Action, WindowEvent};
use std::rc::Rc;

use crate::assets::paths::{models, names, shaders, textures};
use crate::assets::AssetManager;
use crate::camera::OrbitCamera;
use crate::config::{ui as ui_cfg, window as win_cfg};
use crate::input::Input;
use crate::light::{
    components::{Attenuation, LightProperties},
    DirectionalLight, PointLight,
};
use crate::logic::controller::{
    FloatingController, OrbitController, OscillationController, RotationController,
};
use crate::math::ray::Ray;
use crate::primitives::{Capsule, Cube, Plane, Sphere};
use crate::renderer::Renderer;
use crate::scene::collider::Collider;
use crate::scene::manager::Scene;
use crate::scene::material_factory::MaterialFactory;
use crate::scene::object::SceneObject3D;

use crate::time::Time;
use crate::ui::Button;
use crate::ui::TextRenderer;
use crate::ui::UIManager;

pub trait RenderMode {
    fn update(&mut self, time: &Time);
    fn render(&mut self);
    fn handle_event(&mut self, event: &WindowEvent, time: &mut Time);
}
pub struct Game {
    // Assets
    assets: AssetManager,

    // Scene
    scene: Scene,

    // UI
    ui_manager: UIManager,
    pause_button: Button,
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
        let mut assets = AssetManager::new();

        // 1. Shaders
        let colored_shader = assets.load_shader(
            names::SHADER_COLORED,
            shaders::LIT_VERT,
            shaders::LIT_COLOR_FRAG,
        );
        let textured_shader = assets.load_shader(
            names::SHADER_TEXTURED,
            shaders::LIT_VERT,
            shaders::LIT_TEXTURED_FRAG,
        );
        let ui_shader = assets.load_shader(
            names::SHADER_UI_TEXT,
            shaders::UI_VERT,
            shaders::UI_TEXT_FRAG,
        );
        let ui_rect_shader = assets.load_shader(
            names::SHADER_UI_COLOR,
            shaders::UI_VERT,
            shaders::UI_COLOR_FRAG,
        );
        let skybox_shader = assets.load_shader(
            names::SHADER_SKYBOX,
            shaders::SKYBOX_VERT,
            shaders::SKYBOX_FRAG,
        );

        // 2. Textures
        let texture = assets.load_texture(names::TEX_GRASS, textures::GRASS);
        let sphere_texture = assets.load_texture(names::TEX_STONE, textures::STONE_BRICKS);

        // 3. Cubemap
        let skybox_cubemap = assets.load_cubemap(names::TEX_SKYBOX, textures::SKYBOX);

        let text_renderer = TextRenderer::new(ui_shader);
        let ui_manager = UIManager::new(text_renderer, ui_rect_shader);

        // Renderer
        let renderer = Renderer::new(skybox_shader, skybox_cubemap);

        let light = DirectionalLight::simple(Vec3::new(-0.2, -1.0, -0.3), 0.1, 0.3, 1.0, 32.0);

        // Models
        let tree2_model = assets.load_model(names::MODEL_TREE, models::TREE);
        let xwing_model = assets.load_model(names::MODEL_XWING, models::XWING);
        let statue_model = assets.load_model(names::MODEL_STATUE, models::STATUE);

        // Shared Meshes
        let cube_mesh = Rc::new(Cube::new(1.0));
        let sphere_mesh = Rc::new(Sphere::new(0.6, 32, 32));
        let capsule_mesh = Rc::new(Capsule::new(0.4, 1.2, 32, 16, 16));
        let plane_mesh = Rc::new(Plane::new(80.0));

        // Create Material Factory
        let materials = MaterialFactory::new(colored_shader.clone(), textured_shader.clone());

        // Create Materials using Factory
        let grass_material = materials.textured(texture.clone());
        let stone_material = materials.textured(sphere_texture.clone());
        let green_material = materials.grass_green();
        let red_material = materials.red();
        let grey_material = materials.light_grey();

        let mut scene = Scene::new();

        // Create Scene Objects
        let center_cube = SceneObject3D::new(Box::new(cube_mesh.clone()), grass_material.clone())
            .with_name("Center Cube")
            .with_collider(Collider::new_cube(1.0));
        scene.add_object(center_cube);

        let mut green_cube =
            SceneObject3D::new(Box::new(cube_mesh.clone()), green_material.clone())
                .with_name("Green Cube")
                .with_collider(Collider::new_cube(1.0))
                .with_controller(Box::new(RotationController::new(Vec3::Y, 1.0)));
        green_cube.transform.position = Vec3::new(0.0, 2.0, 0.0);
        scene.green_cube_id = scene.add_object(green_cube);

        let mut red_cube = SceneObject3D::new(Box::new(cube_mesh.clone()), red_material.clone())
            .with_name("Red Cube")
            .with_collider(Collider::new_cube(1.0))
            .with_controller(Box::new(RotationController::new(Vec3::Y, -1.0)));
        red_cube.transform.position = Vec3::new(0.0, -2.0, 0.0);
        scene.red_cube_id = scene.add_object(red_cube);

        let configs = [(2.5, 1.2), (4.0, 0.8)];
        for i in 0..2 {
            let (radius, speed) = configs[i];
            let sphere = SceneObject3D::new(Box::new(sphere_mesh.clone()), stone_material.clone())
                .with_name(&format!("Orbiting Sphere {}", i))
                .with_collider(Collider::new_sphere(0.6))
                .with_controller(Box::new(OrbitController::new(
                    Vec3::ZERO,
                    radius,
                    speed,
                    i as f32 * std::f32::consts::PI,
                )));
            scene.orbiting_sphere_ids.push(sphere.id);
            scene.add_object(sphere);
        }

        for i in 0..2 {
            let offset = i as f32 * std::f32::consts::PI;
            let capsule =
                SceneObject3D::new(Box::new(capsule_mesh.clone()), grass_material.clone())
                    .with_name(&format!("Floating Capsule {}", i))
                    .with_collider(Collider::new_box(
                        Vec3::new(-0.4, -1.0, -0.4),
                        Vec3::new(0.4, 1.0, 0.4),
                    ))
                    .with_controller(Box::new(FloatingController::new(0.7, offset)));
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

        let wall_mat_x = materials.textured_tiled(
            sphere_texture.clone(),
            Vec2::new(wall_height / 8.0, plane_size / 8.0),
        );

        let wall_mat_z = materials.textured_tiled(
            sphere_texture.clone(),
            Vec2::new(plane_size / 8.0, wall_height / 8.0),
        );

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

        for (i, pos) in [Vec3::new(-8.0, -4.0, -8.0), Vec3::new(8.0, -4.0, 8.0)]
            .iter()
            .enumerate()
        {
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

        let mut xwing = SceneObject3D::new(Box::new(xwing_model), grey_material.clone())
            .with_name("X-Wing")
            .with_collider(Collider::new_sphere(2.0))
            .with_controller(Box::new(OscillationController::new(50.0, 2.0, 0.5)));
        xwing.transform.position = Vec3::new(0.0, 50.0, 10.0);
        xwing.transform.scale = Vec3::splat(1.0);
        scene.xwing_id = scene.add_object(xwing);

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

        let mut point_lights = Vec::new();
        let colors = [
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.5, 0.0),
        ];

        for i in 0..4 {
            point_lights.push(
                PointLight::new(
                    Vec3::ZERO,
                    LightProperties::new(0.2, 2.5, 3.0, 32.0).with_color(colors[i % 4]),
                )
                .with_attenuation(Attenuation::new(1.0, 0.35, 0.44)),
            );
        }

        let pause_button = Button::new("Pause", 1170.0, 660.0, 100.0, 40.0);

        Self {
            assets,
            scene,
            ui_manager,
            pause_button,
            selected_object_id: None,
            renderer,
            input: Input::new(),
            camera: OrbitCamera::new(),
            light,
            point_lights,
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
            obj.update(current_time, delta_time);
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

        // 1. Game Specific UI: Top Panel
        let w = win_cfg::WIDTH as f32;
        let h = win_cfg::HEIGHT as f32;

        self.ui_manager.text_renderer.render_rect(
            &self.ui_manager.ui_rect_shader,
            10.0,
            h - 60.0,
            180.0,
            50.0,
            glam::Vec4::new(0.0, 0.0, 0.0, ui_cfg::PANEL_OPACITY),
            w,
            h,
        );
        self.ui_manager.text_renderer.render_text(
            "Hakkology",
            20.0,
            h - 55.0,
            32.0,
            Vec3::new(1.0, 1.0, 1.0),
            w,
            h,
        );

        // 2. Game Specific UI: Pause Button
        let mut pause_btn = self.pause_button.clone();
        if self.is_paused {
            pause_btn.text = "Resume".to_string();
        }
        pause_btn.draw(
            &self.ui_manager.text_renderer,
            &self.ui_manager.ui_rect_shader,
            w,
            h,
        );

        // 3. Manager UI (Inspector)
        self.ui_manager.render(&self.scene, self.selected_object_id);
    }

    fn handle_event(&mut self, event: &WindowEvent, time: &mut Time) {
        self.input.handle_event(event);

        if let WindowEvent::MouseButton(glfw::MouseButtonLeft, Action::Press, _) = event {
            let (mx, my) = (self.input.mouse.pos.x, self.input.mouse.pos.y);

            // Pause Button
            if self.pause_button.is_clicked(mx, my, win_cfg::HEIGHT as f32) {
                time.toggle_pause();
                self.is_paused = time.is_paused;
                return;
            }

            // Inspector Interaction
            if self.selected_object_id.is_some() {
                let delta = self
                    .ui_manager
                    .inspector
                    .check_clicks(mx, my, win_cfg::HEIGHT as f32);
                if delta != Vec3::ZERO {
                    if let Some(id) = self.selected_object_id {
                        self.apply_transform_delta(id, delta);
                    }
                    return;
                }
            }

            // Scene Selection (Raycast)
            let ray = self.camera.screen_point_to_ray(
                mx,
                my,
                win_cfg::WIDTH as f32,
                win_cfg::HEIGHT as f32,
            );
            self.selected_object_id = self.cast_ray(&ray);
            self.check_intersection(&ray); // For debug log
        }
    }
}
