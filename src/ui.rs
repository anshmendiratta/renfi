use crate::App;

use ratatui::{
    backend::Backend,
    layout::{Constraint, Flex, Layout},
    prelude::Terminal,
    style::{
        palette::tailwind::{self},
        Color, Style, Stylize,
    },
    widgets::{Block, List},
};

use anyhow::Result;

const TEXT_FG: Color = tailwind::LIME.c500;
const TEXT_BG: Color = tailwind::GRAY.c500;

pub fn draw_list(mut terminal: &Terminal<impl Backend>, appstate: &App) -> Result<()> {
    let dimensions = terminal.size()?;
    let layout = Layout::horizontal([
        Constraint::Length(20),
        Constraint::Length(20),
        Constraint::Length(20),
    ])
    .flex(Flex::Center);

    let list = List::new(appstate.items)
        .block(Block::default().title("OPTIONS").fg(TEXT_FG).bg(TEXT_BG))
        .highlight_symbol(">")
        .highlight_style(Style::new().bg(Color::Red).fg(Color::Yellow))
        .repeat_highlight_symbol(true);

    terminal.draw(|frame| {
        let area = frame.size();
        frame.render_widget(appstate, area);
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {}
