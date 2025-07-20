use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
#[serde(untagged)]
pub enum Modifier {
    Single(String),
    List(Vec<String>),
}

impl Modifier {
    pub fn is_empty(&self) -> bool {
        match self {
            Modifier::Single(single) => single.is_empty(),
            Modifier::List(list) => list.is_empty(),
        }
    }
}

impl std::convert::From<Modifier> for Vec<String> {
    fn from(m: Modifier) -> Self {
        match m {
            Modifier::Single(modifier) => vec![modifier],
            Modifier::List(modifiers) => modifiers,
        }
    }
}

impl IntoIterator for &Modifier {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let ms = match self {
            Modifier::Single(m) => vec![m.clone()],
            Modifier::List(ms) => ms.clone(),
        };
        ms.into_iter()
    }
}

impl std::convert::From<Vec<String>> for Modifier {
    fn from(l: Vec<String>) -> Self {
        Self::List(l)
    }
}

impl std::convert::From<&str> for Modifier {
    fn from(m: &str) -> Self {
        Self::Single(m.to_owned())
    }
}

impl std::fmt::Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(modifier) => write!(f, "{modifier}"),
            Self::List(modifiers) => write!(f, "{}", modifiers.join("+")),
        }
    }
}

impl Modifier {
    pub fn sort_unstable(&mut self) {
        match self {
            Self::Single(_) => {}
            Self::List(modifiers) => modifiers.sort_unstable(),
        }
    }
}
