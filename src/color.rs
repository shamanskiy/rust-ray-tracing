use std::ops::{Add, Div};

use cgmath::Vector3;
use image::Rgba;

pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}
impl Color {
    pub const WHITE: Color = Color {
        red: 1.,
        green: 1.,
        blue: 1.,
    };
    pub const LIGHT_BLUE: Color = Color {
        red: 0.5,
        green: 0.7,
        blue: 1.,
    };
    pub const BLACK: Color = Color {
        red: 0.,
        green: 0.,
        blue: 0.,
    };

    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }

    pub fn from_vector(vec3: Vector3<f32>) -> Self {
        Self {
            red: vec3.x,
            green: vec3.y,
            blue: vec3.z,
        }
    }

    pub fn to_rgba(&self) -> Rgba<u8> {
        let red = (self.red * 255.) as u8;
        let green = (self.green * 255.) as u8;
        let blue = (self.blue * 255.) as u8;
        return Rgba([red, green, blue, 255]);
    }

    pub fn blend(color1: Color, color2: Color, blend_ratio: f32) -> Self {
        let red = lerp(color1.red, color2.red, blend_ratio);
        let green = lerp(color1.green, color2.green, blend_ratio);
        let blue = lerp(color1.blue, color2.blue, blend_ratio);
        Self { red, green, blue }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        return Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        };
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Self::Output {
        return Color {
            red: self.red / rhs,
            green: self.green / rhs,
            blue: self.blue / rhs,
        };
    }
}

fn lerp(x: f32, y: f32, alpha: f32) -> f32 {
    return x * (1. - alpha) + y * alpha;
}
