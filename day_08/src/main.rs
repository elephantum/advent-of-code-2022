use std::fs;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Should work"))
                .collect()
        })
        .filter(|x: &Vec<u32>| !x.is_empty())
        .collect()
}

#[allow(clippy::ptr_arg)]
fn solve1(data: &Vec<Vec<u32>>) -> Vec<Vec<bool>> {
    let mut res: Vec<Vec<bool>> = data
        .iter()
        .map(|r| r.iter().map(|_| false).collect())
        .collect();

    let w = res.len();
    let h = res[0].len();

    for i in 0..w {
        let mut h1_ = data[i][0];
        res[i][0] = true;

        let mut h2_ = data[i][h - 1];
        res[i][h - 1] = true;

        for j in 1..h {
            if data[i][j] > h1_ {
                h1_ = data[i][j];
                res[i][j] = true;
            }

            if data[i][h - 1 - j] > h2_ {
                h2_ = data[i][h - 1 - j];
                res[i][h - 1 - j] = true;
            }
        }
    }

    for j in 0..h {
        let mut h1_ = data[0][j];
        res[0][j] = true;

        let mut h2_ = data[w - 1][j];
        res[w - 1][j] = true;

        for i in 1..w {
            if data[i][j] > h1_ {
                h1_ = data[i][j];
                res[i][j] = true;
            }

            if data[w - 1 - i][j] > h2_ {
                h2_ = data[w - 1 - i][j];
                res[w - 1 - i][j] = true;
            }
        }
    }

    res
}

#[allow(clippy::ptr_arg)]
fn look(data: &Vec<Vec<u32>>, w: usize, h: usize, i: usize, j: usize, di: isize, dj: isize) -> u32 {
    let max_height = data[i][j];
    let mut res = 0;

    let mut ci = i;
    let mut cj = j;

    #[allow(clippy::while_let_loop)]
    loop {
        if let Some(ci_t) = ci.checked_add_signed(di) {
            if ci_t >= w {
                break;
            }
            ci = ci_t;
        } else {
            break;
        }
        if let Some(cj_t) = cj.checked_add_signed(dj) {
            if cj_t >= h {
                break;
            }
            cj = cj_t;
        } else {
            break;
        }

        res += 1;

        if data[ci][cj] >= max_height {
            break;
        }
    }

    res
}

fn solve2(data: &Vec<Vec<u32>>) -> u32 {
    let w = data.len();
    let h = data[0].len();

    let mut max_score = 0;

    for i in 0..w {
        for j in 0..h {
            let s_10 = look(data, w, h, i, j, -1, 0);
            let s10 = look(data, w, h, i, j, 1, 0);
            let s0_1 = look(data, w, h, i, j, 0, -1);
            let s01 = look(data, w, h, i, j, 0, 1);

            // let cur_height = data[i][j];
            // println!("Score({i}, {j}) {cur_height}) = {s_10}, {s10}, {s0_1}, {s01}");
            
            let score = s_10 * s10 * s0_1 * s01;

            if max_score < score {
                max_score = score;
            }
        }
    }

    max_score
}

fn main() -> std::io::Result<()> {
    // let file_path = "input_small.txt";
    let file_path = "input.txt";
    let file_data = fs::read_to_string(file_path)?;

    let data = parse_input(&file_data);

    // println!("{data:?}");

    let vis = solve1(&data);
    // println!("{vis:?}");

    let res: u32 = vis
        .iter()
        .map(|r| r.iter().map(|x| *x as u32).sum::<u32>())
        .sum();
    println!("Part 1: {res:?}");

    let res2 = solve2(&data);
    println!("Part 2: {res2:?}");

    Ok(())
}
