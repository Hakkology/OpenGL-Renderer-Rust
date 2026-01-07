use crate::scene::transform::Transform;
use glam::{Quat, Vec3};

pub trait Controller {
    fn update(&self, transform: &mut Transform, current_time: f32, delta_time: f32);
}

/// A simple rotation controller
pub struct RotationController {
    pub axis: Vec3,
    pub speed: f32,
}

impl RotationController {
    pub fn new(axis: Vec3, speed: f32) -> Self {
        Self { axis, speed }
    }
}

impl Controller for RotationController {
    fn update(&self, transform: &mut Transform, current_time: f32, _delta_time: f32) {
        transform.rotation = Quat::from_axis_angle(self.axis, current_time * self.speed);
    }
}

/// A controller for circular orbit
pub struct OrbitController {
    pub center: Vec3,
    pub radius: f32,
    pub speed: f32,
    pub offset: f32,
}

impl OrbitController {
    pub fn new(center: Vec3, radius: f32, speed: f32, offset: f32) -> Self {
        Self {
            center,
            radius,
            speed,
            offset,
        }
    }
}

impl Controller for OrbitController {
    fn update(&self, transform: &mut Transform, current_time: f32, _delta_time: f32) {
        let angle = current_time * self.speed + self.offset;
        transform.position.x = self.center.x + angle.cos() * self.radius;
        transform.position.z = self.center.z + angle.sin() * self.radius;
    }
}

/// A controller for oscillation (Y-axis)
pub struct OscillationController {
    pub base_y: f32,
    pub amplitude: f32,
    pub speed: f32,
}

impl OscillationController {
    pub fn new(base_y: f32, amplitude: f32, speed: f32) -> Self {
        Self {
            base_y,
            amplitude,
            speed,
        }
    }
}

impl Controller for OscillationController {
    fn update(&self, transform: &mut Transform, current_time: f32, _delta_time: f32) {
        transform.position.y = self.base_y + (current_time * self.speed).sin() * self.amplitude;
    }
}

/// A more complex floating/orbiting controller for capsules
pub struct FloatingController {
    pub speed: f32,
    pub offset: f32,
}

impl FloatingController {
    pub fn new(speed: f32, offset: f32) -> Self {
        Self { speed, offset }
    }
}

impl Controller for FloatingController {
    fn update(&self, transform: &mut Transform, current_time: f32, _delta_time: f32) {
        let tilt = 45.0f32.to_radians();
        let tilt_quat = Quat::from_rotation_z(tilt);
        let angle = current_time * self.speed + self.offset;
        let orbit_pos = Vec3::new(angle.cos() * 4.0, 0.0, angle.sin() * 4.0);

        transform.position = tilt_quat.mul_vec3(orbit_pos);
        transform.rotation = Quat::from_rotation_y(current_time) * Quat::from_rotation_x(tilt);
    }
}
