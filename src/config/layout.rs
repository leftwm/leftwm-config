use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    MainAndVertStack,
    MainAndHorizontalStack,
    MainAndDeck,
    GridHorizontal,
    EvenHorizontal,
    EvenVertical,
    Fibonacci,
    LeftMain,
    CenterMain,
    CenterMainBalanced,
    CenterMainFluid,
    Monocle,
    RightWiderLeftStack,
    LeftWiderRightStack,
}

//pub const LAYOUTS: &[Layout] = &[
//    Layout::MainAndVertStack,
//    Layout::MainAndHorizontalStack,
//    Layout::MainAndDeck,
//    Layout::GridHorizontal,
//    Layout::EvenHorizontal,
//    Layout::EvenVertical,
//    Layout::Fibonacci,
//    Layout::LeftMain,
//    Layout::CenterMain,
//    Layout::CenterMainBalanced,
//    Layout::CenterMainFluid,
//    Layout::Monocle,
//    Layout::RightWiderLeftStack,
//    Layout::LeftWiderRightStack,
//];

impl Default for Layout {
    fn default() -> Self {
        Self::MainAndVertStack
    }
}

#[derive(Debug, Error)]
#[error("Could not parse layout: {0}")]
pub struct ParseLayoutError(String);

impl FromStr for Layout {
    type Err = ParseLayoutError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MainAndVertStack" => Ok(Self::MainAndVertStack),
            "MainAndHorizontalStack" => Ok(Self::MainAndHorizontalStack),
            "MainAndDeck" => Ok(Self::MainAndDeck),
            "GridHorizontal" => Ok(Self::GridHorizontal),
            "EvenHorizontal" => Ok(Self::EvenHorizontal),
            "EvenVertical" => Ok(Self::EvenVertical),
            "Fibonacci" => Ok(Self::Fibonacci),
            "LeftMain" => Ok(Self::LeftMain),
            "CenterMain" => Ok(Self::CenterMain),
            "CenterMainBalanced" => Ok(Self::CenterMainBalanced),
            "CenterMainFluid" => Ok(Self::CenterMainFluid),
            "Monocle" => Ok(Self::Monocle),
            "RightWiderLeftStack" => Ok(Self::RightWiderLeftStack),
            "LeftWiderRightStack" => Ok(Self::LeftWiderRightStack),
            _ => Err(ParseLayoutError(s.to_string())),
        }
    }
}
