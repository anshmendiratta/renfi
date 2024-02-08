
use app::App;
use ratatui::prelude::{CrosstermBackend, Terminal};

use anyhow::Result;

mod ui;
mod app;

mod terminal_commands {
    use anyhow::Result;

    pub fn startup() -> Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
        Ok(())
    }

    pub fn shutdown() -> Result<()> {
        crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }
}

fn main() -> Result<()> {
    terminal_commands::startup()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
    let appstate = App {
        items: Vec::from(["hello", "world" ,"list"]),
        ..Default::default()
    };

    ui::draw_list(&mut terminal, &appstate)?;

    terminal_commands::shutdown()
}
