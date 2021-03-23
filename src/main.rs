use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal, Result,
};

mod game;
use game::state::{GameState, Wall};
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

fn tick() {
    println!("Arbitrary tick.");
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

fn main() -> Result<()> {
    let wall0: Wall = Wall::new(0.0, 0.0, 6.0, 0.0);
    let wall1: Wall = Wall::new(0.0, 0.0, 6.0, 90.0);
    println!("{}", wall0);
    println!("{}", wall1);

    //let mut game_state: GameState = GameState::new();

    terminal::enable_raw_mode()?;

    loop {
        match read_char()? {
            '1' => println!("Hehe"),
            '2' => tick(),
            'q' => break,
            _ => {}
        };
    }

    terminal::disable_raw_mode()
}
