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

fn main() {
    let wall = Wall::new(0.0, 0.0, 10.0, 0.0);
    println!("{}", wall);

    loop {
        println!("Example output for user to see for now: ");
        let n = parse_input().trim().parse::<u32>().unwrap();
        println!("{}", n);
    }
}

}
