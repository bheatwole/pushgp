use crate::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, none_of, space0, space1},
    combinator::{eof, opt},
    multi::many1,
    IResult,
};
use rust_decimal::{prelude::FromPrimitive, Decimal};

pub type ParseFn<Vm> = fn(input: &str) -> IResult<&str, Code<Vm>>;

pub struct Parser<Vm: VirtualMachine> {
    parsers: Vec<ParseFn<Vm>>,
}

impl<Vm: VirtualMachine + VirtualMachineMustHaveExec<Vm>> Parser<Vm> {
    pub fn new() -> Parser<Vm> {
        Parser { parsers: vec![] }
    }

    pub fn add_instruction<C: StaticInstruction<Vm>>(&mut self) {
        self.parsers.push(C::parse)
    }

    pub fn parse<'a>(&self, input: &'a str) -> nom::IResult<&'a str, Code<Vm>> {
        match self.parse_list(input) {
            Ok((rest, code)) => return Ok((rest, code)),
            Err(_) => {}
        }
        self.parse_one(input)
    }

    pub fn must_parse(&self, input: &str) -> Code<Vm> {
        let (rest, code) = self.parse(input).unwrap();
        assert_eq!(rest.len(), 0);
        code
    }

    fn parse_list<'a>(&self, input: &'a str) -> nom::IResult<&'a str, Code<Vm>> {
        let mut list = vec![];
        let (mut input, _) = start_list(input)?;
        'outer: loop {
            match self.parse(input) {
                Ok((rest, one)) => {
                    input = rest;
                    list.push(one);
                }
                Err(_) => break 'outer,
            }
        }
        (input, _) = end_list(input)?;

        Ok((input, Box::new(PushList::<Vm>::new(list))))
    }

    fn parse_one<'a>(&self, input: &'a str) -> nom::IResult<&'a str, Code<Vm>> {
        for parse_fn in self.parsers.iter() {
            match parse_fn(input) {
                Ok((rest, instruction)) => return Ok((rest, instruction)),
                Err(_) => {
                    // Continue searching
                }
            }
        }

        // Return an error if none of our parsers could parse the string
        Err(nom::Err::Error(nom::error::make_error(input, nom::error::ErrorKind::Verify)))
    }
}

impl<Vm: VirtualMachine> std::fmt::Debug for Parser<Vm> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parser with {} entries", self.parsers.len())
    }
}

impl<Vm: VirtualMachine> std::cmp::PartialEq for Parser<Vm> {
    fn eq(&self, other: &Parser<Vm>) -> bool {
        if self.parsers.len() == other.parsers.len() {
            for i in 0..self.parsers.len() {
                let lhs = self.parsers[i];
                let rhs = other.parsers[i];
                if lhs as usize != rhs as usize {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
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
    use crate::parse::{parse_code_bool, parse_code_float, parse_code_integer, parse_code_name};
    use crate::*;
    use rust_decimal::Decimal;

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
        let mut parser = Parser::<BaseVm>::new();
        parser.add_instruction::<BoolAnd>();
        let expected: Box<dyn Instruction<BaseVm>> = Box::new(BoolAnd {});
        assert_eq!(&parser.must_parse("BOOL.AND"), &expected);
    }

    #[test]
    fn parse_list() {
        let mut parser = Parser::new();
        parser.add_instruction::<BoolAnd>();
        parser.add_instruction::<BoolLiteralValue>();
        parser.add_instruction::<IntegerLiteralValue>();
        let expected: Box<dyn Instruction<BaseVm>> = Box::new(PushList::<BaseVm>::new(vec![]));
        assert_eq!(&parser.must_parse("( )"), &expected);

        let expected: Box<dyn Instruction<BaseVm>> = Box::new(PushList::<BaseVm>::new(vec![
            Box::new(BoolLiteralValue::new(true)),
            Box::new(IntegerLiteralValue::new(123)),
        ]));
        assert_eq!(&parser.must_parse("( TRUE 123 )"), &expected);

        let expected: Box<dyn Instruction<BaseVm>> = Box::new(PushList::<BaseVm>::new(vec![Box::new(BoolAnd {})]));
        assert_eq!(&parser.must_parse("( BOOL.AND )"), &expected);

        // no trailing paren should fail
        assert!(parser.parse("( 123").is_err());
    }

    #[test]
    fn code_parsing() {
        let mut parser = Parser::new();
        parser.add_instruction::<BoolAnd>();
        parser.add_instruction::<BoolLiteralValue>();
        parser.add_instruction::<FloatLiteralValue>();
        parser.add_instruction::<IntegerLiteralValue>();
        parser.add_instruction::<NameLiteralValue>();

        let code = "( ( TRUE 0.012345 -12784 ) BOOL.AND TRUENAME )";
        let expected: Box<dyn Instruction<BaseVm>> = Box::new(PushList::<BaseVm>::new(vec![
            Box::new(PushList::<BaseVm>::new(vec![
                Box::new(BoolLiteralValue::new(true)),
                Box::new(FloatLiteralValue::new(Decimal::new(12345, 6))),
                Box::new(IntegerLiteralValue::new(-12784)),
            ])),
            Box::new(BoolAnd {}),
            Box::new(NameLiteralValue::new("TRUENAME")),
        ]));
        assert_eq!(&parser.must_parse(code), &expected);
    }
}
