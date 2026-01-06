use glfw::WindowEvent;

pub trait RenderMode {
    /// Her karede (frame) çağrılan güncelleme mantığı
    fn update(&mut self);

    /// Çizim komutlarını içeren fonksyon
    fn render(&self);

    /// Input ve pencere olaylarını işlemek için
    fn handle_event(&mut self, event: &WindowEvent);
}

pub mod basic;
pub mod cube;
