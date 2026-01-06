use glfw::Context;
use crate::window::GlWindow;
use crate::game::RenderMode;
use crate::time::Time;

pub struct Application {
    window: GlWindow,
    mode: Box<dyn RenderMode>,
    time: Time,
}

impl Application {
    // Uygulama oluşturucu. Window ve Mode dışarıdan enjekte edilir.
    pub fn new(window: GlWindow, mode: Box<dyn RenderMode>) -> Application {
        Application { window, mode, time: Time::new() }
    }

    // Modu değiştirmek istersek
    #[allow(dead_code)]
    pub fn set_mode(&mut self, mode: Box<dyn RenderMode>) {
        self.mode = mode;
    }

    // Ana uygulama döngüsü
    pub fn run(&mut self) {
        while !self.window.window.should_close() {
            // Event polling
            self.window.glfw.poll_events();

            // Time update
            let current_time = self.window.glfw.get_time();
            self.time.update(current_time);

            // Ekran temizleme
            self.window.clear(0.2, 0.3, 0.3, 1.0);

            // Modun güncelleme ve çizim fonksiyonlarını çağır
            self.mode.update(&self.time);
            self.mode.render();

            // Buffer swap
            self.window.window.swap_buffers();

            // Event handling
            let events: Vec<(f64, glfw::WindowEvent)> =
                glfw::flush_messages(&self.window.events).collect();
            
            for (_, event) in events {
                // Global window eventleri (örn. ESC ile çıkış)
                self.window.handle_event(&event);

                // Mod'a özgü eventler
                self.mode.handle_event(&event, &mut self.time);
            }
        }
    }
}
