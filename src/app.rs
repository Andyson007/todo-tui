//! The main module.
//! implements App and all of its features

use crossterm::event::KeyCode;

use crate::help::Help;

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
    /// The screen that the user is currently selecting
    pub current_selection: CurrentSelection,
    /// The popup that is shown above everything
    pub popup: Option<Popup>,
    /// The title of the application
    pub title: String,
    /// The currently selected item (An index)
    pub selected: Option<usize>,
    /// All selectable options
    ///
    /// 0: Title,
    /// 1: Description
    /// 2: Scroll height of this description
    // FIXME: This should be Vec<(a, (b,c))> instead
    pub options: Vec<(Box<str>, Box<str>, usize)>,
    /// The current layout of the screen
    pub layout: Layout,
    /// The help menu stored
    pub help: Help,
    /// a bool determining whether we are in the substate and
    /// the information associated with it
    pub substate: Option<(bool, Substate)>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            layout: Layout::Small,
            current_selection: CurrentSelection::Menu,
            popup: None,
            selected: None,
            options: Vec::new(),
            title: String::new(),
            help: Help::parse("./help.json").unwrap(),
            substate: None,
        }
    }
}

impl App {
    /// Changes what item is selected.
    pub fn change_menu_item(&mut self, dir: &Direction) {
        let len = self.options.len();
        if len == 0 {
            return;
        }
        match dir {
            Direction::Up => self.selected = self.selected.map_or(Some(0), |x| Some((x + 1) % len)),
            Direction::Down => {
                self.selected = self
                    .selected
                    .map_or(Some(self.options.len() - 1), |x| Some((x + len - 1) % len));
            }
        }
    }

    /// Sets the popup field sensibly
    ///
    /// # Panics
    /// This function panics when opening up a popup when already in a popup
    pub fn edit(&mut self) {
        assert!(self.popup.is_none(), "we can't already be in a popup");
        if self.selected.is_none() {
            return;
        }
        let loc = self.selected.unwrap();
        let option = self.options[loc].clone();
        self.popup = Some(Popup::Edit {
            title: option.0.to_string(),
            description: option.1.to_string(),
            editing: CurrentEdit::Title,
            to_change: Some(loc),
        });
    }
    /// Sets the state to Add a new item sensibly
    ///
    /// # Panics
    /// Panics when opening a popup whilst already being in a popup
    pub fn add(&mut self) {
        assert!(self.popup.is_none(), "we can't already be in a popup");
        self.popup = Some(Popup::Edit {
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

/// The current layout of the screen
#[derive(Debug)]
pub enum Layout {
    /// Everything is at its smallest size
    Small,
}

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

enum PopupReturnAction {
    /// Exit the Popup
    Exit,
    /// Do nothing
    Nothing,
    /// Edit the item
    /// 0: the item to be edited
    /// 1: its new value
    Edit(usize, (Box<str>, Box<str>, usize)),
    /// Add an item
    /// The value to push
    Add((Box<str>, Box<str>, usize)),
}

impl Popup {
    // HACK: This should not have to take help as an input
    fn handle_input(&mut self, key: KeyCode, help: &Help) -> PopupReturnAction {
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
                KeyCode::Esc => return PopupReturnAction::Exit,
                KeyCode::Enter => {
                    return to_change.as_mut().map_or_else(
                        || {
                            PopupReturnAction::Add((
                                title.to_owned().into_boxed_str(),
                                description.to_owned().into_boxed_str(),
                                0,
                            ))
                        },
                        |x| {
                            PopupReturnAction::Edit(
                                *x,
                                (
                                    title.to_owned().into_boxed_str(),
                                    description.to_owned().into_boxed_str(),
                                    0,
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
                KeyCode::Char('q') => return PopupReturnAction::Exit,
                KeyCode::Char('j') => {
                    if *x != help.0.len() - 1 {
                        *x += 1;
                    }
                }
                KeyCode::Char('k') => *x = x.saturating_sub(1),
                _ => (),
            },
        }
        PopupReturnAction::Nothing
    }
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
pub enum Substate {
    /// Filter for a result
    /// 0: a string representing the current search query
    Filter(String),
}

impl App {
    /// Handles an input
    ///
    /// # Errors
    pub fn handle_input(&mut self, key: KeyCode) -> Option<bool> {
        if self.substate.as_ref().is_some_and(|x| x.0) {
            self.handle_substate(key);
        } else if let Some(ref mut popup) = self.popup {
            match popup.handle_input(key, &self.help) {
                PopupReturnAction::Exit => self.popup = None,
                PopupReturnAction::Nothing => {}
                PopupReturnAction::Edit(x, new_val) => {
                    self.options[x] = new_val;
                    self.popup = None;
                }
                PopupReturnAction::Add(new_val) => {
                    self.options.push(new_val);
                    self.popup = None;
                }
            };
        } else {
            match self.current_selection {
                CurrentSelection::Menu => match key {
                    // quit
                    KeyCode::Char('q') => return Some(true),
                    //
                    KeyCode::Char('?') => self.popup = Some(Popup::Help(0)),

                    // Vim motion + Down key
                    KeyCode::Char('j') | KeyCode::Down => self.change_menu_item(&Direction::Up),
                    // Vim motion + Down key
                    KeyCode::Char('k') | KeyCode::Up => self.change_menu_item(&Direction::Down),
                    // Enter edit mode
                    KeyCode::Char('e') if self.selected.is_some() => self.edit(),
                    // Enter add mode (Add a new item)
                    KeyCode::Char('a') => self.add(),
                    // Focus the description
                    KeyCode::Enter => {
                        if self.selected.is_some() {
                            self.current_selection = CurrentSelection::Description;
                        }
                    }

                    // Delete entry
                    KeyCode::Char('d') if self.selected.is_some() => {
                        let selected = unsafe { self.selected.unwrap_unchecked() };
                        self.options.remove(selected);
                        if selected == self.options.len() {
                            if self.options.is_empty() {
                                self.selected = None;
                            } else {
                                self.selected = Some(selected - 1);
                            }
                        }
                    }
                    _ => (),
                },

                CurrentSelection::Description => {
                    match key {
                        // quit
                        KeyCode::Char('q') => self.current_selection = CurrentSelection::Menu,
                        // Vim motions
                        KeyCode::Char('j') | KeyCode::Down => {
                            if self.selected? != self.options.len() - 1 {
                                self.options[self.selected?].2 += 1;
                            }
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            self.options[self.selected?].2 =
                                self.options[self.selected?].2.saturating_sub(1);
                        }
                        _ => (),
                    }
                }
            }
        }
        None
    }

    /// Handles inputs when a substate is focused
    fn handle_substate(&mut self, key: KeyCode) {
        let Some((ref mut editing @ false, ref mut substate)) = self.substate else {
            return;
        };
        match substate {
            Substate::Filter(ref mut search) => match key {
                KeyCode::Enter => *editing = false,
                KeyCode::Esc => {
                    self.substate = None;
                }
                KeyCode::Char(c) => search.push(c),
                _ => (),
            },
        }
    }
}
