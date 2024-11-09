use ratatui_templates::app::{App, AppResult};
use ratatui_templates::event::{Event, EventHandler};
use ratatui_templates::handler::handle_key_events;
use ratatui_templates::tui::Tui;
use crossterm::event::{self, KeyEvent, KeyEventKind, KeyCode};
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;

    let events = EventHandler::new(200);

    let mut tui = Tui::new(terminal, events);
    
    // TODO: init the terminal
    Tui::init(&mut tui)?;
    // Start the main loop.
    while app.running {
        // TODO: Render the user interface. 
        tui.draw(&mut app);

        // TODO: Handle events.        
        let evt = event::read()?;
        if let crossterm::event::Event::Key(key) = evt {
            handle_key_events(key, &mut app);            
        }
    }
    // TODO: Reset the terminal if the app has been terminated
    tui.exit();
    Ok(())
}