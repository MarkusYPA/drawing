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
    pub a1: Point,
    pub b1: Point,
    pub c1: Point,
    pub d1: Point,
    pub a2: Point,
    pub b2: Point,
    pub c2: Point,
    pub d2: Point,
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
        // one more point to complete parallelogram
        let d = &Point::new(a.x + (c.x - b.x), a.y + (c.y - b.y));

        // Diagonals, assign longer one to diag_1
        let mut diag_1 = Line::new(a, c);
        let mut diag_2 = Line::new(b, d);
        let mut len_d1 =
            (((diag_1.0.x - diag_1.1.x).pow(2) + (diag_1.0.y - diag_1.1.y).pow(2)) as f64).sqrt();
        let mut len_d2 =
            (((diag_2.0.x - diag_2.1.x).pow(2) + (diag_2.0.y - diag_2.1.y).pow(2)) as f64).sqrt();
        if len_d2 > len_d1 {
            swap(&mut diag_1, &mut diag_2);
            swap(&mut len_d1, &mut len_d2);
        }

        // Midpoint of initial parallelogram
        let m = Point::new(
            ((a.x as f64 + c.x as f64) / 2.0).round() as i32,
            ((a.y as f64 + c.y as f64) / 2.0).round() as i32,
        );

        // Angles between diagonals, assign bigger one to angle_1
        let mut angle_1 = angle_from_points(&diag_1.0, &m, &diag_2.0); // does order matter?
        let mut angle_2 = angle_from_points(&diag_2.0, &m, &diag_1.1);
        if angle_2 > angle_1 {
            swap(&mut angle_1, &mut angle_2);
        }

        // Angle from diag_1 and length for height lines
        let angle_3 = (angle_1 + angle_2) / 2.0;
        let height = get_cube_height(&diag_1, &diag_2, len_d1);
        let offset = point_from_angle_and_distance(&diag_1, angle_3, height);

        let (a2, b2, c2, d2) = (
            Point::new(a.x + offset.x, a.y + offset.y),
            Point::new(b.x + offset.x, b.y + offset.y),
            Point::new(c.x + offset.x, c.y + offset.y),
            Point::new(d.x + offset.x, d.y + offset.y),
        );

        Self {
            a1: a.clone(),
            b1: b.clone(),
            c1: c.clone(),
            d1: d.clone(),
            a2: a2.clone(),
            b2: b2.clone(),
            c2: c2.clone(),
            d2: d2.clone(),
        }
    }

    pub fn random(max_x: i32, max_y: i32) -> Self {
        let min_x = max_x * 1 / 4;
        let min_y = max_y * 1 / 4;
        let max_x = max_x * 3 / 4;
        let max_y = max_y * 3 / 4;

        Cube::new(
            &Point::new(random_range(min_x..=max_x), random_range(min_y..=max_y)),
            &Point::new(random_range(min_x..=max_x), random_range(min_y..=max_y)),
            &Point::new(random_range(min_x..=max_x), random_range(min_y..=max_y)),
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

        // Lines between corners
        let a1b1 = Line::new(&self.a1, &self.b1);
        let b1c1 = Line::new(&self.b1, &self.c1);
        let c1d1 = Line::new(&self.c1, &self.d1);
        let d1a1 = Line::new(&self.d1, &self.a1);

        let a1a2 = Line::new(&self.a1, &self.a2);
        let b1b2 = Line::new(&self.b1, &self.b2);
        let c1c2 = Line::new(&self.c1, &self.c2);
        let d1d2 = Line::new(&self.d1, &self.d2);

        let a2b2 = Line::new(&self.a2, &self.b2);
        let b2c2 = Line::new(&self.b2, &self.c2);
        let c2d2 = Line::new(&self.c2, &self.d2);
        let d2a2 = Line::new(&self.d2, &self.a2);

        let lines = vec![
            a1b1, b1c1, c1d1, d1a1, a1a2, b1b2, c1c2, d1d2, a2b2, b2c2, c2d2, d2a2,
        ];
        for (i, l) in lines.iter().enumerate() {
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

// Point method
fn point_from_angle_and_distance(l: &Line, a: f64, h: f64) -> Point {
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

    // rotate by angle a and scale by h
    let x = (ux * a.cos() - uy * a.sin()) * h;
    let y = (ux * a.sin() + uy * a.cos()) * h;

    Point {
        x: x as i32,
        y: y as i32,
    }
}

fn get_cube_height(d1: &Line, d2: &Line, len_d1: f64) -> f64 {
    let s = project_point_onto_line(d1, &d2.0);

    let len_as = (((d1.0.x - s.x).pow(2) + (d1.0.y - s.y).pow(2)) as f64).sqrt();
    let len_bs = (((d2.0.x - s.x).pow(2) + (d2.0.y - s.y).pow(2)) as f64).sqrt();
    let rot_1 = len_as / len_d1; // 0-1, 0.5 = looking at edge
    let rot_1 = (rot_1 - 0.5).abs() * 2.0; // 0-1, 0 = looking at edge
    let rot_2 = len_bs * 2.0 / len_d1; // 0-1, 1 = looking at top

    let base_h = (1.0 - rot_1) * (len_d1 / (2.0 as f64).sqrt()) + rot_1 * len_d1;
    (1.0 - rot_2) * base_h
}

fn project_point_onto_line(l: &Line, p: &Point) -> Point {
    let acx = l.1.x as f64 - l.0.x as f64;
    let acy = l.1.y as f64 - l.0.y as f64;

    let abx = p.x as f64 - l.0.x as f64;
    let aby = p.y as f64 - l.0.y as f64;

    let ac_len2 = acx * acx + acy * acy;
    if ac_len2 == 0.0 {
        // Degenerate line (a == c)
        return l.0;
    }

    // Projection scalar
    let t = (abx * acx + aby * acy) / ac_len2;

    // Projected point
    Point {
        x: l.0.x + (t * acx).round() as i32,
        y: l.0.y + (t * acy).round() as i32,
    }
}
