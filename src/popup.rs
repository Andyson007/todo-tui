//! Handles information about popups
use crossterm::event::KeyCode;

use crate::{app::{CurrentEdit, SubstateMode}, help, parse::todo::Items};

/// State data for a popup
#[derive(Debug)]
pub enum Popup {
    /// You are editing an item
    Edit {
        /// The title of the item
        title: String,
        /// The description of the item
        description: String,
        /// The currently highlighted/edited part of the popup
        editing: CurrentEdit,
        /// The index of the currently edited item if its empty then a new item is being added
        to_change: Option<usize>,
    },

    /// Show help menu
    Help(
        /// the index of the currently selected item
        usize,
    ),
}

/// Describes what action should be taken when a popup is present
#[derive(Debug)]
pub enum ReturnAction {
    /// Exit the Popup
    Exit,
    /// Do nothing
    Nothing,
    /// Edit the item
    /// 0: the item to be edited
    /// 1: its new value
    Edit(usize, (Box<str>, (Box<str>, usize))),
    /// Add an item
    /// The value to push
    Add((Box<str>, (Box<str>, usize))),
    /// Enter a substate
    EnterSubState(SubstateMode),
}

impl Popup {
    // HACK: This should not have to take help as an input

    /// Handles input
    pub fn handle_input(&mut self, key: KeyCode, help: &Items<help::Item>) -> ReturnAction {
        match self {
            Self::Edit {
                ref mut title,
                ref mut description,
                ref mut editing,
                to_change,
            } => match key {
                KeyCode::Backspace => drop(
                    match editing {
                        CurrentEdit::Title => title,
                        CurrentEdit::Body => description,
                    }
                    .pop(),
                ),
                KeyCode::Esc => return ReturnAction::Exit,
                KeyCode::Enter => {
                    return to_change.as_mut().map_or_else(
                        || {
                            ReturnAction::Add((
                                title.to_owned().into_boxed_str(),
                                (description.to_owned().into_boxed_str(), 0),
                            ))
                        },
                        |x| {
                            ReturnAction::Edit(
                                *x,
                                (
                                    title.to_owned().into_boxed_str(),
                                    (description.to_owned().into_boxed_str(), 0),
                                ),
                            )
                        },
                    )
                }
                KeyCode::Tab => {
                    *editing = match editing {
                        CurrentEdit::Title => CurrentEdit::Body,
                        CurrentEdit::Body => CurrentEdit::Title,
                    }
                }
                KeyCode::Char(x) => match editing {
                    CurrentEdit::Title => title,
                    CurrentEdit::Body => description,
                }
                .push(x),
                _ => (),
            },
            Self::Help(ref mut x) => match key {
                KeyCode::Char('q') => return ReturnAction::Exit,
                KeyCode::Char('j') => {
                    if *x != help.items.len() - 1 {
                        *x += 1;
                    }
                }
                KeyCode::Char('k') => *x = x.saturating_sub(1),
                KeyCode::Char('/') => {
                    return ReturnAction::EnterSubState(SubstateMode::Filter(String::new()))
                }
                _ => (),
            },
        }
        ReturnAction::Nothing
    }
}
