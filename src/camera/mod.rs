use glam::{Mat4, Vec3};
use crate::input::Input;

pub struct OrbitCamera {
    pub position: Vec3,
    pub target: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    pub sensitivity: f32,
    pub zoom_speed: f32,
}

impl OrbitCamera {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 7.0),
            target: Vec3::ZERO,
            yaw: -90.0,
            pitch: 0.0,
            distance: 7.0,
            min_distance: 2.0,
            max_distance: 20.0,
            sensitivity: 50.0,
            zoom_speed: 0.5,
        }
    }

    pub fn update(&mut self, input: &Input, delta_time: f32) {
        // Rotation with left mouse button
        if input.is_mouse_button_pressed(glfw::MouseButtonLeft) {
            self.yaw += input.mouse.delta.x * self.sensitivity * delta_time;
            self.pitch -= input.mouse.delta.y * self.sensitivity * delta_time;
            
            self.pitch = self.pitch.clamp(-89.0, 89.0);
        }

        // Zoom with scroll
        self.distance -= input.mouse.scroll_y * self.zoom_speed;
        self.distance = self.distance.clamp(self.min_distance, self.max_distance);

        // Calculate forward vector (view direction stored in Spherical Coords)
        let yaw_rad = self.yaw.to_radians();
        let pitch_rad = self.pitch.to_radians();
        
        let forward = Vec3::new(
            yaw_rad.cos() * pitch_rad.cos(),
            pitch_rad.sin(),
            yaw_rad.sin() * pitch_rad.cos()
        ).normalize();
        
        let right = forward.cross(Vec3::Y).normalize();

        // WASD Movement (Moving the target/center of orbit)
        // W/S moves forward/back relative to view
        let move_speed = 5.0 * delta_time; // Adjustable speed
        
        // Project forward to XZ plane for "FPS like" movement or standard free cam
        // User requested: "w up, A left" -> typical WASD
        // Usually W moves "forward" in view direction.
        // For an orbit camera, moving "forward" usually means moving the pivot closer/further or moving the pivot in the scene.
        // Let's implement panning the pivot point on XZ plane relative to camera view.
        
        let flat_forward = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
        let flat_right = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

        if input.is_key_pressed(glfw::Key::W) {
             self.target += flat_forward * move_speed;
        }
        if input.is_key_pressed(glfw::Key::S) {
             self.target -= flat_forward * move_speed;
        }
        if input.is_key_pressed(glfw::Key::A) {
             self.target -= flat_right * move_speed;
        }
        if input.is_key_pressed(glfw::Key::D) {
             self.target += flat_right * move_speed;
        }
        // Optional: E/Q for Up/Down? Or just keep it planar.
        // Let's keep it simplest for now.

        self.position.x = self.distance * yaw_rad.cos() * pitch_rad.cos();
        self.position.y = self.distance * pitch_rad.sin();
        self.position.z = self.distance * yaw_rad.sin() * pitch_rad.cos();
        
        self.position += self.target;
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, Vec3::Y)
    }

    pub fn projection_matrix(&self, aspect_ratio: f32) -> Mat4 {
        Mat4::perspective_rh_gl(45.0f32.to_radians(), aspect_ratio, 0.1, 100.0)
    }

    /// Returns view matrix without translation (for skybox)
    pub fn skybox_view_matrix(&self) -> Mat4 {
        let view = self.view_matrix();
        Mat4::from_mat3(glam::Mat3::from_mat4(view))
    }
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self::new()
    }
}
