extern crate gl;
use gl::types::*;
use std::ptr;
use std::ffi::CString;

use super::part::{ShaderPart, ShaderType};
use super::library::{VertexShaderKind, FragmentShaderKind};

pub struct Program {
    pub id: GLuint,
}

impl Program {
    // Create from program parts directly
    pub fn from_parts(vertex: &ShaderPart, fragment: &ShaderPart) -> Result<Program, String> {
        let program_id;
        unsafe {
            program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vertex.id);
            gl::AttachShader(program_id, fragment.id);
            gl::LinkProgram(program_id);

            let mut success = gl::FALSE as GLint;
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                let mut len = 0;
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
                let error = CString::from_vec_unchecked(vec![b' '; len as usize]);
                gl::GetProgramInfoLog(program_id, len, ptr::null_mut(), error.as_ptr() as *mut GLchar);
                return Err(error.to_string_lossy().into_owned());
            }
        }
        Ok(Program { id: program_id })
    }

    // High level constructor using Enums
    pub fn new(vertex_kind: VertexShaderKind, fragment_kind: FragmentShaderKind) -> Result<Program, String> {
        let vs_source = vertex_kind.get_source();
        let fs_source = fragment_kind.get_source();

        let vs = ShaderPart::from_source(&vs_source, ShaderType::Vertex)?;
        let fs = ShaderPart::from_source(&fs_source, ShaderType::Fragment)?;

        Self::from_parts(&vs, &fs)
    }
    
    // Legacy support wrapper (optional, but requested to match old signature if needed, though we are refactoring)
    // We'll skip legacy signature and update callers.
    
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    // Uniform setters
    pub fn get_uniform_location(&self, name: &str) -> i32 {
        let c_name = CString::new(name).unwrap();
        unsafe { gl::GetUniformLocation(self.id, c_name.as_ptr()) }
    }

    pub fn set_bool(&self, name: &str, value: bool) {
        unsafe {
            gl::Uniform1i(self.get_uniform_location(name), value as i32);
        }
    }

    pub fn set_int(&self, name: &str, value: i32) {
        unsafe {
            gl::Uniform1i(self.get_uniform_location(name), value);
        }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        unsafe {
            gl::Uniform1f(self.get_uniform_location(name), value);
        }
    }

    pub fn set_vec4(&self, name: &str, x: f32, y: f32, z: f32, w: f32) {
        unsafe {
            gl::Uniform4f(self.get_uniform_location(name), x, y, z, w);
        }
    }

    pub fn set_mat4(&self, name: &str, mat: &[f32; 16]) {
        unsafe {
            gl::UniformMatrix4fv(self.get_uniform_location(name), 1, gl::FALSE, mat.as_ptr());
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
