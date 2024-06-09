//! Creates a struct for building Apps
use crate::app::App;

/// A wrapper for [`App`] with many helper functions for
/// easier building
#[derive(Default, Debug)]
pub struct AppBuilder(App);

impl AppBuilder {
    /// Set the title of the app
    pub fn with_title(self, title: impl Into<String>) -> Self {
        Self(App {
            title: title.into(),
            ..self.0
        })
    }

    /// Set the options of the app
    pub fn with_options(self, options: impl Into<Vec<usize>>) -> Self {
        Self(App {
            options: options.into(),
            ..self.0
        })
    }
}

impl From<AppBuilder> for App {
    fn from(value: AppBuilder) -> Self {
        value.0
    }
}
