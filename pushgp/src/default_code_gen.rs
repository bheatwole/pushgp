use crate::*;

// This file contains all the bits that will need to be code generated by macros in order for this to work. For the
// default version, we simply hand-code everything and then work on the macros to make this happen.

// #[derive(Clone, Debug, Eq, Hash, PartialEq)]
// pub enum BaseLiteral {
//     Bool(Bool),
//     Float(Float),
//     Integer(Integer),
//     Name(Name),
// }

// impl std::fmt::Display for BaseLiteral {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match &self {
//             BaseLiteral::Bool(v) => v.nom_fmt(f),
//             BaseLiteral::Float(v) => v.nom_fmt(f),
//             BaseLiteral::Integer(v) => v.nom_fmt(f),
//             BaseLiteral::Name(v) => v.nom_fmt(f),
//         }
//     }
// }

// impl LiteralEnum<BaseLiteral> for BaseLiteral {
//     fn parse(input: &str) -> IResult<&str, BaseLiteral> {
//         if let Ok((rest, value)) = Bool::parse(input) {
//             return Ok((rest, BaseLiteral::Bool(value)));
//         }
//         if let Ok((rest, value)) = Float::parse(input) {
//             return Ok((rest, BaseLiteral::Float(value)));
//         }
//         if let Ok((rest, value)) = Integer::parse(input) {
//             return Ok((rest, BaseLiteral::Integer(value)));
//         }
//         if let Ok((rest, value)) = Name::parse(input) {
//             return Ok((rest, BaseLiteral::Name(value)));
//         }

//         Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Complete)))
//     }
// }

// impl LiteralEnumHasLiteralValue<BaseLiteral, Bool> for BaseLiteral {
//     fn supports_literal_type() -> bool {
//         true
//     }

//     fn make_from_value(value: Bool) -> BaseLiteral {
//         BaseLiteral::Bool(value)
//     }
// }

// impl LiteralEnumHasLiteralValue<BaseLiteral, Float> for BaseLiteral {
//     fn supports_literal_type() -> bool {
//         true
//     }

//     fn make_from_value(value: Float) -> BaseLiteral {
//         BaseLiteral::Float(value)
//     }
// }

// impl LiteralEnumHasLiteralValue<BaseLiteral, Integer> for BaseLiteral {
//     fn supports_literal_type() -> bool {
//         true
//     }

//     fn make_from_value(value: Integer) -> BaseLiteral {
//         BaseLiteral::Integer(value)
//     }
// }

// impl LiteralEnumHasLiteralValue<BaseLiteral, Name> for BaseLiteral {
//     fn supports_literal_type() -> bool {
//         true
//     }

//     fn make_from_value(value: Name) -> BaseLiteral {
//         BaseLiteral::Name(value)
//     }
// }

// impl EphemeralConfiguration<BaseLiteral> for BaseLiteral {
//     fn get_all_literal_types() -> Vec<String> {
//         vec!["Bool".to_owned(), "Float".to_owned(), "Integer".to_owned(), "Name".to_owned()]
//     }

//     fn make_literal_constructor_for_type(literal_type: &str) -> LiteralConstructor<BaseLiteral> {
//         match literal_type {
//             "Bool" => LiteralConstructor { 0: |rng| BaseLiteral::Bool(Bool::random_value(rng)) },
//             "Float" => LiteralConstructor { 0: |rng| BaseLiteral::Float(Float::random_value(rng)) },
//             "Integer" => LiteralConstructor { 0: |rng| BaseLiteral::Integer(Integer::random_value(rng)) },
//             "Name" => LiteralConstructor { 0: |rng| BaseLiteral::Name(Name::random_value(rng)) },
//             _ => panic!("unknown literal type"),
//         }
//     }
// }

pub fn new_virtual_table_with_all_instructions() -> VirtualTable {
    let mut virtual_table = VirtualTable::new();
    crate::execute_bool::BoolAnd::add_to_virtual_table(&mut virtual_table);
    crate::execute_bool::BoolDefine::add_to_virtual_table(&mut virtual_table);
    crate::execute_bool::BoolDup::add_to_virtual_table(&mut virtual_table);
    crate::execute_bool::BoolEqual::add_to_virtual_table(&mut virtual_table);
    crate::execute_bool::BoolFlush::add_to_virtual_table(&mut virtual_table);
    crate::execute_bool::BoolFromFloat::add_to_virtual_table(&mut virtual_table);
    crate::execute_bool::BoolFromInt::add_to_virtual_table(&mut virtual_table);
    crate::execute_bool::BoolNot::add_to_virtual_table(&mut virtual_table);
    crate::execute_bool::BoolOr::add_to_virtual_table(&mut virtual_table);
    crate::execute_bool::BoolPop::add_to_virtual_table(&mut virtual_table);
    crate::execute_bool::BoolRand::add_to_virtual_table(&mut virtual_table);
    crate::execute_bool::BoolRot::add_to_virtual_table(&mut virtual_table);
    crate::execute_bool::BoolShove::add_to_virtual_table(&mut virtual_table);
    crate::execute_bool::BoolStackDepth::add_to_virtual_table(&mut virtual_table);
    crate::execute_bool::BoolSwap::add_to_virtual_table(&mut virtual_table);
    crate::execute_bool::BoolYankDup::add_to_virtual_table(&mut virtual_table);
    crate::execute_bool::BoolYank::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeAppend::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeAtom::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeCar::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeCdr::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeCons::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeContainer::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeContains::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeDefine::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeDefinition::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeDiscrepancy::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeDoNCount::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeDoNRange::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeDoNTimes::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeDoN::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeDo::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeDup::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeEqual::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeExtract::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeFlush::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeFromBoolean::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeFromFloat::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeFromInteger::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeFromName::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeIf::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeInsert::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeLength::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeList::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeMember::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeNoop::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeNthCdr::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeNth::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeNull::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodePop::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodePosition::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeQuote::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeRand::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeRot::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeShove::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeSize::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeStackDepth::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeSubstitute::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeSwap::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeYankDup::add_to_virtual_table(&mut virtual_table);
    crate::execute_code::CodeYank::add_to_virtual_table(&mut virtual_table);
    crate::execute_exec::ExecDefine::add_to_virtual_table(&mut virtual_table);
    crate::execute_exec::ExecDoNCount::add_to_virtual_table(&mut virtual_table);
    crate::execute_exec::ExecDoNRange::add_to_virtual_table(&mut virtual_table);
    crate::execute_exec::ExecDoNTimes::add_to_virtual_table(&mut virtual_table);
    crate::execute_exec::ExecDup::add_to_virtual_table(&mut virtual_table);
    crate::execute_exec::ExecEqual::add_to_virtual_table(&mut virtual_table);
    crate::execute_exec::ExecFlush::add_to_virtual_table(&mut virtual_table);
    crate::execute_exec::ExecIf::add_to_virtual_table(&mut virtual_table);
    crate::execute_exec::ExecK::add_to_virtual_table(&mut virtual_table);
    crate::execute_exec::ExecPop::add_to_virtual_table(&mut virtual_table);
    crate::execute_exec::ExecRot::add_to_virtual_table(&mut virtual_table);
    crate::execute_exec::ExecShove::add_to_virtual_table(&mut virtual_table);
    crate::execute_exec::ExecStackDepth::add_to_virtual_table(&mut virtual_table);
    crate::execute_exec::ExecSwap::add_to_virtual_table(&mut virtual_table);
    crate::execute_exec::ExecS::add_to_virtual_table(&mut virtual_table);
    crate::execute_exec::ExecYankDup::add_to_virtual_table(&mut virtual_table);
    crate::execute_exec::ExecYank::add_to_virtual_table(&mut virtual_table);
    crate::execute_exec::ExecY::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatCos::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatDefine::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatDifference::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatDup::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatEqual::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatFlush::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatFromBoolean::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatFromInteger::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatGreater::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatLess::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatMax::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatMin::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatModulo::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatPop::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatProduct::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatQuotient::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatRand::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatRot::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatShove::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatSin::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatStackDepth::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatSum::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatSwap::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatTan::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatYankDup::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatYank::add_to_virtual_table(&mut virtual_table);
    crate::execute_integer::IntegerDefine::add_to_virtual_table(&mut virtual_table);
    crate::execute_integer::IntegerDifference::add_to_virtual_table(&mut virtual_table);
    crate::execute_integer::IntegerDup::add_to_virtual_table(&mut virtual_table);
    crate::execute_integer::IntegerEqual::add_to_virtual_table(&mut virtual_table);
    crate::execute_integer::IntegerFlush::add_to_virtual_table(&mut virtual_table);
    crate::execute_integer::IntegerFromBoolean::add_to_virtual_table(&mut virtual_table);
    crate::execute_integer::IntegerFromFloat::add_to_virtual_table(&mut virtual_table);
    crate::execute_integer::IntegerGreater::add_to_virtual_table(&mut virtual_table);
    crate::execute_integer::IntegerLess::add_to_virtual_table(&mut virtual_table);
    crate::execute_integer::IntegerMax::add_to_virtual_table(&mut virtual_table);
    crate::execute_integer::IntegerMin::add_to_virtual_table(&mut virtual_table);
    crate::execute_integer::IntegerModulo::add_to_virtual_table(&mut virtual_table);
    crate::execute_integer::IntegerPop::add_to_virtual_table(&mut virtual_table);
    crate::execute_integer::IntegerProduct::add_to_virtual_table(&mut virtual_table);
    crate::execute_integer::IntegerQuotient::add_to_virtual_table(&mut virtual_table);
    crate::execute_integer::IntegerRand::add_to_virtual_table(&mut virtual_table);
    crate::execute_integer::IntegerRot::add_to_virtual_table(&mut virtual_table);
    crate::execute_integer::IntegerShove::add_to_virtual_table(&mut virtual_table);
    crate::execute_integer::IntegerStackDepth::add_to_virtual_table(&mut virtual_table);
    crate::execute_integer::IntegerSum::add_to_virtual_table(&mut virtual_table);
    crate::execute_integer::IntegerSwap::add_to_virtual_table(&mut virtual_table);
    crate::execute_integer::IntegerYankDup::add_to_virtual_table(&mut virtual_table);
    crate::execute_integer::IntegerYank::add_to_virtual_table(&mut virtual_table);
    crate::execute_name::NameDup::add_to_virtual_table(&mut virtual_table);
    crate::execute_name::NameEqual::add_to_virtual_table(&mut virtual_table);
    crate::execute_name::NameFlush::add_to_virtual_table(&mut virtual_table);
    crate::execute_name::NamePop::add_to_virtual_table(&mut virtual_table);
    crate::execute_name::NameQuote::add_to_virtual_table(&mut virtual_table);
    crate::execute_name::NameRandBoundName::add_to_virtual_table(&mut virtual_table);
    crate::execute_name::NameRand::add_to_virtual_table(&mut virtual_table);
    crate::execute_name::NameRot::add_to_virtual_table(&mut virtual_table);
    crate::execute_name::NameShove::add_to_virtual_table(&mut virtual_table);
    crate::execute_name::NameStackDepth::add_to_virtual_table(&mut virtual_table);
    crate::execute_name::NameSwap::add_to_virtual_table(&mut virtual_table);
    crate::execute_name::NameYankDup::add_to_virtual_table(&mut virtual_table);
    crate::execute_name::NameYank::add_to_virtual_table(&mut virtual_table);

    // These must be last, with Name the very last of all
    crate::execute_bool::BoolLiteralValue::add_to_virtual_table(&mut virtual_table);
    crate::execute_float::FloatLiteralValue::add_to_virtual_table(&mut virtual_table);
    crate::execute_integer::IntegerLiteralValue::add_to_virtual_table(&mut virtual_table);
    crate::execute_name::NameLiteralValue::add_to_virtual_table(&mut virtual_table);

    virtual_table
}
// impl InstructionConfiguration for BaseLiteral {
//     fn get_all_instructions() -> Vec<String> {
//         vec![
//             crate::execute_bool::BoolAnd::<BaseContext, BaseLiteral>::name().to_owned(),
//             crate::execute_bool::BoolDefine::<BaseContext, BaseLiteral>::name().to_owned(),
//             crate::execute_bool::BoolDup::<BaseContext, BaseLiteral>::name().to_owned(),
//         ]
//     }
// // }

// pub struct BaseLiteralParser {}
// impl Parser<BaseLiteral> for BaseLiteralParser {
//     fn parse_code_instruction(input: &str) -> IResult<&str, Code<BaseLiteral>> {
//         use nom::{branch::alt, bytes::complete::tag};
//         let (input, instruction) = alt((
//             tag(crate::execute_bool::BoolAnd::<BaseContext, BaseLiteral>::name()),
//             tag(crate::execute_bool::BoolDefine::<BaseContext, BaseLiteral>::name()),
//             tag(crate::execute_bool::BoolDup::<BaseContext, BaseLiteral>::name()),
//         ))(input)?;
//         let (input, _) = crate::parse::space_or_end(input)?;

//         Ok((input, Code::Instruction(instruction.to_owned())))
//     }
// }

// // pub fn new_instruction_table_with_all_instructions<C, L>() -> InstructionTable<C>
// where
//     C: Context + ContextHasBoolStack<L> + ContextHasNameStack<L>,
//     L: LiteralEnum<L>,
// {
//     let mut instructions = InstructionTable::new();
//     crate::execute_bool::BoolAnd::<C, L>::add_to_table(&mut instructions);
//     crate::execute_bool::BoolDefine::<C, L>::add_to_table(&mut instructions);
//     crate::execute_bool::BoolDup::<C, L>::add_to_table(&mut instructions);

//     instructions
// }
