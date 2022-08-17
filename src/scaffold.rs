use crate::parse::Parse;

pub trait Problem {
    type Input: Parse;
    type Part1: std::fmt::Display;
    type Part2: std::fmt::Display;

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
