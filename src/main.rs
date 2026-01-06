extern crate glfw;
extern crate gl;

mod window;
mod app;
mod shaders;
mod input;
mod shapes;
mod light;
mod math;
mod primitives;
mod game;
mod ui;
mod time;
pub mod importer;

use window::GlWindow;
use app::Application;
use game::Game;

fn main() {
    // 1. Pencereyi ve OpenGL context'ini oluştur
    let mut window = GlWindow::new("OpenGL Modular Renderer", 800, 600);
    window.init_gl();

    // 2. Modu oluştur (Context hazır olduğu için shader/buffer yükleyebilir)
    let mode = Box::new(Game::new());

    // 3. Uygulamayı başlat
    let mut app = Application::new(window, mode);
    app.run();
}
