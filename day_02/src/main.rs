use std::fs::{self};

use nom::branch::alt;
use nom::character::complete::{char, newline, space1};
use nom::combinator::value;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Clone)]
enum ExpRes {
    Loss,
    Draw,
    Win,
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    alt((
        value(Hand::Rock, char('A')),
        value(Hand::Rock, char('X')),
        value(Hand::Paper, char('B')),
        value(Hand::Paper, char('Y')),
        value(Hand::Scissors, char('C')),
        value(Hand::Scissors, char('Z')),
    ))(input)
}

fn parse_expres(input: &str) -> IResult<&str, ExpRes> {
    alt((
        value(ExpRes::Loss, char('X')),
        value(ExpRes::Draw, char('Y')),
        value(ExpRes::Win, char('Z')),
    ))(input)
}

fn parse_data1(input: &str) -> IResult<&str, Vec<(Hand, Hand)>> {
    separated_list0(newline, separated_pair(parse_hand, space1, parse_hand))(input)
}

fn parse_data2(input: &str) -> IResult<&str, Vec<(Hand, ExpRes)>> {
    separated_list0(newline, separated_pair(parse_hand, space1, parse_expres))(input)
}

fn score_hand(a: &Hand) -> i32 {
    match a {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    }
}

fn score_round((a, b): &(Hand, Hand)) -> i32 {
    let win_score = match (&a, &b) {
        (Hand::Rock, Hand::Rock) => 3,
        (Hand::Rock, Hand::Paper) => 6,
        (Hand::Rock, Hand::Scissors) => 0,
        (Hand::Paper, Hand::Rock) => 0,
        (Hand::Paper, Hand::Paper) => 3,
        (Hand::Paper, Hand::Scissors) => 6,
        (Hand::Scissors, Hand::Rock) => 6,
        (Hand::Scissors, Hand::Paper) => 0,
        (Hand::Scissors, Hand::Scissors) => 3,
    };

    win_score + score_hand(b)
}

fn score_expres((a, b): &(Hand, ExpRes)) -> i32 {
    let exp_hand = match (&a, &b) {
        (Hand::Rock, ExpRes::Loss) => Hand::Scissors,
        (Hand::Rock, ExpRes::Draw) => Hand::Rock,
        (Hand::Rock, ExpRes::Win) => Hand::Paper,
        (Hand::Paper, ExpRes::Loss) => Hand::Rock,
        (Hand::Paper, ExpRes::Draw) => Hand::Paper,
        (Hand::Paper, ExpRes::Win) => Hand::Scissors,
        (Hand::Scissors, ExpRes::Loss) => Hand::Paper,
        (Hand::Scissors, ExpRes::Draw) => Hand::Scissors,
        (Hand::Scissors, ExpRes::Win) => Hand::Rock,
    };

    let round_score = match &b {
        ExpRes::Loss => 0,
        ExpRes::Draw => 3,
        ExpRes::Win => 6,
    };

    score_hand(&exp_hand) + round_score
}

fn main() -> std::io::Result<()> {
    // let file_path = "input_small.txt";
    let file_path = "input.txt";
    let file_data = fs::read_to_string(file_path)?;

    let parsed1 = parse_data1(&file_data).expect("Parsing failed").1;

    let res1: i32 = parsed1.iter().map(score_round).sum();

    println!("Part 1: {res1:?}");

    let parsed2 = parse_data2(&file_data).expect("Parsing failed").1;

    let res2: i32 = parsed2.iter().map(score_expres).sum();

    println!("Part 2: {res2:?}");

    Ok(())
}
