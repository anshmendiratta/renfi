use std::hash::Hash;
use std::path::PathBuf;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use itertools::Itertools;
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, ToText};
use ratatui::widgets::{Block, Borders, HighlightSpacing};
use ratatui::Frame;
use ratatui::{
    backend::Backend,
    widgets::{ListState, Paragraph, Widget},
    Terminal,
};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::{Buffer, Rect},
    widgets::List,
};
use renamefile_tui::back_logic::rename_file_in_dir;

#[derive(Default)]
pub struct App<'a> {
    state: ListState,
    list: List<'a>,
    names: Vec<String>,
}

impl<'a> App<'a> {
    pub fn with_items(items: impl IntoIterator<Item = String> + Clone) -> Self {
        let list = List::default().items(items.clone());
        Self {
            state: ListState::default(),
            list,
            names: items.into_iter().collect(),
        }
    }

    pub fn run_app(&mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        loop {
            self.run(&mut terminal)?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Up => self.previous(),
                    KeyCode::Down => self.next(),
                    KeyCode::Enter => {
                        let file_to_rename: &str = &std::env::args().collect_vec()[1];
                        self.rename_file(file_to_rename)?
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    fn get_state(&self) -> ListState {
        self.state.clone()
    }

    #[allow(dead_code)]
    fn get_items(&self) -> Vec<String> {
        self.names.clone()
    }

    #[allow(dead_code)]
    fn get_list(&self) -> List<'a> {
        self.list.clone()
    }

    fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        terminal.draw(|f| {
            self.draw(f);
        })?;
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        self.render_title(frame.area(), frame.buffer_mut());
        self.render_list(frame);
    }

    fn render_title(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::default()
            .centered()
            .bold()
            .block(Block::default().title("Rename file").borders(Borders::ALL))
            .render(area, buf);
    }

    fn render_list(&mut self, frame: &mut Frame) {
        let num_items = self.names.len();
        let mut names_copy = self.get_items().clone();
        names_copy.sort_by(|a, b| b.len().cmp(&a.len()));

        let [area] = Layout::horizontal(Constraint::from_lengths([names_copy[0].len() as u16]))
            .flex(ratatui::layout::Flex::Center)
            .areas(frame.area());
        let [area] = Layout::vertical(Constraint::from_lengths([self.get_items().len() as u16]))
            .flex(ratatui::layout::Flex::Center)
            .areas(area);
        let list = List::new(self.names.clone())
            .highlight_style(Style::new().green())
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::WhenSelected)
            .slow_blink()
            .not_bold();

        frame.render_stateful_widget(self.get_list(), area, &mut self.state);
    }

    fn next(&mut self) {
        let new_selection = self.state.selected().unwrap_or(1).saturating_add(1);
        self.state.select(Some(new_selection));
    }

    fn previous(&mut self) {
        let new_selection = self.state.selected().unwrap_or(1).saturating_sub(1);
        self.state.select(Some(new_selection));
    }

    fn rename_file(&self, file_to_rename: &str) -> Result<()> {
        let chosen_option_idx: usize = self.state.selected().unwrap_or(0);
        let chosen_file_name: &str = &self.names[chosen_option_idx];
        let directory_called_in: &str = &std::env::args().collect_vec()[0];

        rename_file_in_dir(
            PathBuf::from(directory_called_in),
            file_to_rename.to_string(),
            chosen_file_name.to_string(),
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::app::App;

    #[test]
    fn check_add() {
        let mut app = App::default();
        app.next();
        assert_eq!(app.get_state().selected(), Some(2));
    }

    #[test]
    fn check_subtract() {
        let mut app = App::default();
        app.previous();
        assert_eq!(app.get_state().selected(), Some(0));
    }
}
