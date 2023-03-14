use crate::{color::Color, game_error::GameError};
use bitfield_struct::bitfield;
use std::convert::From;
use std::fmt;
use std::str::FromStr;

#[bitfield(u8)]
#[derive(PartialEq, Eq)]
pub struct Bug {
    /// The first field occupies the least significant bits
    #[bits(1)]
    pub color: Color,
    #[bits(3)]
    pub kind: Kind,
    #[bits(2)]
    pub number: usize,
    /// we need to fill the u8
    #[bits(2)]
    _padding: usize,
}

impl FromStr for Bug {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(c_chars) = s.chars().next() {
            let color: Color = c_chars.to_string().parse()?;
            if let Some(b_chars) = s.chars().nth(1) {
                let kind: Kind = b_chars.to_string().parse()?;
                let mut bug = Bug::new().with_kind(kind).with_color(color);
                if let Some(ch) = s.chars().nth(2) {
                    if let Ok(num) = ch.to_string().parse() {
                        bug.set_number(num);
                    }
                }
                return Ok(bug);
            }
        }
        Err(GameError::ParsingError {
            found: s.to_string(),
            typ: "piece".to_string(),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Kind {
    Ant = 0,
    Beetle = 1,
    Grasshopper = 2,
    Ladybug = 3,
    Mosquito = 4,
    Pillbug = 5,
    Queen = 6,
    Spider = 7,
}

impl Kind {
    pub fn name(&self) -> &'static str {
        match self {
            Kind::Ant => "Ant",
            Kind::Beetle => "Beetle",
            Kind::Grasshopper => "Grasshopper",
            Kind::Ladybug => "Ladybug",
            Kind::Mosquito => "Mosquito",
            Kind::Pillbug => "Pillbug",
            Kind::Queen => "Queen",
            Kind::Spider => "Spider",
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name()[0..=0].to_string())
    }
}

impl FromStr for Kind {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "a" => Ok(Kind::Ant),
            "B" | "b" => Ok(Kind::Beetle),
            "G" | "g" => Ok(Kind::Grasshopper),
            "L" | "l" => Ok(Kind::Ladybug),
            "M" | "m" => Ok(Kind::Mosquito),
            "P" | "p" => Ok(Kind::Pillbug),
            "Q" | "q" => Ok(Kind::Queen),
            "S" | "s" => Ok(Kind::Spider),
            any => Err(GameError::ParsingError {
                found: any.to_string(),
                typ: "bug string".to_string(),
            }),
        }
    }
}

impl From<u8> for Kind {
    fn from(item: u8) -> Self {
        match item {
            0 => Kind::Ant,
            1 => Kind::Beetle,
            2 => Kind::Grasshopper,
            3 => Kind::Ladybug,
            4 => Kind::Mosquito,
            5 => Kind::Pillbug,
            6 => Kind::Queen,
            7 => Kind::Spider,
            _ => panic!(),
        }
    }
}

impl From<Kind> for u8 {
    fn from(kind: Kind) -> Self {
        kind as u8
    }
}
