use crate::instruction::parse_code_instruction;
use crate::Code;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, one_of, space0},
    character::is_digit,
    combinator::opt,
    multi::{many0, many1},
    IResult,
};
use rust_decimal::{prelude::FromPrimitive, Decimal};

pub fn parse_code(input: &str) -> Code {
    parse_one_code(input).unwrap().1
}

fn parse_one_code(input: &str) -> IResult<&str, Code> {
    alt((
        parse_code_instruction,
        parse_code_bool,
        parse_code_float,
        parse_code_name,
        parse_code_integer,
        parse_code_list,
    ))(input)
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

fn parse_code_list(input: &str) -> IResult<&str, Code> {
    // A list is a start tag, zero or more codes and an end tag
    let (input, _) = start_list(input)?;
    let (input, codes) = many0(parse_one_code)(input)?;
    let (input, _) = end_list(input)?;

    Ok((input, Code::List(codes)))
}

fn parse_code_bool(input: &str) -> IResult<&str, Code> {
    let (input, text_value) = alt((tag("TRUE"), tag("FALSE")))(input)?;
    let (input, _) = space0(input)?;

    Ok((
        input,
        match text_value {
            "TRUE" => Code::LiteralBool(true),
            "FALSE" => Code::LiteralBool(false),
            _ => panic!("can't get here"),
        },
    ))
}

fn parse_code_float(input: &str) -> IResult<&str, Code> {
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
    let float_string = format!(
        "{}{}.{}{}",
        opt_sign.unwrap_or('+'),
        whole,
        fractional,
        opt_exponent.unwrap_or("".to_owned())
    );

    // Parse it
    match float_string.parse::<f64>() {
        Ok(float_value) => Ok((
            input,
            Code::LiteralFloat(Decimal::from_f64(float_value).unwrap()),
        )),
        Err(_) => Err(nom::Err::Error(nom::error::make_error(
            input,
            nom::error::ErrorKind::Verify,
        ))),
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

fn parse_code_integer(input: &str) -> IResult<&str, Code> {
    let (input, opt_sign) = opt(alt((char('+'), char('-'))))(input)?;
    let (input, digits) = digit1(input)?;
    let (input, _) = space0(input)?;

    let digits = format!("{}{}", opt_sign.unwrap_or('+'), digits);

    // Parse it
    match digits.parse::<i64>() {
        Ok(int_value) => Ok((input, Code::LiteralInteger(int_value))),
        Err(_) => Err(nom::Err::Error(nom::error::make_error(
            input,
            nom::error::ErrorKind::Verify,
        ))),
    }
}

fn parse_code_name(input: &str) -> IResult<&str, Code> {
    // Grab as many base64 characters as we can
    let (input, mut base64_string) = many1(base64_char)(input)?;

    // It MAY end with and '=' sign
    let (input, opt_equal) = opt(char('='))(input)?;
    let (input, _) = space0(input)?;

    // If every character is a digit and there is no 'equal' sign, than this is actually a number, not a name
    if opt_equal.is_none() && base64_string.iter().all(|&x| is_digit(x as u8)) {
        return Err(nom::Err::Error(nom::error::make_error(
            input,
            nom::error::ErrorKind::Verify,
        )));
    }

    // Otherwise, re-assemble and decode. Pad the end with '=' to make even multiple of four. We can only have '==' at
    // the end or we get an invalid decode error. So if we have three missing bytes, the first missing byte needs to be
    // 'A' which translates to a byte of 0x00.
    while base64_string.len() % 4 > 0 {
        if 1 == base64_string.len() % 4 {
            base64_string.push('A');
        } else {
            base64_string.push('=');
        }
    }
    let base_64_input: String = base64_string.into_iter().collect();
    match base64::decode(base_64_input) {
        Ok(u8_vec) => {
            let mut u8_array = [0u8; 8];
            for (i, &v) in u8_vec.iter().enumerate() {
                if i >= 8 {
                    break;
                }
                u8_array[i] = v;
            }
            let value = u64::from_le_bytes(u8_array);
            Ok((input, Code::LiteralName(value)))
        }
        Err(_) => Err(nom::Err::Failure(nom::error::Error::new(
            "error parsing name",
            nom::error::ErrorKind::Verify,
        ))),
    }
}

fn base64_char(input: &str) -> IResult<&str, char> {
    one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/")(input)
}

#[cfg(test)]
mod tests {
    use crate::instruction::parse_code_instruction;
    use crate::parse::{
        parse_code, parse_code_bool, parse_code_float, parse_code_integer, parse_code_list,
        parse_code_name,
    };
    use crate::{Code, Instruction};
    use rust_decimal::Decimal;

    #[test]
    fn parse_bool() {
        let expected = Code::LiteralBool(true);
        assert_eq!(parse_code_bool("TRUE").unwrap().1, expected);
    }

    #[test]
    fn parse_float() {
        let expected = Code::LiteralFloat(Decimal::new(1234, 3));
        assert_eq!(parse_code_float("1.234").unwrap().1, expected);

        let expected = Code::LiteralFloat(Decimal::new(12300, 0));
        assert_eq!(parse_code_float("123.0E2").unwrap().1, expected);

        let expected = Code::LiteralFloat(Decimal::new(123, 2));
        assert_eq!(parse_code_float("123.0E-2").unwrap().1, expected);

        assert!(parse_code_float("1234").is_err());
    }

    #[test]
    fn parse_integer() {
        let expected = Code::LiteralInteger(1234);
        assert_eq!(parse_code_integer("1234").unwrap().1, expected);

        let expected = Code::LiteralInteger(-1234);
        assert_eq!(parse_code_integer("-1234").unwrap().1, expected);

        assert!(parse_code_integer("a123").is_err());
    }

    #[test]
    fn parse_name() {
        let expected = Code::LiteralName(9000);
        assert_eq!(parse_code_name("KCMAAAAAAAA=").unwrap().1, expected);
        assert_eq!(parse_code_name("KCMAAAAAAAA").unwrap().1, expected);
        assert_eq!(parse_code_name("KCM").unwrap().1, expected);

        // When we happen to base64 a value that is all numbers, it should still parse as a name if it end with '='
        let expected = Code::LiteralName(15993332992822435283);
        assert_eq!(parse_code_name("01234567890=").unwrap().1, expected);
        assert!(parse_code_name("01234567890").is_err());

        let expected = Code::LiteralName(269275136);
        assert_eq!(parse_code("ANAME"), expected);
    }

    #[test]
    fn parse_instruction() {
        let expected = Code::Instruction(Instruction::BoolAnd);
        assert_eq!(parse_code_instruction("BOOLAND").unwrap().1, expected);
    }

    #[test]
    fn parse_list() {
        let expected = Code::List(vec![]);
        assert_eq!(parse_code_list("( )").unwrap().1, expected);
        let expected = Code::List(vec![Code::LiteralBool(true), Code::LiteralInteger(123)]);
        assert_eq!(parse_code_list("( TRUE 123 )").unwrap().1, expected);

        let expected = Code::List(vec![Code::Instruction(Instruction::BoolAnd)]);
        assert_eq!(parse_code_list("( BOOLAND )").unwrap().1, expected);

        // no trailing paren should fail
        assert!(parse_code_list("( 123").is_err());
    }

    #[test]
    fn code_parsing() {
        let expected = Code::List(vec![
            Code::List(vec![
                Code::LiteralBool(true),
                Code::LiteralFloat(Decimal::new(12345, 6)),
                Code::LiteralInteger(-12784),
                Code::LiteralName(9000),
            ]),
            Code::Instruction(Instruction::BoolAnd),
        ]);
        assert_eq!(
            parse_code("( ( TRUE 0.012345 -12784 KCMAAAAAAAA= ) BOOLAND )"),
            expected
        );
    }
}
