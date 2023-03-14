#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl From<u8> for Color {
    fn from(item: u8) -> Self {
        if item == 0 {
            return Color::White;
        }
        Color::Black
    }
}

impl From<Color> for u8 {
    fn from(item: Color) -> Self {
        if item == Color::White {
            return 0;
        }
        1
    }
}
