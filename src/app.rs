use std::path::PathBuf;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use itertools::Itertools;
use ratatui::style::Stylize;
use ratatui::widgets::StatefulWidget;
use ratatui::{
    backend::Backend,
    widgets::{ListState, Paragraph, Widget},
    Terminal,
};
use ratatui::{
    layout::{Constraint, Direction, Flex, Layout},
    prelude::{Buffer, Rect},
    widgets::List,
};
use renamefile_tui::back_logic::rename_file_in_dir;

#[derive(Default)]
pub struct App<'a> {
    state: ListState,
    names: List<'a>,
    names_vec: Vec<String>,
}

impl App<'_> {
    pub fn with_items(items: impl IntoIterator<Item = String> + Clone) -> Self {
        Self {
            names: List::from_iter(items.clone()),
            state: ListState::default(),
            names_vec: items.into_iter().collect_vec(),
        }
    }

    pub fn run(&mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        loop {
            self.draw(&mut terminal)?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Up => self.previous_selection(),
                    KeyCode::Down => self.next_selection(),
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

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        terminal.draw(|f| f.render_widget(self, f.size()))?;
        Ok(())
    }

    fn render_title(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Rename file")
            .centered()
            .bold()
            .render(area, buf);
    }

    // TODO: Make text area that auto selects upon <enter>
    fn render_list(&self, area: Rect, buf: &mut Buffer) {
        let layout =
            Layout::new(Direction::Vertical, Constraint::from_fills([1])).flex(Flex::Center);
        let [area] = layout.areas(area);

        StatefulWidget::render(self.names.clone(), area, buf, &mut self.state.clone());
    }

    fn next_selection(&mut self) {
        let new_selection = self.state.selected().unwrap_or(1).saturating_add(1);
        self.state.select(Some(new_selection));
    }

    fn previous_selection(&mut self) {
        let new_selection = self.state.selected().unwrap_or(1).saturating_sub(1);
        self.state.select(Some(new_selection));
    }

    fn rename_file(&self, file_to_rename: &str) -> Result<()> {
        let chosen_option_idx: usize = self.state.selected().unwrap_or(0);
        let chosen_file_name: &str = &self.get_items_as_vec()[chosen_option_idx];
        let directory_called_in: &str = &std::env::args().collect_vec()[0];

        rename_file_in_dir(
            PathBuf::from(directory_called_in),
            file_to_rename.to_string(),
            chosen_file_name.to_string(),
        )?;

        Ok(())
    }

    #[allow(dead_code)]
    fn get_state(&self) -> ListState {
        self.state.clone()
    }

    #[allow(dead_code)]
    fn get_items(&self) -> List {
        self.names.clone()
    }

    #[allow(dead_code)]
    fn get_items_as_vec(&self) -> Vec<String> {
        let mut items_vec = Vec::new();
        for item in self.names_vec.clone() {
            items_vec.push(item);
        }

        items_vec
    }
}

impl Widget for &mut App<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        self.render_title(area, buf);
        self.render_list(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use crate::app::App;

    #[test]
    fn check_add() {
        let mut app = App::default();
        app.next_selection();
        assert_eq!(app.get_state().selected(), Some(2));
    }

    #[test]
    fn check_subtract() {
        let mut app = App::default();
        app.previous_selection();
        assert_eq!(app.get_state().selected(), Some(0));
    }
}
