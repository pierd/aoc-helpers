use anyhow::{anyhow, Result};
use std::{fmt::Display, marker::PhantomData, str::FromStr};

pub trait Parse {
    type Parsed;
    fn parse(raw_input: &str) -> Result<Self::Parsed>;
}

impl<T: FromStr> Parse for T
where
    T::Err: Display,
{
    type Parsed = T;

    fn parse(raw_input: &str) -> Result<Self::Parsed> {
        raw_input
            .trim()
            .parse::<T>()
            .map_err(|e| anyhow::anyhow!("Parsing failed: {}", e))
    }
}

pub struct TrimAndParse<T> {
    _phantom: PhantomData<T>,
}

impl<T: FromStr> Parse for TrimAndParse<T>
where
    T::Err: Display,
{
    type Parsed = T;

    fn parse(raw_input: &str) -> Result<Self::Parsed> {
        raw_input
            .trim()
            .parse::<T>()
            .map_err(|e| anyhow::anyhow!("Parsing failed: {}", e))
    }
}

pub struct VecFromLines<T> {
    _phantom: PhantomData<T>,
}

impl<T: Parse> Parse for VecFromLines<T> {
    type Parsed = Vec<T::Parsed>;
    fn parse(raw_input: &str) -> Result<Self::Parsed> {
        raw_input
            .lines()
            .map(|line| T::parse(line))
            .collect::<Result<Vec<T::Parsed>, _>>()
            .map_err(|e| anyhow!("Parse failed: {}", e))
    }
}

pub struct VecFromMultiLines<T> {
    _phantom: PhantomData<T>,
}

impl<T: Parse> Parse for VecFromMultiLines<T> {
    type Parsed = Vec<T::Parsed>;
    fn parse(raw_input: &str) -> Result<Self::Parsed> {
        raw_input
            .split("\n\n")
            .map(|multiple_lines| T::parse(multiple_lines.trim()))
            .collect::<Result<Vec<T::Parsed>, _>>()
            .map_err(|e| anyhow!("Parse failed: {}", e))
    }
}

pub struct VecFromCommaSeparated<T> {
    _phantom: PhantomData<T>,
}

impl<T: FromStr> Parse for VecFromCommaSeparated<T>
where
    T::Err: Display,
{
    type Parsed = Vec<T>;
    fn parse(raw_input: &str) -> Result<Self::Parsed> {
        raw_input
            .split(&[',', ' '])
            .filter(|s| !s.is_empty())
            .map(|part| part.trim().parse::<T>())
            .collect::<Result<Vec<T>, _>>()
            .map_err(|e| anyhow!("Parse failed: {}", e))
    }
}

pub struct VecFromWhitespaceSeparated<T> {
    _phantom: PhantomData<T>,
}

impl<T: Parse> Parse for VecFromWhitespaceSeparated<T> {
    type Parsed = Vec<T::Parsed>;
    fn parse(raw_input: &str) -> Result<Self::Parsed> {
        raw_input
            .split_ascii_whitespace()
            .filter(|s| !s.is_empty())
            .map(|part| T::parse(part.trim()))
            .collect::<Result<Vec<T::Parsed>, _>>()
            .map_err(|e| anyhow!("Parse failed: {}", e))
    }
}

pub struct RowsOfChars<T> {
    _phantom: PhantomData<T>,
}

impl<T: TryFrom<char>> Parse for RowsOfChars<T>
where
    T::Error: Display,
{
    type Parsed = Vec<Vec<T>>;
    fn parse(raw_input: &str) -> Result<Self::Parsed> {
        raw_input
            .lines()
            .map(|line| line.chars().map(T::try_from).collect())
            .collect::<Result<Vec<Vec<T>>, _>>()
            .map_err(|e| anyhow!("Parse failed: {}", e))
    }
}

pub struct TwoSections<A, B> {
    _phantom: PhantomData<(A, B)>,
}

impl<A: Parse, B: Parse> Parse for TwoSections<A, B> {
    type Parsed = (A::Parsed, B::Parsed);
    fn parse(raw_input: &str) -> Result<Self::Parsed> {
        if let Some((first, second)) = raw_input.split_once("\n\n") {
            Ok((A::parse(first)?, B::parse(second)?))
        } else {
            Err(anyhow!("Failed to find 2 sections"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_for_fromstr_types() {
        assert_eq!(usize::parse("42").unwrap(), 42);
        assert_eq!(isize::parse("42").unwrap(), 42);
        assert_eq!(isize::parse("-42").unwrap(), -42);
        assert_eq!(String::parse("foobar").unwrap(), "foobar".to_owned());
    }

    #[test]
    fn test_vec_from_lines() {
        assert_eq!(
            VecFromLines::<usize>::parse("1\n2\n3").unwrap(),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn test_vec_from_multilines() {
        assert_eq!(
            VecFromMultiLines::<String>::parse("1\n2\n\nfoo\nbar").unwrap(),
            vec!["1\n2".to_owned(), "foo\nbar".to_owned(),]
        );
    }

    #[test]
    fn test_vec_from_comma_separated() {
        assert_eq!(
            VecFromCommaSeparated::<usize>::parse("1,2,3").unwrap(),
            vec![1, 2, 3]
        );
        assert_eq!(
            VecFromCommaSeparated::<usize>::parse("1, 2, 3").unwrap(),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn test_vec_from_whitespace_separated() {
        assert_eq!(
            VecFromWhitespaceSeparated::<usize>::parse("1 2 3").unwrap(),
            vec![1, 2, 3]
        );
        assert_eq!(
            VecFromWhitespaceSeparated::<usize>::parse("1\t2  3").unwrap(),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn test_rows_of_chars() {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        enum ABC {
            A,
            B,
            C,
        }

        impl TryFrom<char> for ABC {
            type Error = String;

            fn try_from(value: char) -> Result<Self, Self::Error> {
                match value {
                    'a' => Ok(Self::A),
                    'b' => Ok(Self::B),
                    'c' => Ok(Self::C),
                    _ => Err("Boom".to_owned()),
                }
            }
        }

        assert_eq!(
            RowsOfChars::<ABC>::parse("abc\na\nb\nc").unwrap(),
            vec![
                vec![ABC::A, ABC::B, ABC::C],
                vec![ABC::A],
                vec![ABC::B],
                vec![ABC::C],
            ]
        );
    }
}
