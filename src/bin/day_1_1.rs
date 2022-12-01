use std::fs;
use std::io;


fn main() -> Result<(), io::Error>{
    let data = fs::read_to_string("resources/input_1_1.txt")?;

    let parts: Vec<&str> = data.split("\n\n").collect();
    let sums: Vec<u32> = parts.iter()
        .map(|numstr| {
            numstr.split("\n")
                .map(|x| x.parse::<u32>().unwrap())
                .sum()
        }).collect();

    println!("{}", sums.iter().max().unwrap());

    Ok(())
}