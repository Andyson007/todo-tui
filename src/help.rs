//! Functions that are associated with showing help menus

use std::{collections::HashMap, error::Error, fs::File, io::BufReader, path::Path};

use crate::{
    parse::{todo::Items, ListItem},
    Score,
};

/// A helpdata wrapper for Ord implementations
#[derive(Debug, Clone, Default)]
pub struct Item(pub (Box<str>, Box<str>));

/// Parses a File containing help messages
///
/// # Errors
/// 1. An io error due to failing to open the file at the specified path
/// 2. Failing to serialize that file
pub fn parse<P>(path: P) -> Result<Items<Item>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let parsed: HashMap<String, String> = serde_json::from_reader(reader)?;
    Ok(parsed
        .into_iter()
        .map(|(a, b)| Item((a.into_boxed_str(), b.into_boxed_str())))
        .collect())
}

impl Score for Item {
    fn score(&self, query: &str) -> Option<i64> {
        (self.0 .0.to_string() + " " + self.0 .1.as_ref()).score(query)
    }
}

impl ListItem for Item {
    fn title(&self) -> String {
        self.0 .0.to_string()
    }

    fn description(&self) -> String {
        self.0 .1.to_string()
    }
}
