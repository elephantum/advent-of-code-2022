use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file_path = "input.txt";

    let file = File::open(file_path)?;

    let lines: Vec<_> = io::BufReader::new(file)
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();
    
    let groups = lines.split(|x| x.is_empty());

    let sums: Vec<i32> = groups
        .map(|g| g.iter().map(|s| s.parse::<i32>().unwrap()))
        .map(|g| g.sum::<i32>())
        .collect();

    let answer_1: i32 = *itertools::max(&sums).unwrap();

    println!("answer 1: {answer_1:?}");

    let mut heap: BinaryHeap<i32> = BinaryHeap::new();

    for i in sums.iter() {
        heap.push(*i);
    }

    let answer_2 = heap
        .into_sorted_vec()
        .iter().rev()
        .take(3)
        .sum::<i32>();

    println!("answer 2: {answer_2}");

    Ok(())
}
