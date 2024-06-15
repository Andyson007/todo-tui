//! Functions that are associated with showing help menus

use std::{collections::HashMap, error::Error, fs::File, io::BufReader, path::Path};

/// Responsible for handling data in the help popup
#[derive(Debug)]
pub struct Help {
    /// All inputs parsed
    pub help_items: Vec<(Box<str>, Box<str>)>,
}

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
            .map(|(a, b)| (a.into_boxed_str(), b.into_boxed_str()))
            .collect::<Vec<_>>();
        ordered.sort_unstable_by(|(a, _), (b, _)| a.cmp(&b));
        Ok(Help {
            help_items: ordered,
        })
    }
}

/// Returns an ordered list how alike it is to
/// the search query
pub fn query<T>(iter: &Vec<T>, query: String) -> Vec<(usize, T)>
where
    T: Ord,
{
    vec![]
}
