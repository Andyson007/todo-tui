//! Infarmation related to staring things that won't get modified regularly
use std::{collections::HashMap, error::Error, fs::File, io::BufReader, path::Path};

use serde::Deserialize;

use crate::{
    help,
    parse::todo::{self, Items},
};

/// The main struct of this module
#[derive(Debug, Default)]
pub struct StaticInfo {
    /// All help entries
    pub help: Items<help::Item>,
    /// All selectable options
    pub lists: HashMap<String, Items<todo::Item>>,
}

impl StaticInfo {
    /// Creates a `StaticInfo` from two file paths
    ///
    /// # Errors
    /// File not found
    pub fn from<P>(lists: P, help: P) -> Result<Self, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        Ok(Self {
            help: help::parse(help)?,
            lists: parse(lists)?,
        })
    }

    /// gets a list from static info (removes it too)
    pub fn get<T>(&mut self, query: T) -> Option<Items<todo::Item>>
    where
        T: Into<String>,
    {
        self.lists.remove(&query.into())
    }
}

fn parse<P>(path: P) -> Result<HashMap<String, Items<todo::Item>>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let parsed: HashMap<String, Vec<Item>> = serde_json::from_reader(reader)?;
    Ok(parsed
        .into_iter()
        .map(|(x, y)| {
            (
                x,
                y.into_iter()
                    .map(std::convert::Into::into)
                    .collect::<Items<todo::Item>>(),
            )
        })
        .collect::<HashMap<String, Items<todo::Item>>>())
}

#[derive(Debug, Deserialize)]
struct Item {
    title: String,
    description: String,
}

impl From<Item> for todo::Item {
    fn from(value: Item) -> Self {
        Self {
            title: value.title.into_boxed_str(),
            description: value.description.into_boxed_str(),
            description_scroll: 0,
        }
    }
}
