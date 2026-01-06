extern crate glfw;
extern crate gl;

mod window;
mod app;
mod shaders;
mod texture;
mod input;
mod shapes;
mod modes;
pub mod importer;

use window::GlWindow;
use app::Application;
use modes::cube::CubeMode;

fn main() {
    // 1. Pencereyi ve OpenGL context'ini oluştur
    let mut window = GlWindow::new("OpenGL Modular Renderer", 800, 600);
    window.init_gl();

    // 2. Modu oluştur (Context hazır olduğu için shader/buffer yükleyebilir)
    let mode = Box::new(CubeMode::new());

    // 3. Uygulamayı başlat
    let mut app = Application::new(window, mode);
    app.run();
}
