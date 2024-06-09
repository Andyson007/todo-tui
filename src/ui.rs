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
    // Create the layout sections.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(&app.title, Style::default().fg(Color::Green)))
        .block(title_block);

    frame.render_widget(title, chunks[0]);

    let mut state = ListState::with_selected(ListState::default(), app.selected);
    let list = List::new(app.options.iter().map(|x| format!("{x}")))
        .block(Block::bordered().title("List"))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED));

    frame.render_stateful_widget(list, chunks[1], &mut state);
}
