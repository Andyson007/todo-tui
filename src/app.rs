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
    pub popup: Option<CurrentPopup>,
    /// The title of the application
    pub title: String,
    /// The currently selected item (An index)
    pub selected: Option<usize>,
    /// All selectable options
    pub options: Vec<(String, String)>,
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// An enum containing an information about what
/// Datum is currently being edited
pub enum CurrentEdit {
    #[allow(missing_docs)]
    Title,
    #[allow(missing_docs)]
    Body,
}

/// The current popup to be shown
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CurrentPopup {
    /// They were in the Menu, but they are now editing an entry
    Edit(CurrentEdit),
    /// Add a new item to the todo list
    Add(CurrentEdit),
}
