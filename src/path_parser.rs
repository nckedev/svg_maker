use core::panic;
use std::fmt::Debug;

use crate::shapes::path::Command;

fn is_separator(byte: u8) -> bool {
    matches!(byte, b',')
}

fn is_whitespace(byte: u8) -> bool {
    matches!(byte, 0x20 | 0x09 | 0x0A | 0x0D)
}

fn is_number_value(byte: u8) -> bool {
    #[allow(clippy::match_like_matches_macro)]
    match byte {
        b'0'..=b'9' => true,
        b'.' | b'+' | b'-' => true,
        b'e' | b'E' => true,
        _ => false,
    }
}

#[rustfmt::skip]
fn is_command(byte: u8) -> bool {
    matches!(
        byte,
        b'M' | b'm'     //moveto
        | b'L' | b'l'   //line
        | b'H' | b'h'   //horizontal line
        | b'V' | b'v'   //vertical line
        | b'C' | b'c'   //cubic bezier
        | b'S' | b's'   //cubic cont.
        | b'Q' | b'q'   //quadratic bezier
        | b'T' | b't'   //quadratic cont.
        | b'A' | b'a'   //arc
        | b'Z' | b'z',  //close
    )
}

pub fn parse(value: &str) -> Result<Vec<Command>, ParseError> {
    let mut tokens = vec![];
    let mut iter = value.bytes().peekable();

    while let Some(byte) = &iter.next() {
        let token = match byte {
            x if is_number_value(*x) => {
                let mut rest = vec![*x];
                // Peek to see if the number is longer than one byte
                while let Some(next_byte) = iter.peek() {
                    if is_number_value(*next_byte) {
                        rest.push(iter.next().unwrap());
                    } else {
                        break;
                    }
                }
                PathToken::Number(rest)
            }
            x if is_command(*x) => PathToken::Command(*x),
            x if is_whitespace(*x) => {
                // peek to see if there are more whitespaces
                // while let Some(next_byte) = iter.peek() {
                //     if !is_whitespace(next_byte) {
                //         break;
                //     }
                // }
                continue;
            }
            x if is_separator(*x) => continue,
            x => {
                return Err(ParseError::UnexpectedCharacter(format!(
                    "unexpected charcter: {}",
                    *x as char
                )));
            }
        };

        tokens.push(token);
    }

    let mut commands = vec![];
    let mut iter = tokens.iter().peekable();
    while let Some(token) = iter.next() {
        let command = match token {
            PathToken::Command(c) => match c {
                b'M' => Command::MoveTo(parse_args(&mut iter)?.into()),
                b'm' => Command::MoveToRelative(parse_args(&mut iter)?.into()),
                b'L' => Command::Line(parse_args(&mut iter)?.into()),
                b'l' => Command::LineRelative(parse_args(&mut iter)?.into()),
                b'H' => Command::HorizontalLine(parse_args::<1>(&mut iter)?[0].into()),
                b'h' => Command::HorizontalLineRelative(parse_args::<1>(&mut iter)?[0].into()),
                b'V' => Command::VerticalLine(parse_args::<1>(&mut iter)?[0].into()),
                b'v' => Command::VerticalLineRelative(parse_args::<1>(&mut iter)?[0].into()),
                b'C' => Command::CubicBezier(parse_args::<6>(&mut iter)?.into()),
                b'c' => Command::CubicBezierRelative(parse_args::<6>(&mut iter)?.into()),
                b'S' => todo!(),
                b's' => todo!(),
                b'Q' => todo!(),
                b'q' => todo!(),
                b'T' => todo!(),
                b't' => todo!(),
                b'A' => todo!(),
                b'a' => todo!(),
                b'Z' => todo!(),
                b'z' => todo!(),
                t => {
                    return Err(ParseError::UnexpectedCharacter(format!(
                        "expected path command, found {}",
                        *t as char
                    )));
                }
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
    // TODO: A command can have mutiple pairs of araguments
    // a L10,10 L20,20 is the same as L10,10,20,20
    // So... we need to check
    // 1. if the next token after the parsed set is a number.
    // 2. does the next set have N numbers
    // how should this be returned? boolena flag "has_more"?
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

#[derive(Debug, PartialEq)]
pub enum ParseError {
    ParseToF64,
    UnexpectedCharacter(String),
    UnexpectedToken(PathToken),
    NoTokenFound,
}

#[derive(Clone, PartialEq)]
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

// Helper functions for testing
#[cfg(test)]
fn assert_parse_success(input: &str, expected: Vec<Command>) {
    let result = parse(input).unwrap();
    assert_eq!(result, expected, "Failed to parse: '{}'", input);
}

#[cfg(test)]
fn assert_parse_error(input: &str, expected_error: ParseError) {
    let result = parse(input);
    assert!(
        result.is_err(),
        "Expected error but got success for: '{}'",
        input
    );
    let actual_error = result.unwrap_err();
    match (actual_error, expected_error) {
        (ParseError::UnexpectedCharacter(_), ParseError::UnexpectedCharacter(_)) => {}
        (ParseError::UnexpectedToken(_), ParseError::UnexpectedToken(_)) => {}
        (actual, expected) => assert_eq!(actual, expected),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        buffer::Buffer,
        path_parser::is_command,
        units::{Coord, CubicArgs, XCoord, YCoord},
        visit::Visit,
    };

    use super::*;

    #[test]
    fn parser() {
        let str = "M10,20 M 20 20M33 1e3";
        let commands = parse(str).unwrap();
        let mut buffer = Buffer::with_capacity(10);
        commands.visit(&mut buffer);
        assert_eq!(buffer.str(), "M10,20 M20,20 M33,1000");
    }

    #[test]
    fn test_is_command() {
        assert!(is_command(b'm'))
    }

    // ===== Helper Functions Tests =====
    mod helper_functions {
        use super::*;

        #[test]
        fn test_is_separator() {
            assert!(is_separator(b','));
            assert!(!is_separator(b';'));
            assert!(!is_separator(b':'));
            assert!(!is_separator(b' '));
        }

        #[test]
        fn test_is_whitespace() {
            assert!(is_whitespace(b' ')); // space
            assert!(is_whitespace(b'\t')); // tab
            assert!(is_whitespace(b'\n')); // newline
            assert!(is_whitespace(b'\r')); // carriage return
            assert!(!is_whitespace(b'x'));
            assert!(!is_whitespace(b','));
        }

        #[test]
        fn test_is_number_value() {
            // Digits
            assert!(is_number_value(b'0'));
            assert!(is_number_value(b'5'));
            assert!(is_number_value(b'9'));

            // Decimal point
            assert!(is_number_value(b'.'));

            // Signs
            assert!(is_number_value(b'+'));
            assert!(is_number_value(b'-'));

            // Scientific notation
            assert!(is_number_value(b'e'));
            assert!(is_number_value(b'E'));

            // Invalid characters
            assert!(!is_number_value(b'x'));
            assert!(!is_number_value(b','));
            assert!(!is_number_value(b' '));
        }

        #[test]
        fn test_bytes_to_f64() {
            // Valid numbers
            assert_eq!(bytes_to_f64(b"10").unwrap(), 10.0);
            assert_eq!(bytes_to_f64(b"-5.5").unwrap(), -5.5);
            assert_eq!(bytes_to_f64(b"1e3").unwrap(), 1000.0);
            assert_eq!(bytes_to_f64(b"1E-3").unwrap(), 0.001);

            // Invalid UTF-8
            assert!(matches!(
                bytes_to_f64(&[255, 0]),
                Err(ParseError::ParseToF64)
            ));

            // Invalid number format
            assert!(matches!(bytes_to_f64(b"abc"), Err(ParseError::ParseToF64)));
            assert!(matches!(
                bytes_to_f64(b"1.2.3"),
                Err(ParseError::ParseToF64)
            ));
        }

        #[test]
        fn test_is_command_comprehensive() {
            // Uppercase commands
            assert!(is_command(b'M')); // MoveTo
            assert!(is_command(b'L')); // LineTo
            assert!(is_command(b'H')); // HorizontalLine
            assert!(is_command(b'V')); // VerticalLine
            assert!(is_command(b'C')); // CubicBezier
            assert!(is_command(b'S')); // CubicBezierExtended
            assert!(is_command(b'Q')); // QuadraticBezier
            assert!(is_command(b'T')); // QuadraticBezierExtended
            assert!(is_command(b'A')); // Arc
            assert!(is_command(b'Z')); // ClosePath

            // Lowercase commands
            assert!(is_command(b'm'));
            assert!(is_command(b'l'));
            assert!(is_command(b'h'));
            assert!(is_command(b'v'));
            assert!(is_command(b'c'));
            assert!(is_command(b's'));
            assert!(is_command(b'q'));
            assert!(is_command(b't'));
            assert!(is_command(b'a'));
            assert!(is_command(b'z'));

            // Invalid commands
            assert!(!is_command(b'x'));
            assert!(!is_command(b'1'));
            assert!(!is_command(b' '));
        }
    }

    // ===== Number Parsing Tests =====
    mod number_parsing {
        use super::*;

        #[test]
        fn test_basic_integers() {
            assert_parse_success("M10 20", vec![Command::MoveTo(Coord::from((10.0, 20.0)))]);
            assert_parse_success("M0 0", vec![Command::MoveTo(Coord::from((0.0, 0.0)))]);
            assert_parse_success(
                "M-10 -20",
                vec![Command::MoveTo(Coord::from((-10.0, -20.0)))],
            );
        }

        #[test]
        fn test_decimal_numbers() {
            assert_parse_success(
                "M10.5 20.25",
                vec![Command::MoveTo(Coord::from((10.5, 20.25)))],
            );
            assert_parse_success("M0.1 0.01", vec![Command::MoveTo(Coord::from((0.1, 0.01)))]);
            assert_parse_success(
                "M-10.5 -20.25",
                vec![Command::MoveTo(Coord::from((-10.5, -20.25)))],
            );
        }

        #[test]
        fn test_scientific_notation() {
            assert_parse_success(
                "M1e3 1E-3",
                vec![Command::MoveTo(Coord::from((1000.0, 0.001)))],
            );
            assert_parse_success(
                "M1.5e2 2.5E-1",
                vec![Command::MoveTo(Coord::from((150.0, 0.25)))],
            );
            assert_parse_success(
                "M-1e3 -1E-3",
                vec![Command::MoveTo(Coord::from((-1000.0, -0.001)))],
            );
        }

        #[test]
        fn test_mixed_number_formats() {
            assert_parse_success(
                "M-1.5e+2 0.25E-1",
                vec![Command::MoveTo(Coord::from((-150.0, 0.025)))],
            );
        }
    }

    // ===== Command Parsing Tests =====
    mod command_parsing {
        use super::*;

        #[test]
        fn test_move_to_commands() {
            assert_parse_success("M10,20", vec![Command::MoveTo(Coord::from((10.0, 20.0)))]);
            assert_parse_success(
                "m10,20",
                vec![Command::MoveToRelative(Coord::from((10.0, 20.0)))],
            );
            assert_parse_success(
                "M-10,-20",
                vec![Command::MoveTo(Coord::from((-10.0, -20.0)))],
            );
        }

        #[test]
        fn test_line_commands() {
            assert_parse_success("L10,20", vec![Command::Line(Coord::from((10.0, 20.0)))]);
            assert_parse_success(
                "l10,20",
                vec![Command::LineRelative(Coord::from((10.0, 20.0)))],
            );
            assert_parse_success("L-10,-20", vec![Command::Line(Coord::from((-10.0, -20.0)))]);
        }

        #[test]
        fn test_horizontal_vertical_commands() {
            assert_parse_success("H10", vec![Command::HorizontalLine(XCoord(10.0))]);
            assert_parse_success("V20", vec![Command::VerticalLine(YCoord(20.0))]);
            assert_parse_success("h10", vec![Command::HorizontalLineRelative(XCoord(10.0))]);
            assert_parse_success("v20", vec![Command::VerticalLineRelative(YCoord(20.0))]);
            assert_parse_success("H-10", vec![Command::HorizontalLine(XCoord(-10.0))]);
            assert_parse_success("V-20", vec![Command::VerticalLine(YCoord(-20.0))]);
        }

        #[test]
        fn test_cubic_bezier_commands() {
            assert_parse_success(
                "C1,2,3,4,5,6",
                vec![Command::CubicBezier(CubicArgs::from([
                    1.0, 2.0, 3.0, 4.0, 5.0, 6.0,
                ]))],
            );
            assert_parse_success(
                "c1,2,3,4,5,6",
                vec![Command::CubicBezierRelative(CubicArgs::from([
                    1.0, 2.0, 3.0, 4.0, 5.0, 6.0,
                ]))],
            );
            assert_parse_success(
                "C-1,-2,-3,-4,-5,-6",
                vec![Command::CubicBezier(CubicArgs::from([
                    -1.0, -2.0, -3.0, -4.0, -5.0, -6.0,
                ]))],
            );
        }
    }

    // ===== Whitespace & Separator Tests =====
    mod whitespace_handling {
        use super::*;

        #[test]
        fn test_multiple_spaces() {
            assert_parse_success(
                "M  10  20",
                vec![Command::MoveTo(Coord::from((10.0, 20.0)))],
            );
            assert_parse_success(
                "M   10   20",
                vec![Command::MoveTo(Coord::from((10.0, 20.0)))],
            );
        }

        #[test]
        fn test_tabs_and_newlines() {
            assert_parse_success(
                "M\t10\n20",
                vec![Command::MoveTo(Coord::from((10.0, 20.0)))],
            );
            assert_parse_success(
                "M\r10\t20",
                vec![Command::MoveTo(Coord::from((10.0, 20.0)))],
            );
            assert_parse_success(
                "M\r\n10\r\n20",
                vec![Command::MoveTo(Coord::from((10.0, 20.0)))],
            );
        }

        #[test]
        fn test_mixed_whitespace() {
            assert_parse_success(
                "M \t 10 \r\n 20",
                vec![Command::MoveTo(Coord::from((10.0, 20.0)))],
            );
            assert_parse_success(
                "M\t\n\r 10 \t\n\r 20",
                vec![Command::MoveTo(Coord::from((10.0, 20.0)))],
            );
        }

        #[test]
        fn test_comma_separators() {
            assert_parse_success("M10,20", vec![Command::MoveTo(Coord::from((10.0, 20.0)))]);
            assert_parse_success("L10,20", vec![Command::Line(Coord::from((10.0, 20.0)))]);
            assert_parse_success(
                "C1,2,3,4,5,6",
                vec![Command::CubicBezier(CubicArgs::from([
                    1.0, 2.0, 3.0, 4.0, 5.0, 6.0,
                ]))],
            );
        }

        #[test]
        fn test_mixed_separators() {
            assert_parse_success("M10, 20", vec![Command::MoveTo(Coord::from((10.0, 20.0)))]);
            assert_parse_success("M10 ,20", vec![Command::MoveTo(Coord::from((10.0, 20.0)))]);
            assert_parse_success("M10 , 20", vec![Command::MoveTo(Coord::from((10.0, 20.0)))]);
            assert_parse_success("M 10,20", vec![Command::MoveTo(Coord::from((10.0, 20.0)))]);
            assert_parse_success(
                "M 10 , 20",
                vec![Command::MoveTo(Coord::from((10.0, 20.0)))],
            );
        }

        #[test]
        fn test_no_whitespace() {
            // "M1020" without whitespace should fail to parse correctly
            // The parser will interpret "1020" as one number (1020.0) and expect another number
            assert!(matches!(parse("M1020"), Err(ParseError::NoTokenFound)));
        }
    }

    // ===== Error Condition Tests =====
    mod error_conditions {
        use super::*;

        #[test]
        fn test_invalid_characters() {
            assert!(matches!(
                parse("M@10,20"),
                Err(ParseError::UnexpectedCharacter(_))
            ));
            assert!(matches!(
                parse("M10#20"),
                Err(ParseError::UnexpectedCharacter(_))
            ));
            assert!(matches!(
                parse("M10;20"),
                Err(ParseError::UnexpectedCharacter(_))
            ));
            assert!(matches!(
                parse("M10:20"),
                Err(ParseError::UnexpectedCharacter(_))
            ));
        }

        #[test]
        fn test_missing_arguments() {
            assert_parse_error("M10", ParseError::NoTokenFound);
            assert_parse_error("L", ParseError::NoTokenFound);
            assert_parse_error("H", ParseError::NoTokenFound);
            assert_parse_error("V", ParseError::NoTokenFound);
            assert_parse_error("C1,2,3,4,5", ParseError::NoTokenFound);
        }

        #[test]
        fn test_invalid_numbers() {
            assert!(matches!(parse("M1.2.3,20"), Err(ParseError::ParseToF64)));
            assert!(matches!(
                parse("Mabc,20"),
                Err(ParseError::UnexpectedCharacter(_))
            ));
            assert!(matches!(parse("M1e,20"), Err(ParseError::ParseToF64)));
            assert!(matches!(parse("Me10,20"), Err(ParseError::ParseToF64)));
        }

        #[test]
        fn test_empty_string() {
            assert_parse_success("", vec![]);
        }

        #[test]
        fn test_whitespace_only() {
            assert_parse_success("   \t\n\r   ", vec![]);
            assert_parse_success("\t\t\t", vec![]);
            assert_parse_success("\n\n\n", vec![]);
        }
    }

    // ===== Integration Tests =====
    mod integration_tests {
        use super::*;

        #[test]
        fn test_multiple_commands() {
            let input = "M10,20 L30,40 H50 V60";
            let expected = vec![
                Command::MoveTo(Coord::from((10.0, 20.0))),
                Command::Line(Coord::from((30.0, 40.0))),
                Command::HorizontalLine(XCoord(50.0)),
                Command::VerticalLine(YCoord(60.0)),
            ];
            assert_parse_success(input, expected);
        }

        #[test]
        fn test_mixed_relative_absolute() {
            let input = "M10,20 l10,10 L30,40";
            let expected = vec![
                Command::MoveTo(Coord::from((10.0, 20.0))),
                Command::LineRelative(Coord::from((10.0, 10.0))),
                Command::Line(Coord::from((30.0, 40.0))),
            ];
            assert_parse_success(input, expected);
        }

        #[test]
        fn test_complex_path() {
            let input = "M10,20 C1,2,3,4,5,6 L30,40";
            let expected = vec![
                Command::MoveTo(Coord::from((10.0, 20.0))),
                Command::CubicBezier(CubicArgs::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0])),
                Command::Line(Coord::from((30.0, 40.0))),
            ];
            assert_parse_success(input, expected);
        }

        #[test]
        fn test_repeated_commands() {
            let input = "M10,20 M30,40 M50,60";
            let expected = vec![
                Command::MoveTo(Coord::from((10.0, 20.0))),
                Command::MoveTo(Coord::from((30.0, 40.0))),
                Command::MoveTo(Coord::from((50.0, 60.0))),
            ];
            assert_parse_success(input, expected);
        }

        #[test]
        fn test_all_command_types() {
            let input = "M10,20 L30,40 H50 V60 C1,2,3,4,5,6";
            let expected = vec![
                Command::MoveTo(Coord::from((10.0, 20.0))),
                Command::Line(Coord::from((30.0, 40.0))),
                Command::HorizontalLine(XCoord(50.0)),
                Command::VerticalLine(YCoord(60.0)),
                Command::CubicBezier(CubicArgs::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0])),
            ];
            assert_parse_success(input, expected);
        }
    }

    // ===== Edge Cases =====
    mod edge_cases {
        use super::*;

        #[test]
        fn test_very_large_numbers() {
            assert_parse_success(
                "M1e308,1e-308",
                vec![Command::MoveTo(Coord::from((1e308, 1e-308)))],
            );
        }

        #[test]
        fn test_scientific_notation_edge_cases() {
            assert_parse_success(
                "M1e+3,1E-3",
                vec![Command::MoveTo(Coord::from((1000.0, 0.001)))],
            );
            assert_parse_success("M1e0,1E0", vec![Command::MoveTo(Coord::from((1.0, 1.0)))]);
        }

        #[test]
        fn test_leading_trailing_whitespace() {
            assert_parse_success(
                "  M10,20  ",
                vec![Command::MoveTo(Coord::from((10.0, 20.0)))],
            );
            assert_parse_success(
                "\tM10,20\n",
                vec![Command::MoveTo(Coord::from((10.0, 20.0)))],
            );
        }
    }
}
