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

use crate::app::{App, CurrentMode};

/// Draws the ui.
/// It probably assumes a lot about the
/// terminal being in raw mode etc.
pub fn ui(frame: &mut Frame, app: &App) {
    match app.current_mode {
        ref mode @ (CurrentMode::Menu | CurrentMode::Edit(_) | CurrentMode::Add(_)) => {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(frame.size());

            draw_selection(frame, chunks[0], app);
            draw_info(frame, chunks[1], app);
            // Used to draw on top of the menu
            match mode {
                CurrentMode::Menu => (),
                CurrentMode::Edit(x) => {
                    let area = centered_rect(50, 50, frame.size());
                    frame.render_widget(Clear, area);

                    let popup_block = Block::default().title("Edit").borders(Borders::ALL);
                    frame.render_widget(popup_block, area);

                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Length(3), Constraint::Min(1)])
                        .margin(1)
                        .split(area);

                    let title = Block::default()
                        .title("Title")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Green));
                    let title_text =
                        Paragraph::new(&*app.options[app.selected.unwrap()].0).block(title);
                    frame.render_widget(title_text, chunks[0]);

                    let description = Block::default()
                        .title("Description")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Green));
                    let description_text =
                        Paragraph::new(&*app.options[app.selected.unwrap()].1).block(description);
                    frame.render_widget(description_text, chunks[1]);
                }
                CurrentMode::Add(_) => todo!(),
            }
        }
    }
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

/// Draws a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
