use std::rc::Rc;
use glfw::{WindowEvent, Action, Key};

use crate::shapes::{Shape, ShapeEnum, Triangle, Rectangle, Circle, Vector2D};
use crate::shaders::{Shader, VertexShaderKind, FragmentShaderKind};

pub struct BasicMode {
    shapes: Vec<ShapeEnum>,
}

impl BasicMode {
    pub fn new() -> Self {
        let mut shapes: Vec<ShapeEnum> = Vec::new();
        
        // Example: Using the new modular Enum system
        // Create a shader that accepts a uniform color
        let shader = Rc::new(Shader::new(
            VertexShaderKind::Standard, 
            FragmentShaderKind::ColorFromUniform
        ).expect("Failed to create shader program"));

        // Set a default color (e.g., Orange)
        shader.use_program();
        shader.set_vec4("u_Color", 1.0, 0.5, 0.2, 1.0);

        // Triangle
        let triangle = Triangle::new(
            shader.clone(),
            Vector2D::new(-0.6, 0.5),
            Vector2D::new(-0.4, 0.5),
            Vector2D::new(-0.5, 0.7)
        );
        shapes.push(ShapeEnum::Triangle(triangle));

        // Rectangle
        // We could create another shader instance for different color if we wanted
        // But for now they share the same 'orange' shader instance
        let rect = Rectangle::new(
            shader.clone(),
            Vector2D::new(0.0, 0.5),
            0.4, 
            0.4
        );
        shapes.push(ShapeEnum::Rectangle(rect));

        // Circle
        let circle = Circle::new(
            shader.clone(),
            Vector2D::new(0.5, -0.5),
            0.2,
            32
        );
        shapes.push(ShapeEnum::Circle(circle));

        BasicMode { shapes }
    }
}

impl super::RenderMode for BasicMode {
    fn update(&mut self) {
        // Animasyon mantığı buraya
    }

    fn render(&self) {
        for shape in &self.shapes {
            // ShapeEnum implements Shape, so we can call draw directly
            shape.draw();
        }
    }

    fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::Key(Key::Space, _, Action::Press, _) => {
                println!("Space key detected in BasicMode.");
            }
            _ => {}
        }
    }
}
