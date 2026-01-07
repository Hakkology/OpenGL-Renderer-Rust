pub mod window {
    pub const WIDTH: u32 = 1280;
    pub const HEIGHT: u32 = 720;
    pub const TITLE: &str = "OpenGL Renderer - Rust";
    pub const VSYNC: bool = true;
}

pub mod camera {
    use glam::Vec3;

    /// Initial camera position
    pub const INITIAL_POSITION: Vec3 = Vec3::new(0.0, 0.0, 15.0);

    /// Initial camera target (what the camera looks at)
    pub const INITIAL_TARGET: Vec3 = Vec3::ZERO;

    /// Initial yaw angle in degrees
    pub const INITIAL_YAW: f32 = -90.0;

    /// Initial pitch angle in degrees  
    pub const INITIAL_PITCH: f32 = 15.0;

    /// Initial distance from target
    pub const INITIAL_DISTANCE: f32 = 15.0;

    /// Minimum zoom distance
    pub const MIN_DISTANCE: f32 = 3.0;

    /// Maximum zoom distance
    pub const MAX_DISTANCE: f32 = 50.0;

    /// Mouse rotation sensitivity
    pub const SENSITIVITY: f32 = 50.0;

    /// Scroll wheel zoom speed
    pub const ZOOM_SPEED: f32 = 1.0;

    /// Base movement speed (WASD)
    pub const MOVE_SPEED: f32 = 5.0;

    /// Speed multiplier when holding Shift
    pub const SPRINT_MULTIPLIER: f32 = 2.0;

    /// Field of view in degrees
    pub const FOV: f32 = 45.0;

    /// Near clipping plane
    pub const NEAR_PLANE: f32 = 0.1;

    /// Far clipping plane
    pub const FAR_PLANE: f32 = 3000.0;
}

pub mod rendering {
    /// Shadow map resolution
    pub const SHADOW_MAP_SIZE: u32 = 2048;

    /// Point shadow map resolution (per face)
    pub const POINT_SHADOW_SIZE: u32 = 512;

    /// Far plane for point light shadows
    pub const SHADOW_FAR_PLANE: f32 = 25.0;

    /// Number of point lights supported
    pub const MAX_POINT_LIGHTS: usize = 5;

    /// Number of spot lights supported
    pub const MAX_SPOT_LIGHTS: usize = 4;
}

pub mod ui {
    /// Font size for text rendering
    pub const FONT_SIZE: f32 = 16.0;

    /// UI panel opacity (0.0 - 1.0)
    pub const PANEL_OPACITY: f32 = 0.5;
}
