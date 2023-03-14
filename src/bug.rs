use crate::color::Color;
use bitfield_struct::bitfield;
use std::convert::From;

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
    fn from(item: Kind) -> Self {
        match item {
            Kind::Ant => 0,
            Kind::Beetle => 1,
            Kind::Grasshopper => 2,
            Kind::Ladybug => 3,
            Kind::Mosquito => 4,
            Kind::Pillbug => 5,
            Kind::Queen => 6,
            Kind::Spider => 7,
        }
    }
}
