pub mod program;
pub mod part;
pub mod library;

pub use program::Program;
pub use program::Program as Shader; // Alias for backward compatibility
pub use part::{ShaderPart, ShaderType};
pub use library::{VertexShaderKind, FragmentShaderKind};
