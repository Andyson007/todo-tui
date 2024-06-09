//! This module is responsible for handling all ui operations
//! It uses an [`App`] instance for this

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, List, Paragraph},
    Frame,
};

use ratatui::{prelude::*, widgets::*};

use crate::app::App;

/// Draws the ui.
/// It probably assumes a lot about the
/// terminal being in raw mode etc.
pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.size());

    draw_selection(frame, chunks[0], app);
    draw_info(frame, chunks[1], app)
}

/// draws the associated inforation with the current item
fn draw_info(frame: &mut Frame, chunk: Rect, app: &App) {
    let info = Paragraph::new(Text::raw(match app.selected {
        Some(x) => app.options[x].1.to_owned(),
        None => "".to_string(),
    }))
    .block(Block::bordered());

    frame.render_widget(info, chunk);
}

/// Draws all things that are interactable
fn draw_selection(frame: &mut Frame, chunk: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(chunk);
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(&app.title, Style::default().fg(Color::Green)))
        .block(title_block);

    frame.render_widget(title, chunks[0]);

    let mut state = ListState::with_selected(ListState::default(), app.selected);
    let list = List::new(app.options.iter().map(|x| x.0.to_owned()))
        .block(Block::bordered().title("List"))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED));

    frame.render_stateful_widget(list, chunks[1], &mut state);
}
