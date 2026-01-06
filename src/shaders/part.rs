extern crate gl;
use gl::types::*;
use std::ffi::CString;
use std::ptr;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone, Copy)]
pub enum ShaderType {
    Vertex = gl::VERTEX_SHADER as isize,
    Fragment = gl::FRAGMENT_SHADER as isize,
}

pub struct ShaderPart {
    pub id: GLuint,
    pub kind: ShaderType,
}

impl ShaderPart {
    pub fn from_source(source: &str, kind: ShaderType) -> Result<ShaderPart, String> {
        let shader_id;
        unsafe {
            shader_id = gl::CreateShader(kind as GLenum);
            let c_str = CString::new(source.as_bytes()).unwrap();
            gl::ShaderSource(shader_id, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader_id);

            let mut success = gl::FALSE as GLint;
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                let mut len = 0;
                gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut len);
                let error = CString::from_vec_unchecked(vec![b' '; len as usize]);
                gl::GetShaderInfoLog(shader_id, len, ptr::null_mut(), error.as_ptr() as *mut GLchar);
                return Err(error.to_string_lossy().into_owned());
            }
        }
        Ok(ShaderPart { id: shader_id, kind })
    }

    pub fn from_file(path: &str, kind: ShaderType) -> Result<ShaderPart, String> {
        let mut file = File::open(path).map_err(|e| format!("Unable to open file {}: {}", path, e))?;
        let mut source = String::new();
        file.read_to_string(&mut source).map_err(|e| format!("Unable to read file {}: {}", path, e))?;
        Self::from_source(&source, kind)
    }
}

impl Drop for ShaderPart {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}
