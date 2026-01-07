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
    // Create window and OpenGL context
    let mut window = GlWindow::new(win_cfg::TITLE, win_cfg::WIDTH, win_cfg::HEIGHT);
    window.init_gl();

    // Init game state (OpenGL context is ready)
    let mode = Box::new(Game::new());

    // Start application
    let mut app = Application::new(window, mode);
    app.run();
}
