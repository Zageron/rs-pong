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
