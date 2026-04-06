mod input;
mod state;
mod hypr;

use anyhow::Result;
use input::InputHandler;
use state::State;
use hypr::dispatch;

fn main() -> Result<()> {
    println!("Starting hypr-alt-supervisor...");

    let mut input = InputHandler::new()?;
    let mut state = State::new();

    loop {
        if let Some(event) = input.next_event()? {
            if let Some(action) = state.handle_event(event) {
                println!("Action triggered: {:?}", action);
                if let Err(e) = dispatch(action) {
                    eprintln!("Dispatch error: {}", e);
                }
            }
        }
    }
}

