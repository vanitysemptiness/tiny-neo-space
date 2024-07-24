use macroquad::prelude::*;


pub struct Stroke {
    pub points: Vec<Vec2>,
    pub fickness: f32,
    pub color: Color
}

impl Stroke {
    pub fn new(points: Vec<Vec2>, fickness: f32, color: Color) -> Self {
        Stroke {
            points: points,
            fickness: fickness,
            color: color,
        }
    }
}