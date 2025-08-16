use super::*;
use rand::random_range;

// enum for colors (white, red, purple, ...)?
// to pick them randomly with random range?
// Red: Color(255, 0 ,0 , 255),

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Line(pub Point, pub Point);

#[derive(Debug)]
pub struct Triangle(pub Point, pub Point, pub Point);

#[derive(Debug)]
pub struct Rectangle(pub Point, pub Point);

#[derive(Debug)]
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
        //Self(*a, *b)  // alternative way
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
        Circle::new(
            &Point::random(max_x, max_y),
            random_range(0..=(max_x + max_y) / 3),
        )
    }
}

//  ======= Drawable =======

impl Drawable for Point {
    fn color(&self) -> Color {
        Color::white()
    }

    fn draw(&self, image: &mut Image) {
        // draw one pixel
        image.display(self.x, self.y, self.color());
    }
}

impl Drawable for Line {
    fn color(&self) -> Color {
        Color::white()
    }

    fn draw(&self, image: &mut Image) {
        let x_length = (self.1.x - self.0.x).abs();
        let y_length = (self.1.y - self.0.y).abs();
        let is_steep = y_length > x_length;

        // Always go from smaller to bigger value
        let (start, end) = if is_steep {
            if self.0.y > self.1.y {
                (self.1, self.0)
            } else {
                (self.0, self.1)
            }
        } else {
            if self.0.x > self.1.x {
                (self.1, self.0)
            } else {
                (self.0, self.1)
            }
        };

        if !is_steep {
            for x in start.x..=end.x {
                let completion = (x as f64 - start.x as f64) / x_length as f64;
                let y = completion * (end.y - start.y) as f64 + start.y as f64;
                let y = y as i32;
                image.display(x, y, self.color());
            }
        } else {
            for y in self.0.y..=self.1.y {
                let completion = (y as f64 - start.y as f64) / y_length as f64;
                let x = completion * (end.x - start.x) as f64 + start.x as f64;
                let x = x as i32;
                image.display(x, y, self.color());
            }
        }
    }
}

impl Drawable for Triangle {
    fn color(&self) -> Color {
        Color::white()
    }

    fn draw(&self, image: &mut Image) {
        // draw all lines between points
    }
}

impl Drawable for Rectangle {
    fn color(&self) -> Color {
        Color::white()
    }

    fn draw(&self, image: &mut Image) {
        // work out all four points and draw lines in between
    }
}

impl Drawable for Circle {
    fn color(&self) -> Color {
        Color::white()
    }

    fn draw(&self, image: &mut Image) {
        // draw points according to pythagoras
    }
}
