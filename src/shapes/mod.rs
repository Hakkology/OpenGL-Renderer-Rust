pub trait Shape {
    fn init(&mut self);
    fn draw(&self);
}

pub mod triangle;
pub mod rectangle;
pub mod circle;
pub mod cube;
pub mod vector2d;

pub use vector2d::Vector2D;
pub use triangle::Triangle;
pub use rectangle::Rectangle;
pub use circle::Circle;
pub use cube::Cube;

// Enum for generic handling
pub enum ShapeEnum {
    Triangle(Triangle),
    Rectangle(Rectangle),
    Circle(Circle),
    Cube(Cube),
}

impl Shape for ShapeEnum {
    fn init(&mut self) {
        match self {
            ShapeEnum::Triangle(s) => s.init(),
            ShapeEnum::Rectangle(s) => s.init(),
            ShapeEnum::Circle(s) => s.init(),
            ShapeEnum::Cube(s) => s.init(),
        }
    }

    fn draw(&self) {
        match self {
            ShapeEnum::Triangle(s) => s.draw(),
            ShapeEnum::Rectangle(s) => s.draw(),
            ShapeEnum::Circle(s) => s.draw(),
            ShapeEnum::Cube(s) => s.draw(),
        }
    }
}
