//! The main module.
//! implements App and all of its features

/// Represents the current screen that
/// the user has selected
#[derive(Debug)]
pub enum CurrentScreen {
    /// They are currently selecting the menu in th emiddle on the left
    Menu,
}

#[derive(Debug)]
/// Contains all state information of the app
pub struct App {
    /// The screen that the user is currently selecting
    pub current_screen: CurrentScreen,
    /// The title of the application
    pub title: String,
    /// The currently selected item (An index)
    pub selected: Option<usize>,
    /// All selectable options
    pub options: Vec<usize>,
}

impl Default for App {
    fn default() -> Self {
        App {
            current_screen: CurrentScreen::Menu,
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
