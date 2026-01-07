extern crate gl;
extern crate glfw;

mod app;
mod assets;
mod camera;
mod config;
mod game;
pub mod importer;
mod input;
mod light;
mod logic;
mod math;
mod primitives;
mod renderer;
mod scene;
mod shaders;
mod shadow;
mod shapes;
mod time;
mod ui;
mod window;

use app::Application;
use config::window as win_cfg;
use game::Game;
use window::GlWindow;

fn main() {
    // 1. Pencereyi ve OpenGL context'ini oluştur
    let mut window = GlWindow::new(win_cfg::TITLE, win_cfg::WIDTH, win_cfg::HEIGHT);
    window.init_gl();

    // 2. Modu oluştur (Context hazır olduğu için shader/buffer yükleyebilir)
    let mode = Box::new(Game::new());

    // 3. Uygulamayı başlat
    let mut app = Application::new(window, mode);
    app.run();
}
