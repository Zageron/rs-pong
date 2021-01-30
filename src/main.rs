fn main() {
    const PLAY_FIELD_BLOCKS_HORIZONTAL: u16 = 32;
    const PLAY_FIELD_BLOCKS_VERTICAL: u16 = 32;
    const RESOLUTION_WIDTH: u16 = 640;
    const RESOLUTION_HEIGHT: u16 = 640;
    const BLOCK_WIDTH: u16 = RESOLUTION_WIDTH / PLAY_FIELD_BLOCKS_HORIZONTAL;
    const BLOCK_HEIGHT: u16 = RESOLUTION_HEIGHT / PLAY_FIELD_BLOCKS_VERTICAL;

    println!(
        "Block Width: {} | Block Height: {}",
        BLOCK_WIDTH, BLOCK_HEIGHT
    );

    ball_spawned_log(0, 0, 0., 0.);
    ball_bounced_log(0, 0, 0., 0.);
}

fn ball_spawned_log(point_x: u8, point_y: u8, vector_x: f32, vector_y: f32) {
    println!(
        "The ball has spawned at [{},{}], with a new vector of [{},{}].",
        point_x, point_y, vector_x, vector_y
    );
}

fn ball_bounced_log(point_x: u8, point_y: u8, vector_x: f32, vector_y: f32) {
    println!(
        "The ball has bounced off of the wall at [{},{}], with a new vector of [{},{}].",
        point_x, point_y, vector_x, vector_y
    );
}
