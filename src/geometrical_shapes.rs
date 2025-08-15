pub trait Drawable {
    fn draw(&self);
    fn color(&self);
}

pub trait Displayable {
    fn display(&self);
}

pub struct Point(pub i32, pub i32);

pub struct Line<'a>(pub &'a Point, pub &'a Point);

pub struct Triangle<'a>(pub &'a Point, pub &'a Point, pub &'a Point);

pub struct Rectangle<'a>(pub &'a Point, pub &'a Point);

pub struct Circle<'a> {
    pub center: &'a Point,
    pub radius: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }
}

impl<'a> Line<'a> {
    pub fn new(a: &'a Point, b: &'a Point) -> Self {
        Self(a, b)
    }
}

impl<'a> Triangle <'a>{
    pub fn new(a: &'a Point, b: &'a Point, c: &'a Point) -> Self {
        Self(a, b, c)
    }
}

impl<'a> Rectangle <'a>{
    pub fn new(a: &'a Point, b: &'a Point) -> Self {
        Self(a, b)
    }
}

impl<'a> Circle <'a> {
    pub fn new(center: &'a Point, radius: i32) -> Self {
        Self{center, radius}
    }
}