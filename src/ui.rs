//! This module is responsible for handling all ui operations
//! It uses an [`App`] instance for this

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, List, Paragraph},
    Frame,
};

use ratatui::{
    prelude::*,
    widgets::{Clear, ListState, Wrap},
};

use crate::{
    app::{App, CurrentEdit, CurrentSelection, ScreenLayout, State, Substate, SubstateMode},
    popup::Popup,
    query,
};

/// Draws the ui.
/// It probably assumes a lot about the
/// terminal being in raw mode etc.
pub fn ui(frame: &mut Frame, app: &App) {
    match app.layout {
        ScreenLayout::Small(ref state) => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(1),
                    Constraint::Length(u16::from(state.substate.is_some())),
                ])
                .split(frame.size());
            if let Some(substate) = &state.substate {
                substate.render(substate.in_state, frame, chunks[1]);
            }
            match state.current_selection {
                ref a @ (CurrentSelection::Menu | CurrentSelection::Description) => {
                    let chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(chunks[0]);

                    draw_selection(frame, chunks[0], state, a);
                    draw_info(frame, chunks[1], state, a);
                    // Used to draw on top of the menu
                }
            }
            if let Some(popup) = &state.popup {
                match popup {
                    Popup::Edit {
                        ref title,
                        ref description,
                        ref editing,
                        ..
                    } => {
                        render_title_desc(title, description, editing, frame);
                    }
                    Popup::Help(selected) => {
                        let area = centered_rect(60, 60, frame.size());
                        frame.render_widget(Clear, area);
                        let (substate_control, opts) = {
                            if let Some(Substate {
                                in_state,
                                substate_mode,
                            }) = &state.substate
                            {
                                (
                                    *in_state,
                                    match substate_mode {
                                        SubstateMode::Filter(x) => query(
                                            app.static_information.help.items.to_vec(),
                                            x.as_str(),
                                        ),
                                    },
                                )
                            } else {
                                (
                                    false,
                                    app.static_information
                                        .help
                                        .items
                                        .iter()
                                        .cloned()
                                        .enumerate()
                                        .collect(),
                                )
                            }
                        };
                        // HACK: This only renders the cursor at the top while searching The cursor
                        // automatically jumps back to its previous position afterward
                        let selected = if substate_control { 0 } else { *selected };
                        let chunks = Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints(Constraint::from_percentages([30, 70]))
                            .split(area);
                        let mut state =
                            ListState::with_selected(ListState::default(), Some(selected));
                        let description = &opts.get(selected);
                        let text = List::new(opts.iter().map(|x| x.1 .0 .0.to_string()))
                            .block(Block::default().title("Help").borders(Borders::ALL))
                            .scroll_padding(3)
                            .highlight_style(Style::new().add_modifier(Modifier::REVERSED));
                        frame.render_stateful_widget(text, chunks[0], &mut state);
                        let description = Paragraph::new(Text::raw(
                            description.map_or(String::new(), |description| {
                                description.1 .0 .1.to_string()
                            }),
                        ))
                        .block(Block::default().title("Desc").borders(Borders::ALL));
                        frame.render_widget(description, chunks[1]);
                    }
                }
            }
        }
        ScreenLayout::ListChoice => {
            // TODO: Make it do something
        }
    }
}

/// draws the associated inforation with the current item
fn draw_info(frame: &mut Frame, chunk: Rect, state: &State, selection: &CurrentSelection) {
    let info = Paragraph::new(Text::raw(state.selected.map_or_else(String::new, |x| {
        state.current_data[x].description.to_string()
    })))
    .block(
        Block::bordered().style(if matches!(selection, CurrentSelection::Description) {
            Color::Green
        } else {
            Color::White
        }),
    )
    .wrap(Wrap { trim: false })
    .scroll((
        state.selected.map_or(0, |x| {
            state.current_data[x]
                .description_scroll
                .try_into()
                .expect("Corgats! You wasted time")
        }),
        0,
    ));

    frame.render_widget(info, chunk);
}

/// Draws all things that are interactable
fn draw_selection(frame: &mut Frame, chunk: Rect, state: &State, selection: &CurrentSelection) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(chunk);
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        &state.title,
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    let mut list_state = ListState::with_selected(ListState::default(), state.selected);
    let list = List::new(state.current_data.titles())
        .block(Block::bordered().title("List").style(
            if matches!(selection, CurrentSelection::Menu) {
                Color::Green
            } else {
                Color::White
            },
        ))
        .scroll_padding(3)
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED));

    frame.render_stateful_widget(list, chunks[1], &mut list_state);
}

/// This code is absolutely stolen from the ratatui json example
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

fn render_title_desc(title: &str, description: &str, editing: &CurrentEdit, frame: &mut Frame) {
    let area = centered_rect(50, 50, frame.size());
    frame.render_widget(Clear, area);

    let popup_block = Block::default().title("Edit").borders(Borders::ALL);
    frame.render_widget(popup_block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .margin(1)
        .split(area);

    let title_block = Block::default()
        .title("Title")
        .borders(Borders::ALL)
        .border_style(
            Style::default().fg(if matches!(editing, CurrentEdit::Title) {
                Color::Green
            } else {
                Color::White
            }),
        );
    let title_text = Paragraph::new(
        title
            .chars()
            .chain(if matches!(editing, CurrentEdit::Title) {
                Some('█')
            } else {
                None
            })
            .collect::<String>(),
    )
    .block(title_block);
    frame.render_widget(title_text, chunks[0]);

    let description_block = Block::default()
        .title("Description")
        .borders(Borders::ALL)
        .border_style(
            Style::default().fg(if matches!(editing, CurrentEdit::Body) {
                Color::Green
            } else {
                Color::White
            }),
        );
    let description_text = Paragraph::new(
        description
            .chars()
            .chain(if matches!(editing, CurrentEdit::Body) {
                Some('█')
            } else {
                None
            })
            .collect::<String>(),
    )
    .block(description_block)
    .wrap(Wrap { trim: false });
    frame.render_widget(description_text, chunks[1]);
}

impl Substate {
    #![allow(clippy::doc_markdown)]
    /// renders self into a widget
    ///
    /// # Parameters
    /// in_state are we currently in the substate?
    /// frame: The global frame to draw on
    /// chunk: The Rectangle which we are allowed to modify
    pub fn render(&self, in_state: bool, frame: &mut Frame, chunk: Rect) {
        match &self.substate_mode {
            SubstateMode::Filter(x) => frame.render_widget(
                Text::raw({
                    let mut ret = format!("/{x}");
                    if in_state {
                        ret.push('█');
                    }
                    ret
                })
                .style(Style::default().fg(Color::Blue)),
                chunk,
            ),
        }
    }
}
