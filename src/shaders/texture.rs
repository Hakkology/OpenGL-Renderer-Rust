extern crate gl;
use gl::types::*;
use std::ffi::c_void;

#[derive(Debug, Clone)]
pub struct Texture {
    pub id: GLuint,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn new(width: u32, height: u32, data: &[u8], format: GLenum) -> Texture {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            // Byte hizalamasını 1 yapalım (özellikle 1 kanallı metin textureları için kritik)
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

            // Texture parametreleri
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as i32,
                width as i32,
                height as i32,
                0,
                format,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const c_void,
            );
        }

        Texture { id, width, height }
    }

    pub fn from_file(path: &str) -> Result<Texture, String> {
        println!("Loading texture from: {}", path);
        let img = image::open(path).map_err(|e| e.to_string())?;
        let img = img.flipv(); // OpenGL expects (0,0) at bottom-left
        let width = img.width();
        let height = img.height();
        println!("Texture loaded: {}x{} pixels", width, height);
        
        let (data, format) = match img.color() {
            image::ColorType::Rgb8 => (img.to_rgb8().into_raw(), gl::RGB),
            image::ColorType::Rgba8 => (img.to_rgba8().into_raw(), gl::RGBA),
            _ => {
                // Default to RGB if possible
                (img.to_rgb8().into_raw(), gl::RGB)
            }
        };

        Ok(Texture::new(width, height, &data, format))
    }

    pub fn bind(&self, unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

#[derive(Debug, Clone)]
pub struct CubeMap {
    pub id: GLuint,
}

impl CubeMap {
    pub fn from_files(paths: Vec<&str>) -> Result<CubeMap, String> {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, id);

            for (i, path) in paths.iter().enumerate() {
                let img = image::open(path).map_err(|e| e.to_string())?;
                // Cubemaps usually don't need vertical flip if formatted correctly for OpenGL
                // but sometimes they do. Let's keep it standard.
                let width = img.width();
                let height = img.height();
                
                let (data, format) = match img.color() {
                    image::ColorType::Rgb8 => (img.to_rgb8().into_raw(), gl::RGB),
                    image::ColorType::Rgba8 => (img.to_rgba8().into_raw(), gl::RGBA),
                    _ => (img.to_rgb8().into_raw(), gl::RGB),
                };

                gl::TexImage2D(
                    gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
                    0,
                    format as i32,
                    width as i32,
                    height as i32,
                    0,
                    format,
                    gl::UNSIGNED_BYTE,
                    data.as_ptr() as *const c_void,
                );
            }

            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
        }

        Ok(CubeMap { id })
    }

    /// Load cubemap from a single horizontal cross layout image (4x3 grid)
    /// Layout: [empty, top, empty, empty]
    ///         [left, front, right, back]
    ///         [empty, bottom, empty, empty]
    pub fn from_cross_file(path: &str) -> Result<CubeMap, String> {
        let img = image::open(path).map_err(|e| e.to_string())?;
        let img = img.to_rgba8();
        let width = img.width();
        let height = img.height();
        
        let face_width = width / 4;
        let face_height = height / 3;

        // Face positions in the cross layout (x, y in face units)
        // Order: +X, -X, +Y, -Y, +Z, -Z
        let face_positions = [
            (2, 1), // +X (right)
            (0, 1), // -X (left)
            (1, 0), // +Y (top)
            (1, 2), // -Y (bottom)
            (1, 1), // +Z (front)
            (3, 1), // -Z (back)
        ];

        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, id);

            for (i, (fx, fy)) in face_positions.iter().enumerate() {
                let x = fx * face_width;
                let y = fy * face_height;
                
                let mut face_data = Vec::with_capacity((face_width * face_height * 4) as usize);
                for row in 0..face_height {
                    for col in 0..face_width {
                        let pixel = img.get_pixel(x + col, y + row);
                        face_data.extend_from_slice(&pixel.0);
                    }
                }

                gl::TexImage2D(
                    gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
                    0,
                    gl::RGBA as i32,
                    face_width as i32,
                    face_height as i32,
                    0,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    face_data.as_ptr() as *const c_void,
                );
            }

            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
        }

        Ok(CubeMap { id })
    }

    pub fn bind(&self, unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.id);
        }
    }
}

impl Drop for CubeMap {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
