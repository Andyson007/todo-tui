//! The main module.
//! implements App and all of its features

use crossterm::event::KeyCode;

use crate::{
    help::Help,
    parse::todo::Items,
    popup::{self, Popup},
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
    /// 1.0: Description
    /// 1.1: Scroll height of this description
    #[allow(clippy::type_complexity)]
    pub options: Items,
    /// The current layout of the screen
    pub layout: ScreenLayout,
    /// The help menu stored
    pub help: Help,
    /// a bool determining whether we are in the substate and
    /// the information associated with it
    pub substate: Option<(bool, Substate)>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            layout: ScreenLayout::Small,
            current_selection: CurrentSelection::Menu,
            popup: None,
            selected: None,
            options: Items::default(),
            title: String::new(),
            help: Help::parse("./help.json").unwrap(),
            substate: None,
        }
    }
}

impl App {
    /// Changes what item is selected.
    pub fn change_menu_item(&mut self, dir: &Direction) {
        let len = self.options.amount();
        if len == 0 {
            return;
        }
        match dir {
            Direction::Up => self.selected = self.selected.map_or(Some(0), |x| Some((x + 1) % len)),
            Direction::Down => {
                self.selected = self.selected.map_or_else(
                    || Some(self.options.amount() - 1),
                    |x| Some((x + len - 1) % len),
                );
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
        let option = &self.options[loc];
        self.popup = Some(Popup::Edit {
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
pub enum ScreenLayout {
    /// Everything is at its smallest size
    Small,
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
    pub fn handle_input(&mut self, key: KeyCode) -> Option<bool> {
        if self.handle_substate(key) {
            return None;
        }
        if let Some(ref mut popup) = self.popup {
            match popup.handle_input(key, &self.help) {
                popup::ReturnAction::Exit => self.popup = None,
                popup::ReturnAction::Nothing => {}
                popup::ReturnAction::Edit(x, new_val) => {
                    self.options[x] = new_val.into();
                    self.popup = None;
                }
                popup::ReturnAction::Add(new_val) => {
                    self.options.add(new_val.into());
                    self.popup = None;
                }
                popup::ReturnAction::EnterSubState(x) => self.substate = Some((true, x)),
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
                        if selected == self.options.amount() {
                            if self.options.is_empty() {
                                self.selected = None;
                            } else {
                                self.selected = Some(selected - 1);
                            }
                        }
                    }
                    KeyCode::Char('/') => {
                        self.substate = Some((true, Substate::Filter(String::new())));
                    }
                    _ => (),
                },

                CurrentSelection::Description => {
                    match key {
                        // quit
                        KeyCode::Char('q') => self.current_selection = CurrentSelection::Menu,
                        // Vim motions
                        KeyCode::Char('j') | KeyCode::Down => {
                            if self.selected? != self.options.amount() - 1 {
                                self.options[self.selected?].description_scroll += 1;
                            }
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            self.options[self.selected?].description_scroll = self.options
                                [self.selected?]
                                .description_scroll
                                .saturating_sub(1);
                        }
                        _ => (),
                    }
                }
            }
        }
        None
    }

    /// Handles inputs when a substate is focused
    ///
    /// # Return value
    /// Whether the function eats the input or not
    /// true -> don't continue processing
    /// false -> continue processing
    fn handle_substate(&mut self, key: KeyCode) -> bool {
        let Some((ref mut editing @ true, ref mut substate)) = self.substate else {
            return false;
        };
        match substate {
            Substate::Filter(ref mut search) => match key {
                KeyCode::Enter => *editing = false,
                KeyCode::Esc => {
                    self.substate = None;
                }
                KeyCode::Backspace => drop(search.pop()),
                KeyCode::Char(c) => search.push(c),
                _ => (),
            },
        }
        true
    }
}
