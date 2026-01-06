pub mod cube;
pub mod sphere;
pub mod capsule;
pub mod skybox;

pub use cube::Cube;
pub use sphere::Sphere;
pub use capsule::Capsule;
pub use skybox::Skybox;

pub trait Primitive {
    fn draw(&self);
}
