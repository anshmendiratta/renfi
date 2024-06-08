use anyhow::Result;
use app::App;
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::List,
};

mod app;

fn main() -> Result<()> {
    terminal_commands::startup()?;

    let terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
    let appstate_items: Vec<&str> = Vec::from(["list1", "list2", "list3"]);
    let appstate = &mut App::default();
    appstate.items = List::new(appstate_items);

    appstate.run(terminal)?;

    terminal_commands::shutdown()
}

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
