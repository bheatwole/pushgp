use rand::{prelude::SliceRandom, Rng};
use std::convert::From;

use crate::Suit;

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Card {
    AceOfSpades = 0,
    TwoOfSpades,
    ThreeOfSpades,
    FourOfSpades,
    FiveOfSpades,
    SixOfSpades,
    SevenOfSpades,
    EightOfSpades,
    NineOfSpades,
    TenOfSpades,
    JackOfSpades,
    QueenOfSpades,
    KingOfSpades,
    AceOfDiamonds,
    TwoOfDiamonds,
    ThreeOfDiamonds,
    FourOfDiamonds,
    FiveOfDiamonds,
    SixOfDiamonds,
    SevenOfDiamonds,
    EightOfDiamonds,
    NineOfDiamonds,
    TenOfDiamonds,
    JackOfDiamonds,
    QueenOfDiamonds,
    KingOfDiamonds,
    AceOfClubs,
    TwoOfClubs,
    ThreeOfClubs,
    FourOfClubs,
    FiveOfClubs,
    SixOfClubs,
    SevenOfClubs,
    EightOfClubs,
    NineOfClubs,
    TenOfClubs,
    JackOfClubs,
    QueenOfClubs,
    KingOfClubs,
    AceOfHearts,
    TwoOfHearts,
    ThreeOfHearts,
    FourOfHearts,
    FiveOfHearts,
    SixOfHearts,
    SevenOfHearts,
    EightOfHearts,
    NineOfHearts,
    TenOfHearts,
    JackOfHearts,
    QueenOfHearts,
    KingOfHearts,
}

impl Card {
    pub fn suit(self) -> Suit {
        let self_int_value = self as u8;

        if self_int_value <= Card::KingOfSpades as u8 {
            Suit::Spades
        } else if self_int_value < Card::KingOfDiamonds as u8 {
            Suit::Diamonds
        } else if self_int_value < Card::KingOfClubs as u8 {
            Suit::Clubs
        } else {
            Suit::Hearts
        }
    }

    pub fn index_in_suit(self) -> usize {
        (self as u8 % 13) as usize
    }

    pub fn is_red(self) -> bool {
        self.suit().is_red()
    }

    pub fn is_black(self) -> bool {
        self.suit().is_black()
    }

    pub fn is_solitaire_play_legal(self, can_play_on_top_of: Card) -> bool {
        self.is_red() != can_play_on_top_of.is_red()
            && self.index_in_suit() + 1 == can_play_on_top_of.index_in_suit()
    }

    pub fn is_next_card_in_suit(&self, previous_card: Option<Card>) -> bool {
        if let Some(previous_card) = previous_card {
            self.suit() == previous_card.suit()
                && self.index_in_suit() + 1 == previous_card.index_in_suit()
        } else {
            self.index_in_suit() == 0
        }
    }

    pub fn make_deck() -> Vec<Card> {
        let mut deck = vec![];
        for i in 0u8..52 {
            deck.push(i.try_into().unwrap());
        }
        deck
    }

    pub fn make_shuffled_deck<R>(shuffles: usize, rng: &mut R) -> Vec<Card>
    where
        R: Rng + ?Sized,
    {
        let mut deck = Card::make_deck();
        for _ in 0..shuffles {
            deck.shuffle(rng);
        }
        deck
    }
}

impl From<u8> for Card {
    fn from(value: u8) -> Self {
        match value {
            x if x == Card::AceOfSpades as u8 => Card::AceOfSpades,
            x if x == Card::TwoOfSpades as u8 => Card::TwoOfSpades,
            x if x == Card::ThreeOfSpades as u8 => Card::ThreeOfSpades,
            x if x == Card::FourOfSpades as u8 => Card::FourOfSpades,
            x if x == Card::FiveOfSpades as u8 => Card::FiveOfSpades,
            x if x == Card::SixOfSpades as u8 => Card::SixOfSpades,
            x if x == Card::SevenOfSpades as u8 => Card::SevenOfSpades,
            x if x == Card::EightOfSpades as u8 => Card::EightOfSpades,
            x if x == Card::NineOfSpades as u8 => Card::NineOfSpades,
            x if x == Card::TenOfSpades as u8 => Card::TenOfSpades,
            x if x == Card::JackOfSpades as u8 => Card::JackOfSpades,
            x if x == Card::QueenOfSpades as u8 => Card::QueenOfSpades,
            x if x == Card::KingOfSpades as u8 => Card::KingOfSpades,
            x if x == Card::AceOfDiamonds as u8 => Card::AceOfDiamonds,
            x if x == Card::TwoOfDiamonds as u8 => Card::TwoOfDiamonds,
            x if x == Card::ThreeOfDiamonds as u8 => Card::ThreeOfDiamonds,
            x if x == Card::FourOfDiamonds as u8 => Card::FourOfDiamonds,
            x if x == Card::FiveOfDiamonds as u8 => Card::FiveOfDiamonds,
            x if x == Card::SixOfDiamonds as u8 => Card::SixOfDiamonds,
            x if x == Card::SevenOfDiamonds as u8 => Card::SevenOfDiamonds,
            x if x == Card::EightOfDiamonds as u8 => Card::EightOfDiamonds,
            x if x == Card::NineOfDiamonds as u8 => Card::NineOfDiamonds,
            x if x == Card::TenOfDiamonds as u8 => Card::TenOfDiamonds,
            x if x == Card::JackOfDiamonds as u8 => Card::JackOfDiamonds,
            x if x == Card::QueenOfDiamonds as u8 => Card::QueenOfDiamonds,
            x if x == Card::KingOfDiamonds as u8 => Card::KingOfDiamonds,
            x if x == Card::AceOfClubs as u8 => Card::AceOfClubs,
            x if x == Card::TwoOfClubs as u8 => Card::TwoOfClubs,
            x if x == Card::ThreeOfClubs as u8 => Card::ThreeOfClubs,
            x if x == Card::FourOfClubs as u8 => Card::FourOfClubs,
            x if x == Card::FiveOfClubs as u8 => Card::FiveOfClubs,
            x if x == Card::SixOfClubs as u8 => Card::SixOfClubs,
            x if x == Card::SevenOfClubs as u8 => Card::SevenOfClubs,
            x if x == Card::EightOfClubs as u8 => Card::EightOfClubs,
            x if x == Card::NineOfClubs as u8 => Card::NineOfClubs,
            x if x == Card::TenOfClubs as u8 => Card::TenOfClubs,
            x if x == Card::JackOfClubs as u8 => Card::JackOfClubs,
            x if x == Card::QueenOfClubs as u8 => Card::QueenOfClubs,
            x if x == Card::KingOfClubs as u8 => Card::KingOfClubs,
            x if x == Card::AceOfHearts as u8 => Card::AceOfHearts,
            x if x == Card::TwoOfHearts as u8 => Card::TwoOfHearts,
            x if x == Card::ThreeOfHearts as u8 => Card::ThreeOfHearts,
            x if x == Card::FourOfHearts as u8 => Card::FourOfHearts,
            x if x == Card::FiveOfHearts as u8 => Card::FiveOfHearts,
            x if x == Card::SixOfHearts as u8 => Card::SixOfHearts,
            x if x == Card::SevenOfHearts as u8 => Card::SevenOfHearts,
            x if x == Card::EightOfHearts as u8 => Card::EightOfHearts,
            x if x == Card::NineOfHearts as u8 => Card::NineOfHearts,
            x if x == Card::TenOfHearts as u8 => Card::TenOfHearts,
            x if x == Card::JackOfHearts as u8 => Card::JackOfHearts,
            x if x == Card::QueenOfHearts as u8 => Card::QueenOfHearts,
            x if x == Card::KingOfHearts as u8 => Card::KingOfHearts,
            _ => panic!("illegal value for card"),
        }
    }
}
