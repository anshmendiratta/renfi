use anyhow::Result;
use app::App;
use itertools::Itertools;
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::path::{Path, PathBuf};

use renamefile_tui::back_logic::get_possible_file_names;

mod app;

fn main() -> Result<()> {
    let assignments_dir_env = std::env::var("ASSIGNMENTS_DIR").unwrap_or_default();
    terminal_commands::startup()?;

    let terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
    let provided_file_name = PathBuf::from(&std::env::args().collect_vec()[1])
        .file_name()
        .unwrap_or_default()
        .to_os_string()
        .into_string()
        .unwrap_or_default();
    assert!(provided_file_name.len() != 0);
    let appstate_items: Vec<String> =
        get_possible_file_names(Path::new(assignments_dir_env.as_str()), &provided_file_name)
            .unwrap_or_default();

    let app_state = &mut App::with_items(appstate_items);
    let mv_cmd_output = app_state.run_app(terminal)?;

    terminal_commands::shutdown()?;
    println!("{}", mv_cmd_output);

    Ok(())
}

mod terminal_commands {
    use anyhow::Result;

    pub fn startup() -> Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
        Ok(())
    }

    pub fn shutdown() -> Result<()> {
        crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }
}
