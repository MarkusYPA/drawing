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
                image.display(x, y.round() as i32, color.clone());
            }
        } else {
            for y in start.y..=end.y {
                let completion = (y as f64 - start.y as f64) / (end.y - start.y) as f64;
                let x = completion * (end.x - start.x) as f64 + start.x as f64;
                image.display(x.round() as i32, y, color.clone());
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
}

impl Rectangle {
    pub fn new(a: &Point, b: &Point) -> Self {
        Self(a.clone(), b.clone())
    }

    pub fn random(max_x: i32, max_y: i32) -> Self {
        Rectangle::new(&Point::random(max_x, max_y), &Point::random(max_x, max_y))
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
        let random_color = self.color();

        Line::new(&self.0, &self.1).draw_with_color(image, random_color.clone());
        Line::new(&self.1, &self.2).draw_with_color(image, random_color.clone());
        Line::new(&self.2, &self.0).draw_with_color(image, random_color.clone());
    }
}

impl Drawable for Rectangle {
    fn draw<I: Displayable>(&self, image: &mut I) {
        let random_color = self.color();

        let a = &self.0;
        let b = &Point::new(self.0.x, self.1.y);
        let c = &self.1;
        let d = &Point::new(self.1.x, self.0.y);

        Line::new(a, b).draw_with_color(image, random_color.clone());
        Line::new(b, c).draw_with_color(image, random_color.clone());
        Line::new(c, d).draw_with_color(image, random_color.clone());
        Line::new(d, a).draw_with_color(image, random_color.clone());
    }
}

impl Drawable for Circle {
    fn draw<I: Displayable>(&self, image: &mut I) {
        let random_color = self.color();
        let steps = (self.radius as f64 / (2.0 as f64).sqrt()) as i32;

        for s in 0..=steps {
            let offset_1 = ((self.radius.pow(2) - s.pow(2)) as f64).sqrt().round() as i32;
            let offset_2 = offset_1 * -1;

            // top and bottom quarters of circle
            let x = s + self.center.x;
            image.display(x, offset_1 + self.center.y, random_color.clone());
            image.display(x, offset_2 + self.center.y, random_color.clone());
            image.display(x - s * 2, offset_1 + self.center.y, random_color.clone());
            image.display(x - s * 2, offset_2 + self.center.y, random_color.clone());

            // left and right quarters of circle
            let y = s + self.center.y;
            image.display(offset_1 + self.center.x, y, random_color.clone());
            image.display(offset_2 + self.center.x, y, random_color.clone());
            image.display(offset_1 + self.center.x, y - s * 2, random_color.clone());
            image.display(offset_2 + self.center.x, y - s * 2, random_color.clone());
        }
    }
}
