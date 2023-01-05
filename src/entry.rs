use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum EntryType {
    Article,
    Book,
}

impl FromStr for EntryType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "article" => Ok(Self::Article),
            "book" => Ok(Self::Book),
            _ => Err(()),
        }
    }
}
