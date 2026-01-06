use std::rc::Rc;
use glfw::{Action, Key, WindowEvent};
use glam::{Mat4, Vec3};

use crate::shapes::{Shape, ShapeEnum, Cube};
use crate::shaders::{Shader, VertexShaderKind, FragmentShaderKind};
use crate::input::Input;
use crate::importer::AssetImporter;


pub struct CubeMode {
    cube: ShapeEnum,
    shader: Rc<Shader>,
    input: Input,
    camera_pos: Vec3,
    camera_front: Vec3,
    camera_up: Vec3,
    yaw: f32,
    pitch: f32,
}

impl CubeMode {
    pub fn new() -> Self {
        let shader = Rc::new(Shader::new(
            VertexShaderKind::ModelViewProjection, 
            FragmentShaderKind::VertexColor // Use Normals (mapped to Attribute 1) as colors for visualization
        ).expect("Failed to create shader program"));

        // Note: Cube vertices map Normals to Attribute 1.
        // VertexShaderKind::ModelViewProjection takes Attribute 1 as 'aColor'.
        // FragmentShaderKind::VertexColor outputs 'ourColor'.
        // Result: A rainbow colored cube based on surface normals.

        // Create a default texture or load one. For now, let's assume we might need one.
        // If we don't have one, we can just not bind anything and see what happens (usually black or garbage).
        // Let's create a 1x1 white texture to be safe?
        // For simplicity, we assume user adds texture later or we just define proper Cube colors.
        // To keep it simple and visual, let's just use "ColorFromUniform" for now if we don't have a texture?
        // But prompt asked for Cube specifically. 
        // Let's stick to texture shader but maybe set a color mix.
        
        let cube = Cube::new(shader.clone());
        
        CubeMode {
            cube: ShapeEnum::Cube(cube),
            shader,
            input: Input::new(),
            camera_pos: Vec3::new(0.0, 0.0, 3.0),
            camera_front: Vec3::new(0.0, 0.0, -1.0),
            camera_up: Vec3::new(0.0, 1.0, 0.0),
            yaw: -90.0,
            pitch: 0.0,
        }
    }
}

impl super::RenderMode for CubeMode {
    fn update(&mut self) {
        // Simple Arcball / Orbit logic
        // If mouse left pressed, delta x/y controls yaw/pitch
        if self.input.is_mouse_button_pressed(glfw::MouseButtonLeft) {
             let sensitivity = 0.1;
             self.yaw += self.input.mouse_delta.x * sensitivity;
             self.pitch -= self.input.mouse_delta.y * sensitivity; // Reversed y-axis
             
             if self.pitch > 89.0 { self.pitch = 89.0; }
             if self.pitch < -89.0 { self.pitch = -89.0; }
        }

        // Calculate Camera Position based on Yaw/Pitch (Orbiting around 0,0,0)
        let front = Vec3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos()
        ).normalize();
        
        // Actually, "orbit" means moving the camera position, but keeping focus on 0,0,0
        // The above calculation is for First Person Camera (looking direction)
        // To Orbit, we set the position on a sphere:
        let radius = 5.0;
        self.camera_pos.x = radius * self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        self.camera_pos.y = radius * self.pitch.to_radians().sin();
        self.camera_pos.z = radius * self.yaw.to_radians().sin() * self.pitch.to_radians().cos();
        
        // Reset deltas after processing
        self.input.reset_delta();
    }

    fn render(&self) {
        self.shader.use_program();
        
        // MVP Matrices
        let projection = Mat4::perspective_rh_gl(45.0f32.to_radians(), 800.0/600.0, 0.1, 100.0);
        let view = Mat4::look_at_rh(self.camera_pos, Vec3::ZERO, self.camera_up);
        let model = Mat4::IDENTITY; // Cube at 0,0,0

        self.shader.set_mat4("projection", &projection.to_cols_array());
        self.shader.set_mat4("view", &view.to_cols_array());
        self.shader.set_mat4("model", &model.to_cols_array());
        
        // Optional: Set default texture unit
        // self.shader.set_int("u_Texture", 0); 
        
        self.cube.draw();
    }

    fn handle_event(&mut self, event: &WindowEvent) {
        self.input.handle_event(event);

        match event {
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
               // Application handles close via window prop, but we can't access it here directly easily
               // except via modifying a shared state or just letting Main loop handle it?
               // The request said "EScape programı kapatsın".
               // In `glfw_window.rs` we already have Escape -> window.set_should_close(true).
               // So that part is already global! We don't need to do it here. 
            }
            _ => {}
        }
    }
}
