use glfw::{fail_on_errors, Action, Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent, WindowMode};

pub struct GlWindow {
    pub glfw: Glfw,
    pub window: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,
}

impl GlWindow {
    // Yeni bir GlWindow oluşturur
    pub fn new(title: &str, width: u32, height: u32) -> GlWindow {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap_or_else(|e| {
            panic!("Failed to initialize GLFW: {:?}", e);
        });

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Compat));

        let (mut window, events) = glfw
            .create_window(width, height, title, WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.make_current();
        window.set_key_polling(true);

        GlWindow { glfw, window, events }
    }

    // OpenGL'i başlatır
    pub fn init_gl(&mut self) {
        gl::load_with(|symbol| self.window.get_proc_address(symbol) as *const _);
        
        let version = unsafe {
            let version_cstr = std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8);
            version_cstr.to_str().unwrap()
        };
        println!("OpenGL version: {}", version);
        
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    
            let (width, height) = self.window.get_size();
            gl::Viewport(0, 0, width, height);
        }
    }

    // Ekranı temizler
    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
    
    // Pencere olaylarını işler
    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                self.window.set_should_close(true);
            }
            _ => {}
        }
    }
}
