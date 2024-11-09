use ratatui_templates::app::{App, AppResult};
use ratatui_templates::event::{Event, EventHandler};
use ratatui_templates::handler::handle_key_events;
use ratatui_templates::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;

    let tick_rate = 200;
    let events = EventHandler::new(tick_rate);

    let mut tui = Tui::new(terminal, events);
    
    // TODO: init the terminal
    Tui::init(&mut tui);
    // Start the main loop.
    // while app.running {
        // TODO: Render the user interface. 
        // TODO: Handle events.
    // }

    // TODO: Reset the terminal if the app has been terminated

    Ok(())
}
