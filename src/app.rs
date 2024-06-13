//! The main module.
//! implements App and all of its features

/// The current screen that should be shown to
/// behind all other popups
#[derive(Debug)]
pub enum CurrentScreen {
    /// They are currently selecting the menu in th emiddle on the left
    Menu,
    /// The description section in fullscreen
    Description,
}

#[derive(Debug)]
/// Contains all state information of the app
pub struct App {
    /// The screen that the user is currently selecting
    pub current_mode: CurrentScreen,
    /// The popup that is shown above everything
    pub popup: Option<Popup>,
    /// The title of the application
    pub title: String,
    /// The currently selected item (An index)
    pub selected: Option<usize>,
    /// All selectable options
    pub options: Vec<(Box<str>, Box<str>)>,
    /// The current layout of the screen
    pub layout: Layout,
}

impl Default for App {
    fn default() -> Self {
        App {
            layout: Layout::Small,
            current_mode: CurrentScreen::Menu,
            popup: None,
            selected: None,
            options: Vec::new(),
            title: String::new(),
        }
    }
}

impl App {
    /// Changes what item is selected.
    pub fn change_menu_item(&mut self, dir: Direction) {
        let len = self.options.len();
        if len == 0 {
            return;
        }
        match dir {
            Direction::Up => self.selected = self.selected.map_or(Some(0), |x| Some((x + 1) % len)),
            Direction::Down => {
                self.selected = self
                    .selected
                    .map_or(Some(self.options.len() - 1), |x| Some((x + len - 1) % len))
            }
        }
    }

    /// Sets the popup field sensibly
    pub fn edit(&mut self) {
        if self.popup.is_some() || self.selected.is_none() {
            // FIXME:
            panic!("Bad popup state")
        } else {
            let option = self.options[self.selected.unwrap()].clone();
            self.popup = Some(Popup::Edit {
                title: option.0.to_string(),
                description: option.1.to_string(),
                editing: CurrentEdit::Title,
            })
        }
    }
    /// Sets the state to Add a new item sensibly
    pub fn add(&mut self) {
        if self.popup.is_some() {
            // FIXME:
            panic!("Bad popup state")
        } else {
            self.popup = Some(Popup::Edit {
                title: String::new(),
                description: String::new(),
                editing: CurrentEdit::Title,
            })
        }
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
    /// You are about to add an item
    Add {
        /// The title of the item
        title: String,
        /// The description of the item
        description: String,
        /// The currently highlighted/edited part of the popup
        editing: CurrentEdit,
    },
    /// You are editing an item
    Edit {
        /// The title of the item
        title: String,
        /// The description of the item
        description: String,
        /// The currently highlighted/edited part of the popup
        editing: CurrentEdit,
    },
}

/// What part of a todo-item are you editing?
#[derive(Debug)]
#[allow(missing_docs)]
pub enum CurrentEdit {
    Title,
    Body,
}
