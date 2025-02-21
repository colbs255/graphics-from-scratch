mod image;

use image::{Color, Image};

fn main() {
    let mut img = Image::new(1000, 1000, Color::WHITE);
    triangle(
        &mut img,
        Color::GREEN,
        NDC::new(0.25, 0.25).unwrap(),
        NDC::new(0.25, 0.75).unwrap(),
        NDC::new(0.75, 0.5).unwrap(),
    );
    triangle(
        &mut img,
        Color::RED,
        NDC::new(0.25, 0.25).unwrap(),
        NDC::new(0.25, 0.75).unwrap(),
        NDC::new(0.5, 0.5).unwrap(),
    );
    circle(&mut img, Color::BLUE, NDC::new(0.5, 0.5).unwrap(), 30.0);

    let _ = img.save_to_png("output.png");
}

#[derive(Debug, Clone, Copy)]
struct NDC {
    x: f32,
    y: f32,
}

impl NDC {
    pub fn new(x: f32, y: f32) -> Option<NDC> {
        if x >= 0.0 && x <= 1.0 && y >= 0.0 && y <= 1.0 {
            Some(NDC { x, y })
        } else {
            None
        }
    }

    pub fn dist(&self, other: NDC) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}

struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    fn distance(&self, other: Vec2) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx.powi(2) + dy.powi(2)).sqrt() // Calculate the Euclidean distance
    }
}

fn circle(img: &mut Image, color: Color, center: NDC, radius: f32) {
    let width = img.width() as f32;
    let height = img.height() as f32;
    let x_min = (center.x * width - radius).floor() as usize;
    let x_max = (center.x * width + radius).floor() as usize;
    let y_min = (center.y * height - radius).floor() as usize;
    let y_max = (center.y * height + radius).floor() as usize;

    let center_vec = Vec2 {
        x: center.x * width,
        y: center.y * height,
    };
    for x in x_min..=x_max {
        for y in y_min..=y_max {
            let p = Vec2 { x: x as f32, y: y as f32 };
            if center_vec.distance(p) < radius {
                img.set_pixel(x, y, color);
            }
        }
    }
}

fn triangle(img: &mut Image, color: Color, a: NDC, b: NDC, c: NDC) {
    let mut x_min = f32::MAX;
    let mut x_max = f32::MIN;
    let mut y_min = f32::MAX;
    let mut y_max = f32::MIN;

    for point in vec![a, b, c] {
        if point.x < x_min {
            x_min = point.x;
        }
        if point.x > x_max {
            x_max = point.x;
        }
        if point.y < y_min {
            y_min = point.y;
        }
        if point.y > y_max {
            y_max = point.y;
        }
    }
    let width = img.width() as f32;
    let height = img.height() as f32;

    let x_min = (x_min * width).floor() as u32;
    let x_max = (x_max * width).floor() as u32;
    let y_min = (y_min * height).floor() as u32;
    let y_max = (y_max * height).floor() as u32;

    for x in x_min..=x_max {
        for y in y_min..=y_max {
            let p = NDC {
                x: (x as f32) / width,
                y: (y as f32) / height,
            };
            if is_point_in_triangle(p, a, b, c) {
                img.set_pixel(x as usize, y as usize, color);
            }
        }
    }
}

fn is_point_in_triangle(p: NDC, a: NDC, b: NDC, c: NDC) -> bool {
    fn area(a: NDC, b: NDC, c: NDC) -> f32 {
        (a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y)).abs() / 2.0
    }
    // Calculate the area of the full triangle ABC
    let area_abc = area(a, b, c);

    // Calculate the area of the sub-triangle PBC
    let area_pbc = area(p, b, c);

    // Calculate the area of the sub-triangle PCA
    let area_pca = area(p, c, a);

    // Calculate the area of the sub-triangle PAB
    let area_pab = area(p, a, b);

    let sum = area_pbc + area_pca + area_pab;

    // If the sum of the areas of the sub-triangles is equal to the area of the triangle ABC,
    // the point is inside or on the triangle.
    (area_abc - sum).abs() < 1e-6
}
