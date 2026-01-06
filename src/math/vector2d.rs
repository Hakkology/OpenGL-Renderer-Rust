use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    // Yeni bir 2D vektör oluşturur
    pub fn new(x: f32, y: f32) -> Self {
        Vector2D { x, y }
    }

    // Sıfır vektörü oluşturur
    pub fn zero() -> Self {
        Vector2D { x: 0.0, y: 0.0 }
    }

    // Vektörün büyüklüğünü hesaplar
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    // Vektörü normalleştirir
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

    // İki vektörün nokta çarpımını hesaplar
    pub fn dot(&self, other: &Vector2D) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

// Vektör toplama işlemi
impl Add for Vector2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Vektör çıkarma işlemi
impl Sub for Vector2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// Vektörü skaler ile çarpma işlemi
impl Mul<f32> for Vector2D {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Vector2D {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

// Vektörü skaler ile bölme işlemi
impl Div<f32> for Vector2D {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        if scalar != 0.0 {
            Vector2D {
                x: self.x / scalar,
                y: self.y / scalar,
            }
        } else {
            panic!("Sıfıra bölme hatası")
        }
    }
}
