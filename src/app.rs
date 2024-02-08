use std::default;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::Backend,
    widgets::{ListState, Widget},
    Terminal,
};

use anyhow::Result;

pub struct App<'a> {
    should_quit: bool,
    state: ListState,
    pub items: Vec<&'a str>,
}

impl<'a> default::Default for App<'a> {
    fn default() -> Self {
        App {
            should_quit: false,
            state: ListState::default(),
            items: Vec::new(),
        }
    }
}

impl App<'_> {
    pub fn run(&mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        loop {
            self.draw(&mut terminal)?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up => self.previous_selection(),
                    KeyCode::Down => self.next_selection(),
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }

        Ok(())
    }

    pub fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.size()));
        Ok(())
    }

    pub fn next_selection(&mut self) {
        if let Some(result) = self.state.selected().unwrap_or(1).checked_add(1) {
            self.state.select(Some(result))
        }
    }

    pub fn previous_selection(&mut self) {
        if let Some(result) = self.state.selected().unwrap_or(1).checked_sub(1) {
            self.state.select(Some(result))
        }
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn get_state(&self) -> ListState {
        self.state
    }
}

impl Widget for &mut App<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        self.render(area, buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
