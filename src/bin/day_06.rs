use std::fs;
use std::io;
use std::collections::HashSet;

fn main() -> Result<(), io::Error> {
    let buffer = fs::read_to_string("./resources/input_6_1.txt")?;
    let mut temp = String::new();
    for (index, ch) in buffer.chars().enumerate() {
        temp.push(ch);
        if temp.len() == 4 {
            let set: HashSet<char> = temp.chars().collect();
            if set.len() == 4 {
                println!("Index 1 is: {}", index - 1);
                break;
            }
            temp.clear();
        }   
    }

    for (index, ch) in buffer.chars().enumerate() {
        temp.push(ch);
        if temp.len() == 14 {
            let set: HashSet<char> = temp.chars().collect();
            if set.len() == 14 {
                println!("Index 2 is: {}", index);
                break;
            }
            temp.clear();
        }   
    }
    

    Ok(())
}