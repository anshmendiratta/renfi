use std::path::PathBuf;

use anyhow::Result;
use app::App;
use itertools::Itertools;
use ratatui::prelude::{CrosstermBackend, Terminal};
use renamefile_tui::back_logic::get_possible_file_names;

mod app;

fn main() -> Result<()> {
    terminal_commands::startup()?;

    let terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
    let provided_file_name = PathBuf::from(&std::env::args().collect_vec()[1])
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_owned();
    let appstate_items: Vec<String> =
        get_possible_file_names(&provided_file_name).unwrap_or_default();

    let app_state = &mut App::with_items(appstate_items);
    app_state.run(terminal)?;

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
