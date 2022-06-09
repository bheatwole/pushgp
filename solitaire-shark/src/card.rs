use nom::{branch::alt, bytes::complete::tag, IResult};
use pushgp::*;
use pushgp_macros::instruction;
use rand::{prelude::SliceRandom, Rng};
use std::convert::From;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter, EnumString, FromRepr};

use crate::Suit;

#[derive(
    AsRefStr, Copy, Clone, Debug, Eq, PartialEq, EnumString, EnumIter, FromRepr, strum_macros::Display,
)]
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
        } else if self_int_value <= Card::KingOfDiamonds as u8 {
            Suit::Diamonds
        } else if self_int_value <= Card::KingOfClubs as u8 {
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
        for i in Card::iter() {
            deck.push(i);
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

pub trait MustHaveCardStackInContext {
    fn card(&self) -> Stack<Card>;
    fn make_literal_card(&self, value: Card) -> Code;
}

impl<State: std::fmt::Debug + Clone> MustHaveCardStackInContext for Context<State> {
    fn card(&self) -> Stack<Card> {
        Stack::<Card>::new(self.get_stack("Card"))
    }

    fn make_literal_card(&self, value: Card) -> Code {
        let id = self
            .get_virtual_table()
            .id_for_name(CardLiteralValue::name())
            .unwrap();
        Code::InstructionWithData(id, Some(InstructionData::from_u8(value as u8)))
    }
}

impl From<InstructionData> for Card {
    fn from(data: InstructionData) -> Self {
        Card::from_repr(data.get_u8().unwrap()).unwrap()
    }
}

impl From<&InstructionData> for Card {
    fn from(data: &InstructionData) -> Self {
        Card::from_repr(data.get_u8().unwrap()).unwrap()
    }
}

impl Into<InstructionData> for Card {
    fn into(self) -> InstructionData {
        InstructionData::from_u8(self as u8)
    }
}

pub struct CardLiteralValue {}
impl Instruction for CardLiteralValue {
    /// Every instruction must have a name
    fn name() -> &'static str {
        "CARD.LITERALVALUE"
    }

    /// All instructions must be parsable by 'nom' from a string. Parsing an instruction will either return an error to
    /// indicate the instruction was not found, or the optional data, indicating the instruction was found and parsing
    /// should cease.
    fn parse(input: &str) -> IResult<&str, Option<InstructionData>> {
        let (rest, card_name) = alt((
            alt((
                tag("AceOfSpades"),
                tag("TwoOfSpades"),
                tag("ThreeOfSpades"),
                tag("FourOfSpades"),
                tag("FiveOfSpades"),
                tag("SixOfSpades"),
                tag("SevenOfSpades"),
                tag("EightOfSpades"),
                tag("NineOfSpades"),
                tag("TenOfSpades"),
                tag("JackOfSpades"),
                tag("QueenOfSpades"),
                tag("KingOfSpades"),
            )),
            alt((
                tag("AceOfDiamonds"),
                tag("TwoOfDiamonds"),
                tag("ThreeOfDiamonds"),
                tag("FourOfDiamonds"),
                tag("FiveOfDiamonds"),
                tag("SixOfDiamonds"),
                tag("SevenOfDiamonds"),
                tag("EightOfDiamonds"),
                tag("NineOfDiamonds"),
                tag("TenOfDiamonds"),
                tag("JackOfDiamonds"),
                tag("QueenOfDiamonds"),
                tag("KingOfDiamonds"),
            )),
            alt((
                tag("AceOfClubs"),
                tag("TwoOfClubs"),
                tag("ThreeOfClubs"),
                tag("FourOfClubs"),
                tag("FiveOfClubs"),
                tag("SixOfClubs"),
                tag("SevenOfClubs"),
                tag("EightOfClubs"),
                tag("NineOfClubs"),
                tag("TenOfClubs"),
                tag("JackOfClubs"),
                tag("QueenOfClubs"),
                tag("KingOfClubs"),
            )),
            alt((
                tag("AceOfHearts"),
                tag("TwoOfHearts"),
                tag("ThreeOfHearts"),
                tag("FourOfHearts"),
                tag("FiveOfHearts"),
                tag("SixOfHearts"),
                tag("SevenOfHearts"),
                tag("EightOfHearts"),
                tag("NineOfHearts"),
                tag("TenOfHearts"),
                tag("JackOfHearts"),
                tag("QueenOfHearts"),
                tag("KingOfHearts"),
            )),
        ))(input)?;
        Ok((rest, Some(Card::from_str(card_name).unwrap().into())))
    }

    /// All instructions must also be able to write to a string that can later be parsed by nom.
    fn nom_fmt(
        data: &Option<InstructionData>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", Card::from(data.as_ref().unwrap()))
    }

    /// If the instruction makes use of InstructionData, it must be able to generate a random value for code generation.
    /// If it does not use InstructionData, it just returns None
    fn random_value(rng: &mut rand::rngs::SmallRng) -> Option<InstructionData> {
        let value = rng.gen_range((Card::AceOfSpades as u8)..=(Card::KingOfHearts as u8));
        Some(InstructionData::from_u8(value))
    }

    /// Instructions are pure functions on a Context and optional InstructionData. All parameters are read from the
    /// Context and/or data and all outputs are updates to the Context.
    fn execute<State: std::fmt::Debug + Clone>(
        context: &Context<State>,
        data: Option<InstructionData>,
    ) {
        if let Some(stack) = context.get_stack("Card") {
            stack.push(data.unwrap());
        }
    }

    fn add_to_virtual_table<State: std::fmt::Debug + Clone>(table: &mut VirtualTable<State>) {
        table.add_entry(
            Self::name(),
            Self::parse,
            Self::nom_fmt,
            Self::random_value,
            Self::execute,
        );
    }
}

// /// Pops the Card stack and pushes TRUE onto the Bool stack if that Card is the next one to go on the Finished Pile
// "CARD.READYTOFINISH"

// /// Pops the Card stack twice to determine if the first Card popped can be played on the second card via a Solitaire
// /// move (opposite color and one higher in rank)
// "CARD.ISLEGALMOVE"

// /// Pops the Card stack and pushes the associate FinishedPile for the Card onto the Pile stack.
// "CARD.FINISHPILE"

instruction! {
    /// Defines the name on top of the NAME stack as an instruction that will push the top item of the CARD stack
    /// onto the EXEC stack.
    #[stack(Card)]
    fn define(context: &mut Context, value: Card, name: Name) {
        context.define_name(name, context.make_literal_card(value));
    }
}

instruction! {
    /// Duplicates the top item on the CARD stack. Does not pop its argument (which, if it did, would negate the
    /// effect of the duplication!).
    #[stack(Card)]
    fn dup(context: &mut Context) {
        context.card().duplicate_top_item();
    }
}

instruction! {
    /// Pushes TRUE if the top two items on the CARD stack are equal, or FALSE otherwise.
    #[stack(Card)]
    fn equal(context: &mut Context, a: Card, b: Card) {
        context.bool().push(a == b);
    }
}

instruction! {
    /// Empties the Card stack.
    #[stack(Card)]
    fn flush(context: &mut Context) {
        context.card().clear();
    }
}

instruction! {
    /// Pops the top INTEGER and determines which Card it is (0..52) pushing the result onto the CARD stack. The integer
    /// is taken modulus 52 so that it is always a valid Card
    #[stack(Card)]
    fn from_int(context: &mut Context, value: Integer) {
        let value = (value % 52) as u8;
        context.card().push(Card::from_repr(value).unwrap());
    }
}

instruction! {
    /// Pops the CARD stack
    #[stack(Card)]
    fn pop(context: &mut Context, _a: Card) {}
}

instruction! {
    /// Pushes a random Card onto the CARD stack
    #[stack(Card)]
    fn rand(context: &mut Context) {
        let random_card = context.run_random_function(CardLiteralValue::random_value).unwrap();
        if let Some(stack) = context.get_stack("Card") {
            stack.push(random_card);
        }
    }
}

// "CARD.ROT"

// "CARD.SHOVE"

// "CARD.STACKDEPTH"

// "CARD.SWAP"

// "CARD.YANKDUP"

// "CARD.YANK"
