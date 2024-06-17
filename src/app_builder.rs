//! Creates a struct for building Apps
use crate::{app::App, parse::todo::Items};

/// A wrapper for [`App`] with many helper functions for
/// easier building
#[derive(Default, Debug)]
pub struct AppBuilder(App);

impl AppBuilder {
    /// Set the title of the app
    #[must_use]
    pub fn with_title(self, title: impl Into<String>) -> Self {
        Self(App {
            title: title.into(),
            ..self.0
        })
    }

    /// Set the options of the app
    #[must_use]
    pub fn with_options<T>(self, options: impl IntoIterator<Item = (T, T)>) -> Self
    where
        T: Into<String>,
    {
        Self(App {
            options: options
                .into_iter()
                .map(|(a, b)| (a.into().into_boxed_str(), (b.into().into_boxed_str(), 0)).into())
                .collect::<Items>(),
            ..self.0
        })
    }
}

impl From<AppBuilder> for App {
    fn from(value: AppBuilder) -> Self {
        value.0
    }
}
