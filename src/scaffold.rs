use anyhow::{anyhow, Result};
use std::{fmt::Display, marker::PhantomData, str::FromStr};

pub trait Parse {
    type Parsed;
    fn parse(raw_input: &str) -> Result<Self::Parsed>;
}

impl Parse for String {
    type Parsed = Self;

    fn parse(raw_input: &str) -> Result<Self::Parsed> {
        Ok(raw_input.trim().to_owned())
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

impl<T: FromStr> Parse for VecFromLines<T>
where
    T::Err: Display,
{
    type Parsed = Vec<T>;
    fn parse(raw_input: &str) -> Result<Self::Parsed> {
        raw_input
            .lines()
            .map(|line| line.parse::<T>())
            .collect::<Result<Vec<T>, _>>()
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

pub trait Problem {
    type Input: Parse;
    type Part1: Display;
    type Part2: Display;

    fn solve_part1(input: &<Self::Input as Parse>::Parsed) -> Self::Part1;
    fn solve_part2(input: &<Self::Input as Parse>::Parsed) -> Self::Part2;
}

pub fn solve_part1<P: Problem>(raw_input: &str) -> P::Part1 {
    let input = P::Input::parse(raw_input).expect("input should parse");
    P::solve_part1(&input)
}

pub fn solve_part2<P: Problem>(raw_input: &str) -> P::Part2 {
    let input = P::Input::parse(raw_input).expect("input should parse");
    P::solve_part2(&input)
}

pub fn solve<P: Problem>(raw_input: &str) {
    let input = P::Input::parse(raw_input).expect("input should parse");
    println!("Part 1: {}", P::solve_part1(&input));
    println!("Part 2: {}", P::solve_part2(&input));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
