use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead, BufReader, Error, ErrorKind};

pub fn solve() -> String {
    let fpath = Path::new("assets/day1_input.txt");
    let inputs = read_int_inputs(fpath);
    println!("inputs:{:?}", inputs);
    let int_inputs = inputs.unwrap();

    /* Part 1 - Count increases between individual entries */
    let increases = count_increases(&int_inputs);
    let spart1: String = String::from(" Count increases:".to_owned() + &increases.to_string());
    println!("part1| {}", spart1);
    assert_eq!(increases, 1167);

    /* Part 2 - Count increases in sums over a window */
    let sum_increases = count_rolling_sum(&int_inputs);
    let spart2: String = String::from(" Count sum increases:".to_owned() + &sum_increases.to_string());
    println!("part2| {}", spart2);
    assert_eq!(sum_increases, 1130);

    return spart1 + &spart2
}

fn read_int_inputs(fpath:&Path) -> Result<Vec<u16>, io::Error> {
    let f = File::open(fpath)?;
    let buff = BufReader::new(f);
    let mut vin = Vec::new();
    for line in buff.lines() {
        let lin = line?;
        let n = lin
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        vin.push(n);
    }
    Ok(vin)
}

fn count_increases(input:&Vec<u16>) -> u16 {
    let mut ct = 0;
    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            ct+=1;
        }
    }
    return ct
}

fn count_rolling_sum(input:&Vec<u16>) -> u16 {
    let mut ct = 0;
    let window_size = 3;
    let mut prev_window = 0;
    // Loop through array
    for i in 0..(input.len() - window_size + 1) {
        let mut window_sum = 0;
        for w in i..(i + window_size) {
            window_sum += input[w];
        }
        if window_sum > prev_window {
            ct += 1;
        }
        prev_window = window_sum;
    }
    return ct - 1
}

