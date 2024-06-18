use std::error::Error;

use crate::{
    help::{self},
    parse::todo::{Item, Items},
};

#[derive(Debug, Default)]
pub struct StaticInfo {
    pub help: Items<help::Item>,
    /// All selectable options
    pub options: Items<Item>,
}

impl StaticInfo {
    pub fn from<P>(menu: P, help: P) -> Result<StaticInfo, Box<dyn Error>> {
        todo!()
    }
}
