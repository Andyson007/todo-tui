//! Functions that are associated with showing help menus

use std::{collections::HashMap, error::Error, fs::File, io::BufReader, path::Path};

use crate::Score;

/// Responsible for handling data in the help popup
#[derive(Debug, Clone)]
pub struct Help(pub Vec<HelpItem>);

/// A helpdata wrapper for Ord implementations
#[derive(Debug, Clone)]
pub struct HelpItem(pub (Box<str>, Box<str>));

impl Help {
    /// Parses a File containing help messages
    pub fn parse<P>(path: P) -> Result<Self, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let parsed: HashMap<String, String> = serde_json::from_reader(reader)?;
        let mut ordered = parsed
            .into_iter()
            .map(|(a, b)| HelpItem((a.into_boxed_str(), b.into_boxed_str())))
            .collect::<Vec<_>>();
        ordered.sort_unstable_by(|a, b| a.0 .0.cmp(&b.0 .0));
        Ok(Help(ordered))
    }
}

impl Score for HelpItem {
    fn score(&self, query: &str) -> Option<i64> {
        (self.0 .0.to_string() + " " + self.0 .1.as_ref()).score(query)
    }
}
