// Parsing with Result, the ? operator, and an error enum.
#[derive(Debug, PartialEq)]
enum ParseError { Empty, BadDigit(char), Overflow }
fn parse_u8(s: &str) -> Result<u8, ParseError> {
    if s.is_empty() { return Err(ParseError::Empty); }
    let mut v: u32 = 0;
    for c in s.chars() {
        let d = c.to_digit(10).ok_or(ParseError::BadDigit(c))?;
        v = v * 10 + d;
        if v > 255 { return Err(ParseError::Overflow); }
    }
    Ok(v as u8)
}
fn sum_pair(a: &str, b: &str) -> Result<u16, ParseError> {
    Ok(parse_u8(a)? as u16 + parse_u8(b)? as u16)
}
fn main() {
    let ok = sum_pair("200", "55") == Ok(255)
        && parse_u8("") == Err(ParseError::Empty)
        && parse_u8("4x") == Err(ParseError::BadDigit('x'))
        && parse_u8("256") == Err(ParseError::Overflow);
    std::process::exit(if ok { 0 } else { 1 });
}
