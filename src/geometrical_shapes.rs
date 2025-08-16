use super::*;
use rand::random_range;

fn random_color() -> Color {
    Color {
        r: random_range(0..=255),
        g: random_range(0..=255),
        b: random_range(0..=255),
        a: 255,
    }
}
//  ======= Traits =======

pub trait Drawable {
    fn draw<I: Displayable>(&self, image: &mut I);

    fn color(&self) -> Color {
        random_color()
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

//  ======= Structs =======

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

//  ======= Methods =======

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

    fn draw_with_color<I: Displayable>(&self, image: &mut I, color: Color) {
        let x_length = self.1.x - self.0.x;
        let y_length = self.1.y - self.0.y;
        let is_steep = y_length.abs() > x_length.abs();

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
                let completion = (x as f64 - start.x as f64) / (end.x - start.x) as f64;
                let y = completion * (end.y - start.y) as f64 + start.y as f64;
                image.display(x, y as i32, color.clone());
            }
        } else {
            for y in start.y..=end.y {
                let completion = (y as f64 - start.y as f64) / (end.y - start.y) as f64;
                let x = completion * (end.x - start.x) as f64 + start.x as f64;
                image.display(x as i32, y, color.clone());
            }
        }
    }
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Self(a.clone(), b.clone(), c.clone())
    }

    pub fn random(max_x: i32, max_y: i32) -> Self {
        Triangle::new(
            &Point::random(max_x, max_y),
            &Point::random(max_x, max_y),
            &Point::random(max_x, max_y),
        )
    }

    fn draw_with_color<I: Displayable>(&self, image: &mut I, color: Color) {
        Line::new(&self.0, &self.1).draw_with_color(image, color.clone());
        Line::new(&self.1, &self.2).draw_with_color(image, color.clone());
        Line::new(&self.2, &self.0).draw_with_color(image, color.clone());
    }
}

impl Rectangle {
    pub fn new(a: &Point, b: &Point) -> Self {
        Self(a.clone(), b.clone())
    }

    pub fn random(max_x: i32, max_y: i32) -> Self {
        Rectangle::new(&Point::random(max_x, max_y), &Point::random(max_x, max_y))
    }

    fn draw_with_color<I: Displayable>(&self, image: &mut I, color: Color) {
        let a = &self.0;
        let b = &Point::new(self.0.x, self.1.y);
        let c = &self.1;
        let d = &Point::new(self.1.x, self.0.y);

        Line::new(a, b).draw_with_color(image, color.clone());
        Line::new(b, c).draw_with_color(image, color.clone());
        Line::new(c, d).draw_with_color(image, color.clone());
        Line::new(d, a).draw_with_color(image, color.clone());
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
            random_range(40..=(max_x + max_y) / 3),
        )
    }
}

//  ======= Implement Drawable =======

impl Drawable for Point {
    fn draw<I: Displayable>(&self, image: &mut I) {
        image.display(self.x, self.y, self.color());
    }
}

impl Drawable for Line {
    fn draw<I: Displayable>(&self, image: &mut I) {
        self.draw_with_color(image, self.color());
    }
}

impl Drawable for Triangle {
    fn draw<I: Displayable>(&self, image: &mut I) {
        let color = self.color();
        self.draw_with_color(image, color);
    }
}

impl Drawable for Rectangle {
    fn draw<I: Displayable>(&self, image: &mut I) {
        let color = self.color();
        self.draw_with_color(image, color);
    }
}

impl Drawable for Circle {
    fn draw<I: Displayable>(&self, image: &mut I) {
        let color = self.color();

        // draw upper and lower quarters of circle
        let (start_x, end_x) = (
            (self.center.x as f64 - self.radius as f64 / (2.0 as f64).sqrt()) as i32,
            (self.center.x as f64 + self.radius as f64 / (2.0 as f64).sqrt()) as i32,
        );
        for x in start_x..=end_x {
            let x_now = x - self.center.x;
            let y1 = ((self.radius.pow(2) - x_now.pow(2)) as f64).sqrt();
            let y2 = y1 * -1.0;

            image.display(x, y1 as i32 + self.center.y, color.clone());
            image.display(x, y2 as i32 + self.center.y, color.clone());
        }

        // draw left and right quarters of circle
        let (start_y, end_y) = (
            (self.center.y as f64 - self.radius as f64 / (2.0 as f64).sqrt()) as i32,
            (self.center.y as f64 + self.radius as f64 / (2.0 as f64).sqrt()) as i32,
        );
        for y in start_y..=end_y {
            let y_now = y - self.center.y;
            let x1 = ((self.radius.pow(2) - y_now.pow(2)) as f64).sqrt();
            let x2 = x1 * -1.0;

            image.display(x1 as i32 + self.center.x, y, color.clone());
            image.display(x2 as i32 + self.center.x, y, color.clone());
        }
    }
}
