pub mod triangle;
pub mod rectangle;
pub mod circle;
#[allow(unused_imports)]
pub use crate::math::Vector2D;
pub use triangle::Triangle;
pub use rectangle::Rectangle;
pub use circle::Circle;

pub trait Shape {
    fn init(&mut self);
    fn draw(&self);
}

pub enum ShapeEnum {
    Triangle(Triangle),
    Rectangle(Rectangle),
    Circle(Circle),
}

impl Shape for ShapeEnum {
    fn init(&mut self) {
        match self {
            ShapeEnum::Triangle(s) => s.init(),
            ShapeEnum::Rectangle(s) => s.init(),
            ShapeEnum::Circle(s) => s.init(),
        }
    }

    fn draw(&self) {
        match self {
            ShapeEnum::Triangle(s) => s.draw(),
            ShapeEnum::Rectangle(s) => s.draw(),
            ShapeEnum::Circle(s) => s.draw(),
        }
    }
}
