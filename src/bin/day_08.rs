use std::fs;
use std::io;

fn distance_score(direct: &[u32], tree: &u32) -> u32 {
    let mut score = 1;
    for trees in direct {
        if trees >= tree {
            return score;
        }
        score += 1;
    }
    return score - 1;
}

fn main() -> Result<(), io::Error>{
    let input = fs::read_to_string("./resources/input_8_1.txt")?;
    let data: Vec<u32> = input.split("\n").map(|s| {
        s.chars().map(|ch| {
            ch.to_digit(10).unwrap()
        }).collect::<Vec<u32>>()
    }).flatten().collect();

    let columns: Vec<Vec<u32>> = (0..99)
        .map(|index| data.clone().into_iter().skip(index).step_by(99).collect()).collect();

    // 394
    let mut outer: u32 = 0;
    let mut view_scores: Vec<u32> = vec![]; 

    for (y, trees) in data.chunks(99).enumerate() {
        for (x, tree) in trees.iter().enumerate() {
            let left = &trees[0..x];
            let right = &trees[x + 1..99];
            let up = &columns[x][0..y];
            let down = &columns[x][y + 1..99];

            if left.iter().all(|rest| tree > rest) || right.iter().all(|rest| tree > rest) || up.iter().all(|rest| tree > rest) || down.iter().all(|rest| tree > rest) {
                outer += 1;
            }

            let mut left: Vec<u32> = left.iter().map(|x| x.clone()).collect();
            left.reverse();
            let mut up: Vec<u32> = up.iter().map(|x| x.clone()).collect();
            up.reverse();

            view_scores.push(distance_score(&left[0..left.len()], tree) * distance_score(right, tree) * distance_score(&up[0..up.len()], tree) * distance_score(down, tree));
        }
    }

    println!("Visible Trees: {}", outer);
    println!("Highest Score: {}", view_scores.iter().max().unwrap());

    Ok(())
}