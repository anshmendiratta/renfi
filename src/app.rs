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
use renamefile_tui::{get_possible_file_names, rename_file_in_dir};

#[derive(Default)]
pub struct App<'a> {
    should_quit: bool,
    state: ListState,
    pub items: List<'a>,
}

impl App<'_> {
    pub fn run(&mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        loop {
            self.draw(&mut terminal)?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Up => self.previous_selection(),
                    KeyCode::Down => self.next_selection(),
                    KeyCode::Enter => {
                        assert!(std::env::args().len() == 2);
                        let [ref directory_of_rename_file, ref file_to_rename] =
                            std::env::args().collect_vec()[0..=1]
                        else {
                            todo!("Can't match things");
                        };
                        let possible_file_names: Vec<String> =
                            get_possible_file_names(&file_to_rename)?;
                        let chosen_option_idx: usize = self.state.selected().unwrap_or(0);
                        let chosen_file_name: &str = &possible_file_names[chosen_option_idx];

                        rename_file_in_dir(
                            PathBuf::from(directory_of_rename_file),
                            file_to_rename.to_string(),
                            chosen_file_name.to_string(),
                        )?;
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    pub fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        terminal.draw(|f| f.render_widget(self, f.size()))?;
        Ok(())
    }

    pub fn render_title(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Rename file")
            .centered()
            .bold()
            .render(area, buf);
    }

    // TODO: Make text area that auto selects upon <enter>
    pub fn render_list(&self, area: Rect, buf: &mut Buffer) {
        let layout =
            Layout::new(Direction::Vertical, Constraint::from_fills([1])).flex(Flex::Center);
        let [area] = layout.areas(area);

        StatefulWidget::render(self.items.clone(), area, buf, &mut self.state.clone());
    }

    pub fn next_selection(&mut self) {
        let new_selection = self.state.selected().unwrap_or(1).saturating_add(1);
        self.state.select(Some(new_selection));
    }

    pub fn previous_selection(&mut self) {
        let new_selection = self.state.selected().unwrap_or(1).saturating_sub(1);
        self.state.select(Some(new_selection));
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn state(&self) -> &ListState {
        &self.state
    }
}

impl Widget for &mut App<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        self.render_title(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use crate::app::App;

    #[test]
    fn check_add() {
        let mut app = App::default();
        app.next_selection();
        assert_eq!(app.state().selected(), Some(2));
    }

    #[test]
    fn check_subtract() {
        let mut app = App::default();
        app.previous_selection();
        assert_eq!(app.state().selected(), Some(0));
    }
}
