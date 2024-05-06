use ratatui::{
    layout::{Constraint, Direction, Flex, Layout},
    prelude::{Buffer, Rect},
    style::Color,
};
use std::default;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::Backend,
    widgets::{ListState, Paragraph, Widget},
    Terminal,
};

use ratatui::style::Stylize;

use anyhow::Result;

pub struct App<'a> {
    should_quit: bool,
    state: ListState,
    pub items: Vec<&'a str>,
}

impl default::Default for App<'_> {
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
        terminal.draw(|frame| frame.render_widget(self, frame.size()))?;
        Ok(())
    }

    pub fn render_title(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("HELLO WORLD")
            .centered()
            .bold()
            .render(area, buf)
    }

    pub fn render_body(&self, area: Rect, buf: &mut Buffer) {
        let rect = Rect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        // TODO: Add flexbox spacing to body text
        let layout = Layout::new(Direction::Vertical, [Constraint::Length(20)])
            .flex(Flex::Center)
            .split(rect);
        let mut buf = Buffer::empty(area);

        let body = Paragraph::new("BODY TEXT")
            .bg(Color::White)
            .fg(Color::Black);

        body.render(area, &mut buf);
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

    pub fn get_state(&self) -> &ListState {
        &self.state
    }
}

impl Widget for &App<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        self.render_title(area, buf);
    }
}

impl Widget for &mut App<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        self.render_title(area, buf);
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
