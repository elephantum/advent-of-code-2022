use std::{collections::HashMap, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline, not_line_ending},
    combinator::{map, value},
    multi::separated_list0,
    sequence::{preceded, separated_pair, tuple},
    IResult, ToUsize,
};

#[derive(Clone, Debug)]
enum Line {
    CdRoot,
    CdChild(String),
    CdUp,
    List(HashMap<String, usize>),
}

fn parse_list_items(input: &str) -> IResult<&str, HashMap<String, usize>> {
    map(
        separated_list0(
            newline,
            alt((
                value(None, preceded(tag("dir "), not_line_ending)),
                map(
                    separated_pair(nom::character::complete::u32, tag(" "), not_line_ending),
                    |(s, n): (u32, &str)| Some((n.to_string(), s.to_usize())),
                ),
            )),
        ),
        |ii| HashMap::from_iter(ii.iter().filter_map(|i| i.clone())),
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    alt((
        value(Line::CdRoot, tag("$ cd /")),
        value(Line::CdUp, tag("$ cd ..")),
        map(
            preceded(tuple((tag("$ ls"), newline)), parse_list_items),
            Line::List,
        ),
        map(preceded(tag("$ cd "), alphanumeric1), |s: &str| {
            Line::CdChild(s.to_string())
        }),
    ))(input)
}

fn parse_data(input: &str) -> IResult<&str, Vec<Line>> {
    separated_list0(newline, parse_line)(input)
}

#[derive(Debug)]
struct Dir {
    subdirs: HashMap<String, Dir>,
    files: HashMap<String, usize>,
    total: usize,
}

fn set_files(dir: &mut Dir, cur_dir: &[String], files: &HashMap<String, usize>) {
    let mut cd = dir;
    for dirname in cur_dir {
        if !cd.subdirs.contains_key(dirname) {
            cd.subdirs.insert(
                dirname.clone(),
                Dir {
                    subdirs: HashMap::new(),
                    files: HashMap::new(),
                    total: 0,
                },
            );
        }

        cd = cd.subdirs.get_mut(dirname).expect("Should be");
    }

    cd.files = files.clone();
}

fn count_sizes(dir: &mut Dir) -> usize {
    let mut size = 0;

    for (_, subdir) in dir.subdirs.iter_mut() {
        size += count_sizes(subdir);
    }

    for (_, filesize) in dir.files.iter() {
        size += filesize;
    }

    dir.total = size;

    size
}

fn interpret_lines(lines: &[Line]) -> Dir {
    let mut dir = Dir {
        subdirs: HashMap::new(),
        files: HashMap::new(),
        total: 0,
    };
    let mut cur_dir: Vec<String> = vec![];

    for line in lines {
        match line {
            Line::CdRoot => {
                cur_dir.clear();
            }
            Line::CdChild(dirname) => cur_dir.push(dirname.clone()),
            Line::CdUp => {
                cur_dir.pop();
            }
            Line::List(files) => set_files(&mut dir, &cur_dir, files),
        };
    }

    dir
}

fn solve1(dir: &Dir) -> usize {
    let mut res = 0;
    if dir.total <= 100000 {
        res += dir.total;
    }

    for (_, subdir) in dir.subdirs.iter() {
        res += solve1(subdir);
    }

    res
}

fn solve2(dir: &Dir) -> usize {
    let total = 70000000;
    let needed = 30000000;

    let current = total - dir.total;

    let res = traversal::dft_post(dir, |d: &Dir| d.subdirs.values())
        .map(|(_, d)| d.total)
        .filter(|x| current + x > needed)
        .min()
        .expect("Should be");

    res
}

fn main() -> std::io::Result<()> {
    // let file_path = "input_small.txt";
    let file_path = "input.txt";
    let file_data = fs::read_to_string(file_path)?;

    let data = parse_data(&file_data).expect("Parse").1;
    // println!("{data:?}");

    let mut dir = interpret_lines(&data);
    count_sizes(&mut dir);
    // println!("{dir:?}");

    let res1 = solve1(&dir);
    println!("Part 1: {res1:?}");

    let res2 = solve2(&dir);
    println!("Part 2: {res2:?}");

    Ok(())
}
