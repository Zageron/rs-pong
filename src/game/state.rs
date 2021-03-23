use ultraviolet::DVec2;

pub use super::types::{Ball, GameRules, GameState, PlayerId, Wall};

impl GameState {
    pub fn new() -> Self {
        Self {
            ball: Ball::new(DVec2 { x: 0.0, y: 0.0 }, DVec2 { x: 0.5, y: 0.7 }, 5),
            player0_score: 0,
            player1_score: 0,
        }
    }

    pub fn tick(&mut self) {
        self.ball.tick += 1;
    }

    pub fn increment_score(&mut self, scorer: PlayerId) {
        match scorer {
            PlayerId::PlayerOne => self.player0_score += 1,
            PlayerId::PlayerTwo => self.player1_score += 1,
            PlayerId::None => {}
        }
    }
}

impl GameRules {
    pub fn has_winner(&self, state: &GameState) -> PlayerId {
        if state.player0_score >= self.score_to_win {
            return PlayerId::PlayerOne;
        } else if state.player1_score >= self.score_to_win {
            return PlayerId::PlayerTwo;
        } else {
            return PlayerId::None;
        }
    }
}

trait Interpolation {
    fn get_destination_position(position: &'static DVec2, angle: &'static f32) -> DVec2
    where
        Self: Sized;

    fn get_destination_normal(position: &'static DVec2, angle: &'static f32) -> DVec2
    where
        Self: Sized;
}

impl Interpolation for Ball {
    fn get_destination_position(position: &'static DVec2, angle: &'static f32) -> DVec2
    where
        Self: Sized,
    {
        DVec2::new(0.0, 0.0)
    }

    fn get_destination_normal(position: &'static DVec2, angle: &'static f32) -> DVec2
    where
        Self: Sized,
    {
        DVec2::new(0.0, 0.0)
    }
}

impl Ball {
    pub fn new(position: DVec2, end_position: DVec2, rate: u8) -> Self {
        Self {
            start_position: position,
            end_position,
            tick: 0,
            rate,
        }
    }

    pub fn direction(&self) -> DVec2 {
        (self.end_position - self.start_position).normalized()
    }

    pub fn is_moving(&self) -> bool {
        self.tick < self.rate
    }
}

impl Wall {
    pub fn normal(&self) -> DVec2 {
        perpendicular(self.point_b - self.point_a)
    }
}

trait RoundToPlace {
    fn round_to(&self, place: u64) -> f64;
}

impl RoundToPlace for f64 {
    fn round_to(&self, place: u64) -> f64 {
        (self * place as f64).round() / place as f64
    }
}

trait Invalid {}

impl Invalid for DVec2 {}

trait Validity {
    const INVALID: DVec2;
    fn is_valid(&self) -> bool;
}

impl Validity for DVec2 {
    const INVALID: DVec2 = DVec2 {
        x: f64::MAX,
        y: f64::MAX,
    };

    fn is_valid(&self) -> bool {
        !DVec2::INVALID.eq(self)
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

    let bounds = pt0a.x <= pt1b.x && pt0b.x >= pt1a.x && pt0a.y <= pt1a.y && pt0b.y >= pt1b.y;

    let perpendicular_delta_pt_0: DVec2 = perpendicular(delta_pt_0);
    let denominator = perpendicular_delta_pt_0.dot(delta_pt_1);
    let numerator = perpendicular_delta_pt_0.dot(delta_pt_a);

    if denominator == 0.0 || numerator == 0.0 || !bounds {
        return DVec2::INVALID;
    }

    ((numerator / denominator) * delta_pt_1) + pt1a
}

fn tick(game_state: &mut GameState) {
    let ball: &Ball = &game_state.ball;

    if ball.is_moving() {
        // Test for interactions with the walls.
        // Mirror the direction of the ball if an interaction is found.
    }

    game_state.tick();
}

fn reflect_vector(direction: DVec2, normal: DVec2) -> DVec2 {
    -2.0 * direction.dot(normal) * normal + direction
}

fn reflect_off_of_wall(ball: &mut Ball, wall: Wall) -> DVec2 {
    let direction = ball.direction();
    let wall_normal = wall.normal();
    reflect_vector(direction, wall_normal)
}

#[cfg(test)]
mod tests {
    use super::{line_intersection, reflect_vector, Validity};
    use crate::Wall;
    use ultraviolet::{DRotor2, DVec2};

    #[test]
    fn rotate_ball_trajectory() {
        let ball_direction: f64 = (45.0_f64).to_radians();
        let ball_rotor: DRotor2 = DRotor2::from_angle(ball_direction);

        let mut ball_trajectory: DVec2 = DVec2::unit_x();
        ball_trajectory.rotate_by(ball_rotor);

        let expect_1: f64 = ball_trajectory.x / ball_trajectory.y; // 1 means it's 45 degrees.
        assert!((expect_1 - 1.0).abs() < 1e-10);
    }

    #[test]
    fn four_corners_get_destination() {
        // Given 4 verticies
        let corners: [DVec2; 4] = [
            DVec2::new(-1.0, -1.0),
            DVec2::new(-1.0, 1.0),
            DVec2::new(1.0, 1.0),
            DVec2::new(1.0, -1.0),
        ];

        let walls: &[usize] = &[0, 1, 2, 3, 0];

        // Given 1 ball position vertex
        let ball_position: DVec2 = DVec2::new(0.0, 0.0);

        // Given 1 ball direction angle
        let ball_direction: f64 = 30.0_f64.to_radians();

        // Given the maximum length of a line
        let max_line_length: f64 = 2.0;

        // Create the direction vector from the ball angle.
        let ball_rotor: DRotor2 = DRotor2::from_angle(ball_direction);
        let mut ball_trajectory: DVec2 = DVec2::unit_x();
        ball_trajectory.rotate_by(ball_rotor);

        // Calculate the line from the given ball position vertex and ball direction angle.
        // Start position
        let ball_vector: DVec2 = ball_trajectory.normalized() * max_line_length;

        let mut end_point: DVec2 = DVec2::INVALID;
        for first in &walls[0..walls.len() - 1] {
            let wall_pt0: DVec2 = corners[*first];
            let wall_pt1: DVec2 = corners[(first + 1) % corners.len()];
            end_point = line_intersection(ball_position, ball_vector, wall_pt0, wall_pt1);
            if end_point.is_valid() {
                break;
            }
        }

        // Find the end position of the line.
        assert_eq!(
            end_point,
            DVec2 {
                x: 1.0,
                y: 0.5773502691896257
            }
        );
    }

    // So we have 4 verts representing the walls of the field.
    // These four points, in sets of two, make up 4 walls.
    // The "ball", is not simulated using physics so actually "colliding" with a wall is not necessary.

    // The "reflection" or "granting a ball a movement" occurs like this:
    // - Ball enters a "I am moving now" state.
    // This begins with a direction and a velocity.
    // An interpolation must start here, but before we can interpolate we should guarantee that ther is a reflection.
    // 1) The ball should always be bouncing at a some angle. The angle shall not be 0 or 90.
    //  It probably makes sense to make this angle clamped at an angle like 30 degrees, or to clamp it between
    //  30 and 45.
    // 2) The ball should determine which normal it will be reflecting at at this calculation time.
    // 3) At the end of the interpolation step, a new calculation is made. (Go back to step 1.)

    #[test]
    fn test_reflect_off_of_wall() {
        let direction_45_degrees: DVec2 = DVec2::new(1.0, 1.0);
        let wall: Wall = Wall::new(0.0, 0.0, 1.0, 0.0);
        let new_direction: DVec2 = reflect_vector(direction_45_degrees, wall.normal());
        assert_eq!(new_direction, DVec2 { x: -1.0, y: 1.0 });
    }

    #[test]
    fn test_interpolation_to_wall() {}

    #[test]
    fn test_line_intersection_success() {
        let pt0a: DVec2 = DVec2::new(0.0, 0.0);
        let pt0b: DVec2 = DVec2::new(1.7320508075688774, 1.0);
        let pt1a: DVec2 = DVec2::new(1.0, 1.0);
        let pt1b: DVec2 = DVec2::new(1.0, -1.0);

        let intersection_pt = line_intersection(pt0a, pt0b, pt1a, pt1b);
        assert_eq!(
            intersection_pt,
            DVec2 {
                x: 1.0,
                y: 0.5773502691896257
            }
        )
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
