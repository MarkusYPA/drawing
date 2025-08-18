use super::*;
use geometrical_shapes as gs;
use gs::{Point, Line};

use rand::random_range;
use std::mem::swap;

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
        let height = get_cube_height(&diag_1, &diag_2, len_d1);

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
        for l in lines {
            l.draw_with_color(image, random_color.clone());
        }
    }
}

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
