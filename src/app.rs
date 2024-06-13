use std::path::PathBuf;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use itertools::Itertools;
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, Borders, HighlightSpacing, StatefulWidget};
use ratatui::Frame;
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
pub struct App {
    state: ListState,
    names: Vec<String>,
}

impl App {
    pub fn with_items(items: impl IntoIterator<Item = String> + Clone) -> Self {
        Self {
            state: ListState::default(),
            names: items.into_iter().collect(),
        }
    }

    pub fn run(&mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        loop {
            self.draw(&mut terminal)?;

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

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        terminal.draw(|f| f.render_widget(self, f.size()))?;
        Ok(())
    }

    fn render_title(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::default()
            .centered()
            .bold()
            .block(Block::default().title("Rename file").borders(Borders::ALL))
            .render(area, buf);
    }

    fn render_list(&self, area: Rect, buf: &mut Buffer) {
        let layout_vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints(Constraint::from_fills([1, 1, 1]))
            .flex(Flex::Center)
            .split(Rect::new(1, 1, 1));
        let layout_horizontal = Layout::default()
            .direction(Direction::Vertical)
            .constraints(Constraint::from_fills([1, 1, 1]))
            .flex(Flex::Center)
            .split(layout_vertical[1]);

        let list = List::new(self.names.clone())
            .highlight_style(Style::new().green())
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::WhenSelected)
            .slow_blink()
            .not_bold();

        StatefulWidget::render(list, area_horiz, buf, &mut self.state.clone());
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

    #[allow(dead_code)]
    fn get_state(&self) -> ListState {
        self.state.clone()
    }

    #[allow(dead_code)]
    fn get_items(&self) -> Vec<String> {
        self.names.clone()
    }
}

impl Widget for &mut App {
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
