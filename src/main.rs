use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal, Result,
};

mod game;
use game::state::GameState;
mod model;
mod render;

pub fn read_char() -> Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        })) = event::read()
        {
            return Ok(c);
        }
    }
}

fn tick(state: &mut GameState) {
    println!(" tick. {}", state);
}

fn tick_up(state: &mut GameState) {
    print!("Paddle up, and ");
    tick(state);
}

fn tick_down(state: &mut GameState) {
    print!("Paddle down, and ");
    tick(state);
}

fn main() -> Result<()> {
    let mut game_state: GameState = GameState::new();

    terminal::enable_raw_mode()?;

    loop {
        match read_char()? {
            'w' => tick_up(&mut game_state),
            's' => tick_down(&mut game_state),
            'q' => break,
            _ => {}
        };
    }

    terminal::disable_raw_mode()
}

// Initialize app state control.
//  - Pre-Game
//  - Active game
//  - Post-Game

// Initialize game state control.
//  - Player 1 and 2 scores.
//  - Player 1 and 2 paddle positions.
//  - Ball position and initial direction vector. (Angle of bounce)
//  - Ball "speed" setting.
//  - Current Turn / Turn Counters
//  - Game State: Turn Start vs Active

// Set up game rules:
//  - Boundaries for "turn end"
//  - Turn termination on boundary.
//  - Interception logic for paddles. (Reflection)
//  - Interception logic for top and bottom walls. (Reflection)
//  - Wait for interaction to fire the pong ball at turn start.
