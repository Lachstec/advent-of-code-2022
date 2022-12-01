use std::fs;
use std::io;


fn main() -> Result<(), io::Error>{
    let sums: Vec<u32>;
    let data = fs::read_to_string("input_1_1.txt")?;

    let parts: Vec<&str> = data.split("\n\n").collect();
    sums = parts.iter()
        .map(|numstr| {
            numstr.split("\n")
                .map(|x| x.parse::<u32>().unwrap())
                .sum()
        }).collect();

    println!("{}", sums.iter().max().unwrap());

    Ok(())
}