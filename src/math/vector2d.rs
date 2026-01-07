#![allow(dead_code)]
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

/// For vector2 ops
impl Vector2D {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2D { x, y }
    }

    pub fn zero() -> Self {
        Vector2D { x: 0.0, y: 0.0 }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag != 0.0 {
            Vector2D {
                x: self.x / mag,
                y: self.y / mag,
            }
        } else {
            *self
        }
    }

    pub fn dot(&self, other: &Vector2D) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

/// Sum
impl Add for Vector2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// Subtraction
impl Sub for Vector2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// Multiplication
impl Mul<f32> for Vector2D {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Vector2D {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

/// Division
impl Div<f32> for Vector2D {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        if scalar != 0.0 {
            Vector2D {
                x: self.x / scalar,
                y: self.y / scalar,
            }
        } else {
            panic!("Division by zero")
        }
    }
}
