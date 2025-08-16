use super::*;

pub trait Drawable {
    fn draw(&self);
    fn color(&self);
}

pub trait Displayable {
    fn display(&self, x: i32, y: i32, color: Color);
}

#[derive(Clone, Debug, Copy)]
pub struct Point {
    pub x: i32, 
    pub y: i32
}

pub struct Line(pub Point, pub Point);

pub struct Triangle(pub Point, pub Point, pub Point);

pub struct Rectangle(pub Point, pub Point);

pub struct Circle {
    pub center: Point,
    pub radius: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self{x, y}
    }
}

impl Line {
    pub fn new(a: &Point, b: &Point) -> Self {
        Self(a.clone(), b.clone())
    }
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Self(a.clone(), b.clone(), c.clone())
    }
}

impl Rectangle {
    pub fn new(a: &Point, b: &Point) -> Self {
        Self(a.clone(), b.clone())
    }
}

impl Circle  {
    pub fn new(center: &Point, radius: i32) -> Self {
        Self{center: center.clone(), radius}
    }
}