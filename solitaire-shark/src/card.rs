use crate::VirtualMachineMustHaveGame;
use nom::{branch::alt, bytes::complete::tag};
use pushgp::*;
use pushgp_macros::stack_instruction;
use rand::{prelude::SliceRandom, Rng};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter, EnumString, FromRepr};

use crate::Suit;

#[derive(
    AsRefStr,
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    EnumString,
    EnumIter,
    FromRepr,
    strum_macros::Display,
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

impl Into<Data> for Card {
    fn into(self) -> Data {
        Data::UnsignedInteger(self as u64)
    }
}

trait DataToCard {
    fn card_value(&self) -> Card;
}

impl DataToCard for Data {
    fn card_value(&self) -> Card {
        match self {
            Data::UnsignedInteger(value) => Card::from_repr(*value as u8).unwrap(),
            _ => panic!(
                "card_value called for Data that does not have a unsigned integer value stored"
            ),
        }
    }
}

pub trait VirtualMachineMustHaveCard<Vm> {
    fn card(&mut self) -> &mut Stack<Card>;
}

pub struct CardLiteralValue {}

impl CardLiteralValue {
    pub fn new_code<Oc: OpcodeConvertor>(oc: &Oc, value: Card) -> Code {
        let opcode = oc.opcode_for_name(Self::static_name()).unwrap();
        Code::new(opcode, value.into())
    }
}

impl StaticName for CardLiteralValue {
    fn static_name() -> &'static str {
        "CARD.LITERALVALUE"
    }
}

impl<Vm: VirtualMachine + VirtualMachineMustHaveCard<Vm>> Instruction<Vm> for CardLiteralValue {
    fn parse(input: &str, opcode: Opcode) -> nom::IResult<&str, Code> {
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

        let card: Card = Card::from_str(card_name).unwrap();
        Ok((rest, Code::new(opcode, card.into())))
    }

    fn fmt(
        f: &mut std::fmt::Formatter<'_>,
        code: &Code,
        _vtable: &InstructionTable<Vm>,
    ) -> std::fmt::Result {
        write!(f, "{}", code.get_data().card_value())
    }

    /// Executing a CardLiteralValue pushes the literal value that was part of the data onto the stack
    fn execute(code: Code, vm: &mut Vm) {
        vm.card().push(code.get_data().card_value())
    }

    fn random_value(engine: &mut VirtualMachineEngine<Vm>) -> Code {
        let value = engine
            .get_rng()
            .gen_range((Card::AceOfSpades as u8)..=(Card::KingOfHearts as u8));
        CardLiteralValue::new_code(engine, Card::from_repr(value).unwrap())
    }
}

/// Pops the Card stack and pushes TRUE onto the Bool stack if that Card is the next one to go on the Finished Pile
#[stack_instruction(Card)]
fn ready_to_finish(vm: &mut Vm, value: Card) {
    let ready = vm.game().card_is_ready_to_finish(value);
    vm.bool().push(ready);
}

/// Draws the next three cards (if available) from the draw pile onto the play pile. If the draw pile is empty, the play
/// pile is recycled. Push the new top card of the play pile onto the Card stack
#[stack_instruction(Card)]
fn draw_next_three(vm: &mut Vm) {
    vm.game().draw_next_three();
    if let Some(top_card_of_play_pile) = vm.game().top_card_of_play_pile() {
        vm.card().push(top_card_of_play_pile);
    }
}

/// Moves the top play pile card to the appropriate finish pile. Pushes whether or not the action could be completed
/// onto the Bool stack
#[stack_instruction(Card)]
fn move_top_play_pile_card_to_finish(vm: &mut Vm) {
    let success = vm.game().move_top_play_pile_card_to_finish();
    vm.bool().push(success);
}

/// Pops the Integer stack and uses that value modulus 7 to choose a work pile. The top card of that work pile is moved
/// to the finish pile if possible. Pushes whether or not the action could be completed onto the Bool stack
#[stack_instruction(Card)]
fn move_top_work_pile_card_to_finish(vm: &mut Vm, work_pile: Integer) {
    let work_pile = mod_for_vec_index(work_pile, 7);
    let success = vm
        .game()
        .move_top_work_pile_card_to_finish(work_pile as usize);
    vm.bool().push(success);
}

/// Pops the Integer stack three times. The top value is the number of cards to move. The second value is the index of
/// the work pile to move from (modulus 7). The third value is the destination work pile (modulus 7).  Pushes whether or
/// not the action could be completed onto the Bool stack
#[stack_instruction(Card)]
fn move_work_pile_cards_to_another_work_pile(
    vm: &mut Vm,
    card_count: Integer,
    source_pile: Integer,
    destination_pile: Integer,
) {
    let source_pile = mod_for_vec_index(source_pile, 7) as usize;
    let destination_pile = mod_for_vec_index(destination_pile, 7) as usize;
    let face_up_count = vm.game().number_of_face_up_cards_in_work_pile(source_pile);
    let success = if face_up_count > 0 {
        let card_count = mod_for_vec_index(card_count, face_up_count as i64) as usize;
        if card_count > 0 {
            vm.game().move_work_pile_cards_to_another_work_pile(
                source_pile,
                card_count,
                destination_pile,
            )
        } else {
            false
        }
    } else {
        false
    };
    vm.bool().push(success);
}

#[stack_instruction(Card)]
fn draw_pile_len(vm: &mut Vm) {
    let len = vm.game().number_of_cards_in_draw_pile();
    vm.integer().push(len as i64);
}

#[stack_instruction(Card)]
fn play_pile_len(vm: &mut Vm) {
    let len = vm.game().number_of_cards_in_play_pile();
    vm.integer().push(len as i64);
}

#[stack_instruction(Card)]
fn top_play_pile(vm: &mut Vm) {
    if let Some(card) = vm.game().top_card_of_play_pile() {
        vm.card().push(card);
    }
}

/// Defines the name on top of the NAME stack as an instruction that will push the top item of the CARD stack
/// onto the EXEC stack.
#[stack_instruction(Card)]
fn define(vm: &mut Vm, value: Card, name: Name) {
    let code = CardLiteralValue::new_code(vm, value);
    vm.engine_mut().define_name(name, code);
}

/// Duplicates the top item on the CARD stack. Does not pop its argument (which, if it did, would negate the
/// effect of the duplication!).
#[stack_instruction(Card)]
fn dup(vm: &mut Vm) {
    vm.card().duplicate_top_item();
}

/// Pushes TRUE if the top two items on the CARD stack are equal, or FALSE otherwise.
#[stack_instruction(Card)]
fn equal(vm: &mut Vm, a: Card, b: Card) {
    vm.bool().push(a == b);
}

/// Empties the Card stack.
#[stack_instruction(Card)]
fn flush(vm: &mut Vm) {
    vm.card().clear();
}

/// Pops the top INTEGER and determines which Card it is (0..52) pushing the result onto the CARD stack. The integer
/// is taken modulus 52 so that it is always a valid Card
#[stack_instruction(Card)]
fn from_int(vm: &mut Vm, value: Integer) {
    let value = mod_for_vec_index(value, 52) as u8;
    vm.card().push(Card::from_repr(value).unwrap());
}

/// Pops the CARD stack
#[stack_instruction(Card)]
fn pop(vm: &mut Vm, _a: Card) {}

/// Pushes a random Card onto the CARD stack
#[stack_instruction(Card)]
fn rand(vm: &mut Vm) {
    let random_value = vm.random_value::<CardLiteralValue>();
    vm.execute_immediate::<CardLiteralValue>(random_value);
}

// "CARD.ROT"

// "CARD.SHOVE"

// "CARD.STACKDEPTH"

// "CARD.SWAP"

// "CARD.YANKDUP"

// "CARD.YANK"

// We cannot always verify that the result of % will be a positive number. This takes care of that
fn mod_for_vec_index(dividend: i64, divisor: i64) -> i64 {
    (dividend % divisor).saturating_abs()
}
