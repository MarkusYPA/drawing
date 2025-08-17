use std::mem::swap;

use super::*;
use rand::random_range;

//  ======= Traits =======

pub trait Drawable {
    fn draw<I: Displayable>(&self, image: &mut I);

    fn color(&self) -> Color {
        Color {
            r: random_range(0..=255),
            g: random_range(0..=255),
            b: random_range(0..=255),
            a: 255,
        }
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

#[derive(Debug)]
pub struct Cube {
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub d: Point,
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

impl Cube {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        let d = Point::new(a.x + (c.x - b.x), a.y + (c.y - b.y));
        Self {
            a: a.clone(),
            b: b.clone(),
            c: c.clone(),
            d,
        }
    }

    pub fn random(max_x: i32, max_y: i32) -> Self {
        Cube::new(
            &Point::random(max_x, max_y),
            &Point::random(max_x, max_y),
            &Point::random(max_x, max_y),
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
            let x1 = self.center.x + s;
            let x2 = self.center.x - s;
            image.display(x1, self.center.y + offset_1, random_color.clone());
            image.display(x1, self.center.y + offset_2, random_color.clone());
            image.display(x2, self.center.y + offset_1, random_color.clone());
            image.display(x2, self.center.y + offset_2, random_color.clone());

            // left and right quarters of circle
            let y1 = self.center.y + s;
            let y2 = self.center.y - s;
            image.display(self.center.x + offset_1, y1, random_color.clone());
            image.display(self.center.x + offset_2, y1, random_color.clone());
            image.display(self.center.x + offset_1, y2, random_color.clone());
            image.display(self.center.x + offset_2, y2, random_color.clone());
        }
    }
}

impl Drawable for Cube {
    fn draw<I: Displayable>(&self, image: &mut I) {
        let random_color = self.color();

        // Diagonals, assign longer one to diag_1
        let mut diag_1 = Line::new(&self.a, &self.c);
        let mut diag_2 = Line::new(&self.b, &self.d);
        let len_d1 =
            (((diag_1.0.x - diag_1.1.x).pow(2) + (diag_1.0.y - diag_1.1.y).pow(2)) as f64).sqrt();
        let len_d2 =
            (((diag_2.0.x - diag_2.1.x).pow(2) + (diag_2.0.y - diag_2.1.y).pow(2)) as f64).sqrt();
        if len_d2 > len_d1 {
            swap(&mut diag_1, &mut diag_2);
        }

        // Midpoint of initial parallelogram
        let m = Point::new(
            ((self.a.x as f64 + self.c.x as f64) / 2.0).round() as i32,
            ((self.a.y as f64 + self.c.y as f64) / 2.0).round() as i32,
        );

        // Angles between diagonals, assign bigger one to angle_1
        let mut angle_1 = angle_from_points(&diag_1.0, &m, &diag_2.0); // does order matter?
        let mut angle_2 = angle_from_points(&diag_2.0, &m, &diag_1.1);
        if angle_2 > angle_1 {
            swap(&mut angle_1, &mut angle_2);
        }

        // Angle from diag_1 and length for height lines
        let angle_3 = (angle_1 + angle_2) / 2.0;
        let height = get_cube_height(angle_1, angle_2, len_d1, len_d2);

        // Lines from existing corners
        let a1b1 = Line::new(&self.a, &self.b);
        let b1c1 = Line::new(&self.b, &self.c);
        let c1d1 = Line::new(&self.c, &self.d);
        let d1a1 = Line::new(&self.d, &self.a);

        // Lines from angle and height
        let a1a2 = new_line_from_point(&self.a, &diag_1, angle_3, height);
        let b1b2 = new_line_from_point(&self.b, &diag_1, angle_3, height);
        let c1c2 = new_line_from_point(&self.c, &diag_1, angle_3, height);
        let d1d2 = new_line_from_point(&self.d, &diag_1, angle_3, height);

        let lines = vec![a1b1, b1c1, c1d1, d1a1, a1a2, b1b2, c1c2, d1d2];
        for l in lines {
            l.draw_with_color(image, random_color.clone());
        }

    }
}

// Make these Cube methods
fn angle_from_points(a: &Point, b: &Point, c: &Point) -> f64 {
    // Vectors ba and bc
    let bax = (a.x - b.x) as f64;
    let bay = (a.y - b.y) as f64;
    let bcx = (c.x - b.x) as f64;
    let bcy = (c.y - b.y) as f64;

    // Dot product and magnitudes
    let dot = bax * bcx + bay * bcy;
    let mag_ba = (bax * bax + bay * bay).sqrt();
    let mag_bc = (bcx * bcx + bcy * bcy).sqrt();

    if mag_ba == 0.0 || mag_bc == 0.0 {
        return 0.0; // Degenerate case: a==b or c==b
    }

    // Clamp the cosine to [-1, 1] to avoid NaN from floating-point rounding
    let cos_theta = (dot / (mag_ba * mag_bc)).clamp(-1.0, 1.0);

    cos_theta.acos() // radians in [0, PI]
}

fn get_cube_height(a1: f64, a2: f64, ld1: f64, ld2: f64) -> f64 {
    let rot_1 = a1 / a2; // 1 = looking at edge, 0 = looking at plane
    let base_h = rot_1 * (ld1 / (2.0 as f64).sqrt()) + (1.0 - rot_1) * ld1; // full or partial long diagonal 
    let rot_2 = ld2 / ld1; // 1 = looking from top, 0 = looking from side
    (1.0 - rot_2) * base_h
}

fn new_line_from_point(p: &Point, l: &Line, a: f64, h: f64) -> Line {
    // direction of reference line
    let dx = (l.1.x - l.0.x) as f64;
    let dy = (l.1.y - l.0.y) as f64;

    // normalize
    let len = (dx * dx + dy * dy).sqrt();
    if len == 0.0 {
        panic!("Reference line has zero length");
    }
    let ux = dx / len;
    let uy = dy / len;

    // rotate by angle a
    let rx = ux * a.cos() - uy * a.sin();
    let ry = ux * a.sin() + uy * a.cos();

    // scale by h and add to point p
    let qx = p.x as f64 + h * rx;
    let qy = p.y as f64 + h * ry;

    let q = Point {
        x: qx.round() as i32,
        y: qy.round() as i32,
    };

    Line(p.clone(), q)
}
