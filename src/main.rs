use std::fmt;

use ultraviolet::DVec2;

struct Field {
    walls: [Wall; 4],
}

// impl Field {
//     pub fn new(width: u32, height: u32) => Field {
//         let wall0 =
//     }
// }

struct Ball {
    position: DVec2,
    velocity: DVec2,
}

impl Ball {
    fn new(position: DVec2, velocity: DVec2) -> Self {
        Self { position, velocity }
    }
}

struct Wall {
    point_a: DVec2,
    point_b: DVec2,
}

trait RoundToPlace {
    fn round_to(&self, place: u64) -> f64;
}

impl RoundToPlace for f64 {
    fn round_to(&self, place: u64) -> f64 {
        (self * place as f64).round() / place as f64
    }
}

trait Validity {
    fn is_valid(&self) -> bool;
}

impl Validity for DVec2 {
    fn is_valid(&self) -> bool {
        const INVALID_DVEC2: DVec2 = DVec2 {
            x: f64::MAX,
            y: f64::MAX,
        };

        !INVALID_DVEC2.eq(self)
    }
}

// Wall consists of two points, a point a and a point b.
// Should we define a wall as a point, a length, and an orientation?

impl Wall {
    pub fn new(x: f64, y: f64, length: f64, orientation: f64) -> Wall {
        // Calculate point a and b of the wall by taking the origin,
        // the length, and the orientation.
        let len = length / 2.0;

        let a_x = orientation.sin().round_to(1000) * len;
        let a_y = orientation.cos().round_to(1000) * len;

        let opposite_angle: f64 =
            (orientation + std::f64::consts::PI) % (2.0 * std::f64::consts::PI);

        let b_x = opposite_angle.sin().round_to(1000) * len;
        let b_y = opposite_angle.cos().round_to(1000) * len;

        let point_a: DVec2 = DVec2::new(a_x + x, a_y + y);
        let point_b: DVec2 = DVec2::new(b_x + x, b_y + y);

        Wall { point_a, point_b }
    }
}

impl fmt::Display for Wall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.point_a, self.point_b)
    }
}

fn parse_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed");
    input
}

fn perpendicular(vector: DVec2) -> DVec2 {
    DVec2 {
        x: -vector.y,
        y: vector.x,
    }
}

fn line_intersection(pt0a: DVec2, pt0b: DVec2, pt1a: DVec2, pt1b: DVec2) -> DVec2 {
    let delta_pt_0: DVec2 = pt0b - pt0a;
    let delta_pt_1: DVec2 = pt1b - pt1a;
    let delta_pt_a: DVec2 = pt0a - pt1a;

    let perpendicular_delta_pt_0: DVec2 = perpendicular(delta_pt_0);
    let denominator = perpendicular_delta_pt_0.dot(delta_pt_1);
    let numerator = perpendicular_delta_pt_0.dot(delta_pt_a);

    if denominator == 0.0 || numerator == 0.0 {
        return DVec2 {
            x: f64::MAX,
            y: f64::MAX,
        };
    }

    ((numerator / denominator) * delta_pt_1) + pt1a
}

fn main() {
    let wall0: Wall = Wall::new(0.0, 0.0, 6.0, 0.0);
    let wall1: Wall = Wall::new(0.0, 0.0, 6.0, 90.0);
    println!("{}", wall0);
    println!("{}", wall1);

    loop {
        println!("Example output for user to see for now: ");
        let n = parse_input().trim().parse::<u32>().unwrap();
        println!("{}", n);
    }
}

#[cfg(test)]
mod tests {
    use crate::Validity;
    use crate::{line_intersection, Wall};
    use ultraviolet::DVec2;

    #[test]
    fn test_line_intersection_success() {
        let pt0a: DVec2 = DVec2::new(0.0, 0.0);
        let pt0b: DVec2 = DVec2::new(6.0, 6.0);
        let pt1a: DVec2 = DVec2::new(0.0, 6.0);
        let pt1b: DVec2 = DVec2::new(6.0, 0.0);

        let intersection_pt = line_intersection(pt0a, pt0b, pt1a, pt1b);
        assert_eq!(intersection_pt, DVec2 { x: 3.0, y: 3.0 })
    }

    #[test]
    fn test_line_intersection_failure() {
        let pt0a: DVec2 = DVec2::new(0.0, 0.0);
        let pt0b: DVec2 = DVec2::new(6.0, 6.0);
        let pt1a: DVec2 = DVec2::new(-6.0, -6.0);
        let pt1b: DVec2 = DVec2::new(-6.0, -10.0);

        let intersection_pt = line_intersection(pt0a, pt0b, pt1a, pt1b);
        assert!(!intersection_pt.is_valid());
    }

    #[test]
    fn create_wall_from_point_length_and_orientation() {
        // Horizontal Line
        {
            let wall: Wall = Wall::new(0.0, 0.0, 1.0, 0.0);
            assert_eq!(wall.point_a, DVec2 { x: 0., y: 0.5 });
            assert_eq!(wall.point_b, DVec2 { x: 0., y: -0.5 });
        }

        // Vertical Line
        {
            let wall: Wall = Wall::new(0.0, 0.0, 1.0, std::f64::consts::PI / 2.0);
            assert_eq!(wall.point_a, DVec2 { x: 0.5, y: 0.0 });
            assert_eq!(wall.point_b, DVec2 { x: -0.5, y: 0.0 });
        }
    }
}
