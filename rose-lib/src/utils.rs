use serde::{Deserialize, Serialize};

pub type Quaternion = Vector4<f32>;

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Color4 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color4 {
    pub fn new() -> Color4 {
        Self::default()
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl Vector2<f32> {
    pub fn new() -> Vector2<f32> {
        Self::default()
    }
}

impl Vector2<i32> {
    pub fn new() -> Vector2<i32> {
        Self::default()
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl Vector3<f32> {
    pub fn new() -> Vector3<f32> {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Vector3<i16> {
    pub fn new() -> Vector3<i16> {
        Vector3 { x: 0, y: 0, z: 0 }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Vector4<T> {
    pub w: T,
    pub x: T,
    pub y: T,
    pub z: T,
}

impl Vector4<f32> {
    pub fn new() -> Vector4<f32> {
        Vector4 {
            w: 0.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Vector4<i16> {
    pub fn new() -> Vector4<i16> {
        Vector4 {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct BoundingBox<T> {
    pub min: Vector3<T>,
    pub max: Vector3<T>,
}
