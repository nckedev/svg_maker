use core::panic;
use std::fmt::Debug;

use crate::shapes::path::Command;

fn is_separator(byte: &u8) -> bool {
    matches!(byte, b',')
}

fn is_whitespace(byte: &u8) -> bool {
    matches!(byte, 0x20 | 0x09 | 0x0A | 0x0D)
}

fn is_number_value(byte: &u8) -> bool {
    #[allow(clippy::match_like_matches_macro)]
    match byte {
        b'0'..=b'9' => true,
        b'.' | b'+' | b'-' => true,
        b'e' | b'E' => true,
        _ => false,
    }
}

fn is_command(byte: &u8) -> bool {
    matches!(
        byte,
        b'M' | b'm'
            | b'Z'
            | b'z'
            | b'L'
            | b'l'
            | b'H'
            | b'h'
            | b'V'
            | b'v'
            | b'C'
            | b'c'
            | b'S'
            | b's'
            | b'Q'
            | b'q'
            | b'T'
            | b't'
            | b'A'
            | b'a',
    )
}

pub fn parse(value: &str) -> Result<Vec<Command>, ParseError> {
    let mut tokens = vec![];
    let mut iter = value.bytes().peekable();

    while let Some(byte) = &iter.next() {
        let token = match byte {
            x if is_number_value(x) => {
                let mut rest = vec![*x];
                // Peek to see if the number is longer than one byte
                while let Some(next_byte) = iter.peek() {
                    if is_number_value(next_byte) {
                        rest.push(iter.next().unwrap());
                    } else {
                        break;
                    }
                }
                PathToken::Number(rest)
            }
            x if is_command(x) => PathToken::Command(*x),
            x if is_whitespace(x) => {
                // peek to see if there are more whitespaces
                // while let Some(next_byte) = iter.peek() {
                //     if !is_whitespace(next_byte) {
                //         break;
                //     }
                // }
                continue;
            }
            x if is_separator(x) => continue,
            x => return Err(ParseError::UnexpectedCharacter(*x as char)),
        };

        tokens.push(token);
    }

    let mut commands = vec![];
    let mut iter = tokens.iter().peekable();
    while let Some(token) = iter.next() {
        let command = match token {
            PathToken::Command(c) => match c {
                b'M' => {
                    let command_args = parse_args(&mut iter)?;
                    Command::MoveTo(command_args.into())
                }
                b'm' => Command::MoveToRelative(parse_args(&mut iter)?.into()),
                _ => Command::Invalid,
            },
            _ => panic!("error for token: {token:?}"),
        };
        commands.push(command);
    }
    Ok(commands)
}

#[allow(clippy::needless_range_loop)]
fn parse_args<'a, const N: usize>(
    iter: &mut impl Iterator<Item = &'a PathToken>,
) -> Result<[f64; N], ParseError> {
    let mut arr = [0.; N];
    for i in 0..N {
        arr[i] = match iter.next() {
            Some(PathToken::Number(nr)) => bytes_to_f64(nr)?,
            Some(t) => return Err(ParseError::UnexpectedToken(t.clone())),
            None => return Err(ParseError::NoTokenFound),
        };
    }

    Ok(arr)
}

fn bytes_to_f64(bytes: &[u8]) -> Result<f64, ParseError> {
    let nr = str::from_utf8(bytes)
        .map_err(|_| ParseError::ParseToF64)?
        .parse::<f64>()
        .map_err(|_| ParseError::ParseToF64)?;
    Ok(nr)
}

#[derive(Debug)]
pub enum ParseError {
    ParseToF64,
    UnexpectedCharacter(char),
    UnexpectedToken(PathToken),
    NoTokenFound,
}

#[derive(Clone)]
pub(crate) enum PathToken {
    Number(Vec<u8>),
    Command(u8),
    // Separator,
    // Whitespace,
}

impl Debug for PathToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Number(arg0) => &format!("Number: {}", str::from_utf8(arg0).unwrap()),
            Self::Command(arg0) => &format!("Command: {}", *arg0 as char),
            // Self::Separator => "Separator",
            // Self::Whitespace => "Whitespace",
        };

        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use crate::{buffer::Buffer, path_parser::is_command, visit::Visit};

    use super::*;

    #[test]
    fn parser() {
        let str = "M10,20 M 20 20M33 1e3";
        let commands = super::parse(str).unwrap();
        let mut buffer = Buffer::with_capacity(10);
        commands.visit(&mut buffer);
        assert_eq!(buffer.str(), "M10,20 M20,20 M33,1000");
    }

    #[test]
    fn test_is_command() {
        assert!(is_command(&b'm'))
    }
}
