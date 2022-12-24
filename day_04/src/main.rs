use std::fs;

use nom::{
    character::complete::char, character::complete::newline, multi::separated_list0,
    sequence::separated_pair, IResult,
};

type Assignment = (i32, i32);
type Pair = (Assignment, Assignment);

fn parse_assignment(input: &str) -> IResult<&str, Assignment> {
    separated_pair(
        nom::character::complete::i32,
        char('-'),
        nom::character::complete::i32,
    )(input)
}

fn parse_file(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list0(
        newline,
        separated_pair(parse_assignment, char(','), parse_assignment),
    )(input)
}

#[allow(clippy::bool_to_int_with_if)]
fn is_contains(a: &Pair) -> u32 {
    if a.0.0 <= a.1.0 && a.0.1 >= a.1.1 || a.0.0 >= a.1.0 && a.0.1 <= a.1.1 {
        1
    } else {
        0
    }
}

#[allow(clippy::if_same_then_else,clippy::bool_to_int_with_if)]
fn is_intersects(a: &Pair) -> u32 {
    if a.0.0 <= a.1.0 && a.0.1 >= a.1.0 || a.0.0 <= a.1.1 && a.0.1 >= a.1.1 {
        1
    } else if a.1.0 <= a.0.0 && a.1.1 >= a.0.0 || a.1.0 <= a.0.1 && a.1.1 >= a.0.1 {
        1
    } else {
        0
    }
}

fn main() -> std::io::Result<()> {
    // let file_path = "input_small.txt";
    let file_path = "input.txt";
    let file_data = fs::read_to_string(file_path)?;

    let data = parse_file(&file_data).expect("Parse").1;

    let res1: u32 = data.iter().map(is_contains).sum();
    println!("Part 1: {res1:?}");

    let res2: u32 = data.iter().map(is_intersects).sum();
    println!("Part 2: {res2:?}");

    Ok(())
}
