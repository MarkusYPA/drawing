use super::*;
use rand::random_range;

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self);
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
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
        Self { x, y }
    }

    pub fn random(max_x: i32, max_y: i32) -> Self {
        Point::new(random_range(0..=max_x), random_range(0..=max_y))
    }
}

impl Line {
    pub fn new(a: &Point, b: &Point) -> Self {
        Self(a.clone(), b.clone())
    }

    pub fn random(max_x: i32, max_y: i32) -> Self {
        Line::new(&Point::random(max_x, max_y), &Point::random(max_x, max_y))
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

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        Self {
            center: center.clone(),
            radius,
        }
    }

    pub fn random(max_x: i32, max_y: i32) -> Self {
        Circle::new(&Point::random(max_x, max_y), random_range(0..=(max_x + max_y) / 3))
    }
}

impl Drawable for Point {
    fn color(&self) {
        
    }

    fn draw(&self, image: &mut Image) {
        
    }
}

impl Drawable for Line {
    fn color(&self) {
        
    }

    fn draw(&self, image: &mut Image) {
        
    }
}

impl Drawable for Triangle {
    fn color(&self) {
        
    }

    fn draw(&self, image: &mut Image) {
        
    }
}

impl Drawable for Rectangle {
    fn color(&self) {
        
    }

    fn draw(&self, image: &mut Image) {
        
    }
}

impl Drawable for Circle {
    fn color(&self) {
        
    }

    fn draw(&self, image: &mut Image) {
        
    }
}

