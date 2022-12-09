use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let data = fs::read_to_string("./resources/input_5_1.txt")?;
    let (mut initial_state, moves): (Vec<&str>, Vec<&str>) = {
        let parts = data.split_once("\n\n").unwrap();
        (parts.0.split("\n").collect(),
        parts.1.split("\n").collect())
    };

    let mut state: Vec<String> = vec![String::new(); 9];

    initial_state.pop();
    for line in initial_state.iter() {
        for (index, ch) in line.chars().enumerate() {
            match ch {
                '[' | ']' | ' ' => {},
                _ => {
                    match index {
                        1 => state[index - 1].push(ch),
                        _ => state[index / 4].push(ch),
                    }
                }
            }
        }
    }
    state = state.iter().map(|string| string.chars().rev().collect()).collect();

    for line in moves.iter() {
        let steps: Vec<u32> = line.split(" ").filter_map(|string| {
            match string.parse::<u32>() {
                Ok(num) => Some(num),
                Err(_e) => None,
            }
        }).collect();
        let mut temp = String::new();
        for _ in 0..steps[0] {
            let ch = state[(steps[1] - 1) as usize].pop().unwrap();
            temp.push(ch);
        }

        for _ in 0..steps[0] {
            let ch = temp.pop().unwrap();
            state[(steps[2] -1) as usize].push(ch);
        }
    }

    for string in state {
        print!("{}", string.chars().last().unwrap());
    }
    println!();

    Ok(())
}