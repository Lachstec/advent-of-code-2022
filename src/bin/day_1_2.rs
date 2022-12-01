use std::{
    io,
    fs
};

fn main() -> Result<(), io::Error> {
    let mut sums: Vec<u32>;
    let data = fs::read_to_string("resources/input_1_1.txt")?;

    let parts: Vec<&str> = data.split("\n\n").collect();
    sums = parts.iter()
        .map(|numstr| {
            numstr.split("\n")
                .map(|x| x.parse::<u32>().unwrap())
                .sum()
        }).collect();

    sums.sort_unstable();
    let sum: u32 = sums.iter()
        .rev()
        .take(3)
        .sum();

    println!("Result: {}", sum);

    Ok(())
}