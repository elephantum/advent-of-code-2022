use std::{collections::HashSet, fs};

use nom::{
    character::complete::{alphanumeric1, newline},
    combinator::{map, opt},
    multi::{many0, separated_list0},
    sequence::{terminated, tuple},
    IResult,
};

type Rucksack = (Vec<u8>, Vec<u8>);

fn parse_file1(input: &str) -> IResult<&str, Vec<Rucksack>> {
    separated_list0(
        newline,
        map(alphanumeric1, |s: &str| {
            (
                s[0..s.len() / 2].as_bytes().to_vec(),
                s[s.len() / 2..s.len()].as_bytes().to_vec(),
            )
        }),
    )(input)
}

type Group = (Vec<u8>, Vec<u8>, Vec<u8>);

fn parse_file2(input: &str) -> IResult<&str, Vec<Group>> {
    let line_p = |i| {
        map(terminated(alphanumeric1, opt(newline)), |s: &str| {
            s.as_bytes().to_vec()
        })(i)
    };
    many0(tuple((line_p, line_p, line_p)))(input)
}

fn find_common1(r: &Rucksack) -> u8 {
    let h = (
        HashSet::<&u8>::from_iter(&r.0),
        HashSet::<&u8>::from_iter(&r.1),
    );

    **h.0.intersection(&h.1).next().expect("Intersection")
}

fn find_common2(g: &Group) -> u8 {
    let h = (
        HashSet::<&u8>::from_iter(&g.0),
        HashSet::<&u8>::from_iter(&g.1),
        HashSet::<&u8>::from_iter(&g.2),
    );

    let j: HashSet<&u8> = h.0
        .intersection(&h.1)
        .cloned()
        .collect();
    
    **j.intersection(&h.2).next().expect("Intersection")
}

fn calc_priority(i: u8) -> u32 {
    if (b'a'..=b'z').contains(&i) {
        (i - b'a' + 1).into()
    } else if (b'A'..=b'Z').contains(&i) {
        (i - b'A' + 27).into()
    } else {
        0
    }
}

fn main() -> std::io::Result<()> {
    // let file_path = "input_small.txt";
    let file_path = "input.txt";
    let file_data = fs::read_to_string(file_path)?;

    let data1 = parse_file1(&file_data).expect("Parsing failed").1;

    let res1: u32 = data1.iter().map(find_common1).map(calc_priority).sum();
    println!("Part 1: {res1:?}");

    let data2 = parse_file2(&file_data).expect("Parse part 2").1;
    let res2: u32 = data2.iter().map(find_common2).map(calc_priority).sum();
    println!("Part 2: {res2:?}");

    Ok(())
}
