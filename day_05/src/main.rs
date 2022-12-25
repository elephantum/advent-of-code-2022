use std::{fs, iter::repeat};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, newline},
    combinator::{map, value},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult, ToUsize,
};

type WorkzoneLine = Vec<Option<char>>;

type Stack = Vec<char>;
type Workzone = Vec<Stack>;

fn parse_workzone_line(input: &str) -> IResult<&str, WorkzoneLine> {
    separated_list1(
        tag(" "),
        alt((
            value(None, tag("   ")),
            delimited(tag("["), map(anychar, Some), tag("]")),
        )),
    )(input)
}

fn lines_to_stack(lines: Vec<WorkzoneLine>) -> Workzone {
    let size = lines[0].len();
    let mut res = Workzone::from_iter(repeat(vec![]).take(size));

    for line in lines.iter().rev() {
        for i in 0..size {
            if let Some(c) = line[i] {
                res[i].push(c);
            }
        }
    }

    res
}

#[derive(Debug)]
struct Instruction {
    from: usize,
    to: usize,
    amount: usize,
}

fn parse_instruction(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list0(
        newline,
        map(
            tuple((
                preceded(tag("move "), nom::character::complete::u32),
                preceded(tag(" from "), nom::character::complete::u32),
                preceded(tag(" to "), nom::character::complete::u32),
            )),
            |(a, f, t)| Instruction {
                from: (f - 1).to_usize(),
                to: (t - 1).to_usize(),
                amount: a.to_usize(),
            },
        ),
    )(input)
}

fn parse_file(input: &str) -> IResult<&str, (Workzone, Vec<u32>, Vec<Instruction>)> {
    tuple((
        map(
            terminated(separated_list0(newline, parse_workzone_line), newline),
            lines_to_stack,
        ),
        terminated(
            separated_list1(
                tag(" "),
                delimited(tag(" "), nom::character::complete::u32, tag(" ")),
            ),
            tuple((newline, newline)),
        ),
        parse_instruction,
    ))(input)
}

fn do_instruction1(mut w: Workzone, instr: &Instruction) -> Workzone {
    for _ in 0..instr.amount {
        let c = w[instr.from].pop().expect("Move");
        w[instr.to].push(c);
    }
    w
}

fn do_instruction2(mut w: Workzone, instr: &Instruction) -> Workzone {
    let f = &mut w[instr.from];
    let c: Vec<char> = f.drain(f.len()-instr.amount..f.len()).collect();
    w[instr.to].extend(c);

    w
}

fn main() -> std::io::Result<()> {
    // let file_path = "input_small.txt";
    let file_path = "input.txt";
    let file_data = fs::read_to_string(file_path)?;

    let data = parse_file(&file_data).expect("Parse").1;

    let mut wz1 = data.0.clone();

    for instr in &data.2 {
        wz1 = do_instruction1(wz1, instr);
    }

    let res1: String = wz1.iter().map(|v| v[v.len()-1]).collect();
    println!("Part 1: {res1:?}");

    let mut wz2 = data.0.clone();

    for instr in &data.2 {
        wz2 = do_instruction2(wz2, instr);
    }

    let res2: String = wz2.iter().map(|v| v[v.len()-1]).collect();
    println!("Part 2: {res2:?}");

    Ok(())
}
