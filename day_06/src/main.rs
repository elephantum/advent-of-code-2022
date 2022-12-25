use std::{fs, collections::HashSet};

fn solve(data: &str, packet_size: usize) -> usize {
    for i in packet_size..data.len() {
        let h = HashSet::<char>::from_iter(data[i-packet_size..i].chars());

        if h.len() == packet_size {
            return i
        }
    }

    0
}

fn main() -> std::io::Result<()> {
    // let file_path = "input_small.txt";
    let file_path = "input.txt";
    let file_data = fs::read_to_string(file_path)?;

    let res1 = solve(&file_data, 4);
    println!("Part 1: {res1:?}");

    let res2 = solve(&file_data, 14);
    println!("Part 2: {res2:?}");

    Ok(())
}
