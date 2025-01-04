use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use itertools::Itertools;
use ratatui::layout::Flex;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Text;
use ratatui::widgets::{Block, HighlightSpacing};
use ratatui::Frame;
use ratatui::{
    backend::Backend,
    widgets::{ListState, Widget},
    Terminal,
};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::Buffer,
    widgets::List,
};
use renamefile_tui::back_logic::rename_file_in_dir;

#[derive(Default)]
pub struct App {
    state: ListState,
    // list: List<'a>,
    names: Vec<String>,
}

const SELECTED_STYLE: Style = Style::new().fg(Color::Black).bg(Color::White);

impl App {
    pub fn with_items(items: impl IntoIterator<Item = String> + Clone) -> Self {
        // let list = List::default()
        //     .items(items.clone())
        // .highlight_style(SELECTED_STYLE);
        Self {
            state: ListState::default(),
            // list,
            names: items.into_iter().collect(),
        }
    }

    pub fn run_app(&mut self, mut terminal: Terminal<impl Backend>) -> anyhow::Result<String> {
        loop {
            self.run(&mut terminal)?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => std::process::exit(0),
                    KeyCode::Up => self.previous(),
                    KeyCode::Down => self.next(),
                    KeyCode::Enter => {
                        let args_vec = std::env::args().collect_vec();
                        let file_to_rename: &str = &args_vec[1];
                        let mv_cmd_output = self.rename_file(file_to_rename)?;

                        return Ok(mv_cmd_output.to_owned());
                    }
                    _ => {}
                }
            }
        }
    }

    #[allow(dead_code)]
    fn get_state(&self) -> ListState {
        self.state.clone()
    }

    #[allow(dead_code)]
    fn get_items(&self) -> Vec<String> {
        self.names.clone()
    }

    fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        terminal.draw(|f| {
            self.draw(f);
        })?;
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let mut buf = frame.buffer_mut().clone();
        self.render_title(frame, &mut buf);
        self.render_list(frame);
    }

    fn render_title(&self, frame: &mut Frame, buf: &mut Buffer) {
        let title = Text::raw("Pick a title to rename the file")
            .centered()
            .bold();
        let [centered_top_layout] =
            Layout::horizontal(Constraint::from_lengths([title.width() as u16]))
                .flex(Flex::Center)
                // .vertical_margin(20)
                .areas(frame.area());

        Widget::render(title, centered_top_layout, buf);
    }

    fn render_list(&mut self, frame: &mut Frame) {
        let mut names_copy = self.get_items().clone();
        // Sort by string length, and store in reverse.
        names_copy.sort_by(|a, b| b.len().cmp(&a.len()));

        // `names_copy[0]` is the longest string.
        let [area] = Layout::horizontal(Constraint::from_lengths([names_copy[0].len() as u16]))
            .flex(ratatui::layout::Flex::Center)
            .areas(frame.area());
        let [area] = Layout::vertical(Constraint::from_lengths([
            self.get_items().len() as u16 + 2_16
        ]))
        .flex(ratatui::layout::Flex::Center)
        .spacing(2)
        .areas(area);
        let list = List::new(self.names.clone())
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_style(SELECTED_STYLE)
            .highlight_spacing(HighlightSpacing::WhenSelected)
            .block(Block::default().title("List of names"));

        frame.render_stateful_widget(list, area, &mut self.state);
    }

    fn next(&mut self) {
        let new_selection = self.state.selected().unwrap_or(1).saturating_add(1);
        self.state.select(Some(new_selection));
    }

    fn previous(&mut self) {
        let new_selection = self.state.selected().unwrap_or(1).saturating_sub(1);
        self.state.select(Some(new_selection));
    }

    fn rename_file<'a>(&'a self, file_to_rename: &'a str) -> Result<String> {
        let chosen_option_idx: usize = self.state.selected().unwrap_or(0);
        let chosen_file_name: &str = &self.names[chosen_option_idx];
        let mv_cmd_output = rename_file_in_dir(file_to_rename, chosen_file_name)?;

        Ok(mv_cmd_output)
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
