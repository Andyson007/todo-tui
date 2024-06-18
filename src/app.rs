//! The main module.
//! implements App and all of its features

use std::{error::Error, path::Path};

use crossterm::event::KeyCode;

use crate::{
    parse::todo::{self, Items},
    popup::{self, Popup},
    static_info::StaticInfo,
};

/// The current screen that should be shown to
/// behind all other popups
#[derive(Debug)]
pub enum CurrentSelection {
    /// They are currently selecting the menu in th emiddle on the left
    Menu,
    /// The description section in fullscreen
    Description,
}

#[derive(Debug)]
/// Contains all state information of the app
pub struct App {
    /// The help menu stored and all the todo lists are stored here
    pub static_information: StaticInfo,
    /// What layout the screen is currently in. These layouts contain information about the state
    pub layout: ScreenLayout,
}

impl App {
    /// Takes files and makes an app from them
    ///
    /// # Errors
    /// File not found
    pub fn from_files<P>(lists: P, help: P) -> Result<Self, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        Ok(Self {
            layout: ScreenLayout::ListChoice,
            static_information: StaticInfo::from(lists, help)?,
        })
    }
}

impl App {
    /// Changes what item is selected.
    pub fn change_menu_item(state: &mut State, dir: &Direction) {
        let len = state.current_data.amount();
        if len == 0 {
            return;
        }
        match dir {
            Direction::Up => {
                state.selected = state.selected.map_or(Some(0), |x| Some((x + 1) % len));
            }
            Direction::Down => {
                state.selected = state.selected.map_or_else(
                    || Some(state.current_data.amount() - 1),
                    |x| Some((x + len - 1) % len),
                );
            }
        }
    }

    /// Sets the popup field sensibly
    ///
    /// # Panics
    /// This function panics when opening up a popup when already in a popup
    pub fn edit(state: &mut State) {
        assert!(state.popup.is_none(), "we can't already be in a popup");
        if state.selected.is_none() {
            return;
        }
        let loc = state.selected.unwrap();
        let option = &state.current_data[loc];
        state.popup = Some(Popup::Edit {
            title: option.title.to_string(),
            description: option.description.to_string(),
            editing: CurrentEdit::Title,
            to_change: Some(loc),
        });
    }
    /// Sets the state to Add a new item sensibly
    ///
    /// # Panics
    /// Panics when opening a popup whilst already being in a popup
    pub fn add(state: &mut State) {
        assert!(state.popup.is_none(), "we can't already be in a popup");
        state.popup = Some(Popup::Edit {
            title: String::new(),
            description: String::new(),
            editing: CurrentEdit::Title,
            to_change: None,
        });
    }
}

/// The direction that was moved
#[derive(Debug)]
pub enum Direction {
    /// Moved down
    Up,
    /// Moved up
    Down,
}

/// What part of a todo-item are you editing?
#[derive(Debug)]
#[allow(missing_docs)]
pub enum CurrentEdit {
    Title,
    Body,
}

/// Contains substates that should be accessible on every screen
#[derive(Debug)]
// TODO: make a struct
pub enum Substate {
    /// Filter for a result
    /// 0: a string representing the current search query
    Filter(String),
}

impl App {
    /// Handles an input
    #[allow(clippy::missing_panics_doc)]
    pub fn handle_input(&mut self, key: KeyCode) -> Option<bool> {
        match self.layout {
            ScreenLayout::Small(ref mut state) => {
                match Self::handle_substate(state, key) {
                    SubstateReturn::Continue => (),
                    SubstateReturn::Exit => return None,
                    SubstateReturn::Select => {
                        if let Some(Popup::Help(ref mut x)) = state.popup {
                            *x = 0;
                        }
                        return None;
                    }
                }
                if let Some(ref mut popup) = state.popup {
                    match popup.handle_input(key, &self.static_information.help) {
                        popup::ReturnAction::Exit => state.popup = None,
                        popup::ReturnAction::Nothing => {}
                        popup::ReturnAction::Edit(x, new_val) => {
                            self.static_information
                                .lists
                                .get_mut(&state.current_list)
                                .unwrap()[x] = new_val.into();
                            state.popup = None;
                        }
                        popup::ReturnAction::Add(new_val) => {
                            self.static_information
                                .lists
                                .get_mut(&state.current_list)
                                .unwrap()
                                .add(new_val.into());
                            state.popup = None;
                        }
                        popup::ReturnAction::EnterSubState(x) => state.substate = Some((true, x)),
                    };
                } else if let Some(x) = Self::handle_main_menu(state, key) {
                    return Some(x);
                }
                None
            }
            ScreenLayout::ListChoice => {
                self.layout = ScreenLayout::Small(State {
                    current_selection: CurrentSelection::Menu,
                    popup: None,
                    title: "Andy!".to_string(),
                    selected: None,
                    substate: None,
                    current_list: "AndyCo".to_string(),
                    current_data: self.static_information.lists.get("AndyCo").unwrap().clone(),
                });
                None
            }
        }
    }

    fn handle_main_menu(state: &mut State, key: KeyCode) -> Option<bool> {
        match state.current_selection {
            CurrentSelection::Menu => match key {
                // quit
                KeyCode::Char('q') | KeyCode::Esc => return Some(true),
                // Help
                KeyCode::Char('?') => state.popup = Some(Popup::Help(0)),
                // Vim motion + Down key
                KeyCode::Char('j') | KeyCode::Down => Self::change_menu_item(state, &Direction::Up),
                // Vim motion + Down key
                KeyCode::Char('k') | KeyCode::Up => Self::change_menu_item(state, &Direction::Down),
                // Enter edit mode
                // TODO: rename edit to something more descriptive
                KeyCode::Char('e') if state.selected.is_some() => Self::edit(state),
                // Enter add mode (Add a new item)
                KeyCode::Char('a') => Self::add(state),
                // Focus the description
                KeyCode::Enter => {
                    if state.selected.is_some() {
                        state.current_selection = CurrentSelection::Description;
                    }
                }

                // Delete entry
                KeyCode::Char('d') if state.selected.is_some() => {
                    let selected = unsafe { state.selected.unwrap_unchecked() };
                    state.current_data.remove(selected);
                    if selected == state.current_data.amount() {
                        if state.current_data.is_empty() {
                            state.selected = None;
                        } else {
                            state.selected = Some(selected - 1);
                        }
                    }
                }
                KeyCode::Char('/') => {
                    state.substate = Some((true, Substate::Filter(String::new())));
                }
                _ => (),
            },

            CurrentSelection::Description => {
                match key {
                    // quit
                    KeyCode::Char('q') => state.current_selection = CurrentSelection::Menu,
                    // Vim motions
                    KeyCode::Char('j') | KeyCode::Down => {
                        if state.selected? != state.current_data.amount() - 1 {
                            state.current_data[state.selected?].description_scroll += 1;
                        }
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        state.current_data[state.selected?].description_scroll = state.current_data
                            [state.selected?]
                            .description_scroll
                            .saturating_sub(1);
                    }
                    _ => (),
                }
            }
        }
        None
    }

    /// Handles inputs when a substate is focused
    fn handle_substate(state: &mut State, key: KeyCode) -> SubstateReturn {
        let Some((ref mut editing @ true, ref mut substate)) = state.substate else {
            return SubstateReturn::Continue;
        };
        match substate {
            Substate::Filter(ref mut search) => match key {
                KeyCode::Enter => {
                    *editing = false;
                    return SubstateReturn::Select;
                }
                KeyCode::Esc => {
                    state.substate = None;
                }
                KeyCode::Backspace => drop(search.pop()),
                KeyCode::Char(c) => search.push(c),
                _ => (),
            },
        }
        SubstateReturn::Exit
    }
}

/// HACK: This is a really temporary solution
enum SubstateReturn {
    /// Continue processing key events
    Continue,
    /// Stop processing key events
    Exit,
    /// Stop processing key events, but also select the default selection
    Select,
}

/// The current layout of the screen
#[derive(Debug)]
pub enum ScreenLayout {
    /// Everything is at its smallest size
    Small(State),
    /// Selecting the list to load
    ListChoice,
}

/// State information for the main screen layout
#[derive(Debug)]
pub struct State {
    /// The screen that the user is currently selecting
    pub current_selection: CurrentSelection,
    /// The popup that is shown above everything
    pub popup: Option<Popup>,
    /// The title of the application
    pub title: String,
    /// The currently selected item (An index)
    pub selected: Option<usize>,
    /// a bool determining whether we are in the substate and
    /// the information associated with it
    pub substate: Option<(bool, Substate)>,
    /// What Todo list are we currently editing?
    pub current_list: String,
    /// What items are in the current list?
    pub current_data: Items<todo::Item>,
}
