use ultraviolet::DVec2;

pub struct Wall {
    pub point_a: DVec2,
    pub point_b: DVec2,
}

pub struct Ball {
    pub start_position: DVec2,
    pub end_position: DVec2,
    pub tick: u8,
    pub rate: u8,
}

pub enum PlayerId {
    PlayerOne,
    PlayerTwo,
    None,
}

pub struct GameRules {
    pub score_to_win: u8,
}

pub struct GameState {
    pub ball: Ball,
    pub player0_score: u8,
    pub player1_score: u8,
}

pub struct Field {
    pub walls: [Wall; 4],
}
