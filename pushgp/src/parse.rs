use crate::{Code, VirtualTable};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, none_of, space0, space1},
    combinator::{eof, opt},
    multi::{many0, many1},
    IResult,
};
use rust_decimal::{prelude::FromPrimitive, Decimal};

pub(crate) fn parse(virtual_table: &VirtualTable, input: &str) -> Code {
    parse_one_code(virtual_table, input).unwrap().1
}

fn parse_one_code<'a>(virtual_table: &'a VirtualTable, input: &'a str) -> IResult<&'a str, Code> {
    match virtual_table.call_parse(input) {
        Ok((rest, code)) => return Ok((rest, code)),
        Err(_) => parse_code_list(virtual_table, input),
    }
}

fn parse_code_list<'a>(virtual_table: &'a VirtualTable, input: &'a str) -> IResult<&'a str, Code> {
    let parse_one = |fn_input| parse_one_code(virtual_table, fn_input);

    // A list is a start tag, zero or more codes and an end tag
    let (input, _) = start_list(input)?;
    let (input, codes) = many0(parse_one)(input)?;
    let (input, _) = end_list(input)?;
    Ok((input, Code::List(codes)))
}

fn start_list(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("( ")(input)?;
    Ok((input, ()))
}

fn end_list(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag(")")(input)?;
    let (input, _) = space0(input)?;
    Ok((input, ()))
}

pub fn space_or_end(input: &str) -> IResult<&str, ()> {
    let (input, _) = alt((space1, eof))(input)?;
    Ok((input, ()))
}

pub fn parse_code_bool(input: &str) -> IResult<&str, bool> {
    let (input, text_value) = alt((tag("TRUE"), tag("FALSE")))(input)?;
    let (input, _) = space_or_end(input)?;

    Ok((
        input,
        match text_value {
            "TRUE" => true,
            "FALSE" => false,
            _ => panic!("can't get here"),
        },
    ))
}

pub fn parse_code_float(input: &str) -> IResult<&str, Decimal> {
    // A float MAY start with a sign
    let (input, opt_sign) = opt(alt((char('+'), char('-'))))(input)?;

    // It MUST have a decimal point and digits before and after
    let (input, whole) = digit1(input)?;
    let (input, _) = char('.')(input)?;
    let (input, fractional) = digit1(input)?;

    // It MAY have an exponent
    let (input, opt_exponent) = opt(parse_exponent)(input)?;

    // It MAY have some trailing spaces
    let (input, _) = space0(input)?;

    // Put the whole thing back into a string
    let float_string =
        format!("{}{}.{}{}", opt_sign.unwrap_or('+'), whole, fractional, opt_exponent.unwrap_or("".to_owned()));

    // Parse it
    match float_string.parse::<f64>() {
        Ok(float_value) => Ok((input, Decimal::from_f64(float_value).unwrap())),
        Err(_) => Err(nom::Err::Error(nom::error::make_error(input, nom::error::ErrorKind::Verify))),
    }
}

fn parse_exponent(input: &str) -> IResult<&str, String> {
    // The exponent MUST start with an E or e
    let (input, _) = alt((char('e'), char('E')))(input)?;

    // It MAY have a sign
    let (input, opt_sign) = opt(alt((char('+'), char('-'))))(input)?;

    // It MUST have some digits
    let (input, digits) = digit1(input)?;

    // If we don't have a parse error by then, we have what we need
    Ok((input, format!("E{}{}", opt_sign.unwrap_or('+'), digits)))
}

pub fn parse_code_integer(input: &str) -> IResult<&str, i64> {
    let (input, opt_sign) = opt(alt((char('+'), char('-'))))(input)?;
    let (input, digits) = digit1(input)?;
    let (input, _) = space_or_end(input)?;

    let digits = format!("{}{}", opt_sign.unwrap_or('+'), digits);

    // Parse it
    match digits.parse::<i64>() {
        Ok(int_value) => Ok((input, int_value)),
        Err(_) => Err(nom::Err::Error(nom::error::make_error(input, nom::error::ErrorKind::Verify))),
    }
}

pub fn parse_code_name(input: &str) -> IResult<&str, String> {
    // Grab anything that is not a space, tab, line ending or list marker
    let (input, name_chars) = many1(none_of(" \t\r\n()"))(input)?;
    let (input, _) = space_or_end(input)?;
    let name: String = name_chars.iter().collect();
    Ok((input, name))
}

#[cfg(test)]
mod tests {
    use crate::parse::{
        parse, parse_code_bool, parse_code_float, parse_code_integer, parse_code_list, parse_code_name,
    };
    use crate::*;
    use rust_decimal::{prelude::ToPrimitive, Decimal};

    fn make_literal_bool(virtual_table: &VirtualTable, value: bool) -> Code {
        let id = virtual_table.id_for_name(BoolLiteralValue::name()).unwrap();
        Code::InstructionWithData(id, Some(InstructionData::from_bool(value)))
    }

    fn make_literal_float(virtual_table: &VirtualTable, value: Float) -> Code {
        let id = virtual_table.id_for_name(FloatLiteralValue::name()).unwrap();
        Code::InstructionWithData(id, Some(InstructionData::from_f64(value.to_f64().unwrap())))
    }

    fn make_literal_integer(virtual_table: &VirtualTable, value: i64) -> Code {
        let id = virtual_table.id_for_name(IntegerLiteralValue::name()).unwrap();
        Code::InstructionWithData(id, Some(InstructionData::from_i64(value)))
    }

    fn make_literal_name<S: Into<String>>(virtual_table: &VirtualTable, value: S) -> Code {
        let id = virtual_table.id_for_name(NameLiteralValue::name()).unwrap();
        Code::InstructionWithData(id, Some(InstructionData::from_string(value)))
    }

    fn make_instruction(virtual_table: &VirtualTable, instruction: &'static str) -> Code {
        let id = virtual_table.id_for_name(instruction).unwrap();
        Code::InstructionWithData(id, None)
    }

    #[test]
    fn parse_bool() {
        let expected = true;
        assert_eq!(parse_code_bool("TRUE").unwrap().1, expected);
    }

    #[test]
    fn parse_float() {
        let expected = Decimal::new(1234, 3);
        assert_eq!(parse_code_float("1.234").unwrap().1, expected);

        let expected = Decimal::new(12300, 0);
        assert_eq!(parse_code_float("123.0E2").unwrap().1, expected);

        let expected = Decimal::new(123, 2);
        assert_eq!(parse_code_float("123.0E-2").unwrap().1, expected);

        assert!(parse_code_float("1234").is_err());
    }

    #[test]
    fn parse_integer() {
        let expected = 1234;
        assert_eq!(parse_code_integer("1234").unwrap().1, expected);

        let expected = -1234;
        assert_eq!(parse_code_integer("-1234").unwrap().1, expected);

        assert!(parse_code_integer("a123").is_err());
    }

    #[test]
    fn parse_name() {
        let expected = "1234KCMA|AA/AA.AAA=";
        assert_eq!(parse_code_name("1234KCMA|AA/AA.AAA=").unwrap().1, expected);
    }

    #[test]
    fn parse_instruction() {
        let virtual_table = new_virtual_table_with_all_instructions();
        let expected = make_instruction(&virtual_table, "BOOL.AND");
        assert_eq!(Code::parse(&virtual_table, "BOOL.AND"), expected);
    }

    #[test]
    fn parse_list() {
        let virtual_table = new_virtual_table_with_all_instructions();
        let expected = Code::List(vec![]);
        assert_eq!(parse_code_list(&virtual_table, "( )").unwrap().1, expected);
        let expected =
            Code::List(vec![make_literal_bool(&virtual_table, true), make_literal_integer(&virtual_table, 123)]);
        assert_eq!(parse_code_list(&virtual_table, "( TRUE 123 )").unwrap().1, expected);

        let expected = Code::List(vec![make_instruction(&virtual_table, "BOOL.AND")]);
        assert_eq!(parse_code_list(&virtual_table, "( BOOL.AND )").unwrap().1, expected);

        // no trailing paren should fail
        assert!(parse_code_list(&virtual_table, "( 123").is_err());
    }

    #[test]
    fn code_parsing() {
        let virtual_table = new_virtual_table_with_all_instructions();
        let expected = Code::List(vec![
            Code::List(vec![
                make_literal_bool(&virtual_table, true),
                make_literal_float(&virtual_table, Decimal::new(12345, 6)),
                make_literal_integer(&virtual_table, -12784),
            ]),
            make_instruction(&virtual_table, "BOOL.AND"),
            make_literal_name(&virtual_table, "TRUENAME"),
        ]);
        assert_eq!(parse(&virtual_table, "( ( TRUE 0.012345 -12784 ) BOOL.AND TRUENAME )"), expected);
    }
}
