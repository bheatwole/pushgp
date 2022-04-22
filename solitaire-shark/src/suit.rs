#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Suit {
    Spades = 0,
    Diamonds,
    Clubs,
    Hearts,
}

impl Suit {
    pub fn is_red(&self) -> bool {
        match *self {
            Suit::Diamonds | Suit::Hearts => true,
            _ => false,
        }
    }

    pub fn is_black(&self) -> bool {
        !self.is_red()
    }
}