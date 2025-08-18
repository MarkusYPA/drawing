use std::f32::consts::PI;

use super::*;
use geometrical_shapes::{Line, Point};

use rand::random_range;

pub struct Polygon {
    pub start: Point,
    pub size: i32,
    pub corners: i32,
}

impl Polygon {
    fn new(start: &Point, size: i32, corners: i32) -> Self {
        Polygon {
            start: start.clone(),
            size,
            corners,
        }
    }

    pub fn random(max_x: i32, max_y: i32) -> Self {
        //let corners: i32 = random_range(5..=9);
        let corners = 5;
        let size = random_range(30..=(1500 / corners));

        Polygon::new(
            &Point::new(random_range(0..=max_x), random_range(0..=max_y)),
            size,
            corners,
        )
    }
}

impl Drawable for Polygon {
    fn draw<I: Displayable>(&self, image: &mut I) {
        let random_color = self.color();

        // Equal angle step for each corner
        let step_angle = 2.0 * PI / self.corners as f32;

        // Random initial orientation (0..2π)
        let start_angle = (random_range(0..360) as f32).to_radians();

        // Generate all vertices, starting with self.start
        let mut points = Vec::new();
        points.push(self.start);

        for i in 1..self.corners {
            let angle = start_angle + (i - 1) as f32 * step_angle;
            let prev = points[(i - 1) as usize];

            let x = prev.x as f32 + self.size as f32 * angle.cos();
            let y = prev.y as f32 + self.size as f32 * angle.sin();

            points.push(Point {
                x: x.round() as i32,
                y: y.round() as i32,
            });
        }

        // Draw polygon edges
        for i in 0..self.corners {
            let p1 = points[i as usize];
            let p2 = points[((i + 1) % self.corners) as usize]; // wrap last to first
            Line::new(&p1, &p2).draw_with_color(image, random_color.clone());
        }
    }
}
