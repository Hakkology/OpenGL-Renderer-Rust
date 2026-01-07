extern crate gl;
use crate::shaders::Shader;
use gl::types::*;
use glam::{Mat4, Vec3};

pub struct ShadowMap {
    pub fbo: GLuint,
    pub depth_texture: GLuint,
    pub width: u32,
    pub height: u32,
    pub shader: Shader,
}

impl ShadowMap {
    pub fn new(width: u32, height: u32) -> Self {
        let shader = Shader::from_files(
            "assets/shaders/shadow_depth.vert",
            "assets/shaders/shadow_depth.frag",
        )
        .expect("Failed to create shadow depth shader");

        let mut fbo = 0;
        let mut depth_texture = 0;

        unsafe {
            // Create framebuffer
            gl::GenFramebuffers(1, &mut fbo);

            // Create depth texture
            gl::GenTextures(1, &mut depth_texture);
            gl::BindTexture(gl::TEXTURE_2D, depth_texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::DEPTH_COMPONENT as i32,
                width as i32,
                height as i32,
                0,
                gl::DEPTH_COMPONENT,
                gl::FLOAT,
                std::ptr::null(),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_BORDER as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_BORDER as i32,
            );
            let border_color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
            gl::TexParameterfv(
                gl::TEXTURE_2D,
                gl::TEXTURE_BORDER_COLOR,
                border_color.as_ptr(),
            );

            // Attach depth texture to framebuffer
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::DEPTH_ATTACHMENT,
                gl::TEXTURE_2D,
                depth_texture,
                0,
            );
            gl::DrawBuffer(gl::NONE);
            gl::ReadBuffer(gl::NONE);
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        ShadowMap {
            fbo,
            depth_texture,
            width,
            height,
            shader,
        }
    }

    /// Calculate light space matrix for directional light
    pub fn light_space_matrix(
        &self,
        light_dir: Vec3,
        scene_center: Vec3,
        scene_radius: f32,
    ) -> Mat4 {
        let light_pos = scene_center - light_dir.normalize() * scene_radius * 2.0;
        let light_view = Mat4::look_at_rh(light_pos, scene_center, Vec3::Y);
        let light_projection = Mat4::orthographic_rh_gl(
            -scene_radius,
            scene_radius,
            -scene_radius,
            scene_radius,
            0.1,
            scene_radius * 4.0,
        );
        light_projection * light_view
    }

    /// Begin shadow pass - render to depth buffer
    pub fn begin_pass(&self) {
        unsafe {
            gl::Viewport(0, 0, self.width as i32, self.height as i32);
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
            gl::Clear(gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::FRONT); // Prevent shadow acne by rendering back faces for shadows
        }
        self.shader.use_program();
    }

    /// End shadow pass - restore default framebuffer
    pub fn end_pass(&self, screen_width: u32, screen_height: u32) {
        unsafe {
            gl::Disable(gl::CULL_FACE); // Return to default (double-sided) as expected by the user's scene
            gl::CullFace(gl::BACK);
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::Viewport(0, 0, screen_width as i32, screen_height as i32);
        }
    }

    /// Bind shadow map texture for sampling in lit shaders
    pub fn bind_shadow_map(&self, unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, self.depth_texture);
        }
    }

    /// Set light space matrix in shader
    pub fn set_light_space_matrix(&self, matrix: &Mat4) {
        self.shader
            .set_mat4("lightSpaceMatrix", &matrix.to_cols_array());
    }

    /// Set model matrix in depth shader
    pub fn set_model(&self, model: &Mat4) {
        self.shader.set_mat4("model", &model.to_cols_array());
    }
}

impl Drop for ShadowMap {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.fbo);
            gl::DeleteTextures(1, &self.depth_texture);
        }
    }
}
