use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Clone)]
pub enum DirectoryContents {
    File(u64),
    Dir(String),
}

fn dir_size(dir_map: &HashMap<Vec<String>, Vec<DirectoryContents>>, dir: &(Vec<String>, Vec<DirectoryContents>)) -> u64 {
    let mut total_sum = 0;
    for contents in dir.1.clone() {
        let mut subdirs_size = 0;
        if let DirectoryContents::File(size) = contents {
            total_sum += size;
        } else {
            if let DirectoryContents::Dir(name) = contents {
                let mut new_dir = dir.0.clone();
                new_dir.push(name.to_owned());
                let subdirs = dir_map.get_key_value(&new_dir).unwrap().clone();
                subdirs_size += dir_size(dir_map, &(subdirs.0.to_owned(), subdirs.1.to_vec()));
            }
            total_sum += subdirs_size;
        }
    }
    total_sum
}

fn main() -> Result<(), io::Error> {
    let file_content = fs::read_to_string("./resources/input_7_1.txt")?;
    let mut working_dir: Vec<String> = vec!["/".to_string()];
    let mut dir_map: HashMap<Vec<String>, Vec<DirectoryContents>> = HashMap::new();
    let lines = file_content.lines();

    for line in lines {
        if line.starts_with("$") {
            let parts: Vec<&str> = line.split(" ").collect();
            if parts[1] == "cd" && parts[2] == ".." {
                working_dir.pop();
            } else if parts[1] == "cd" && parts[2] == "/" {
                working_dir = vec!["/".to_string()];
            } else if parts[1] == "cd" {
                working_dir.push(parts[2].to_string());
            }
        } else {
            if line.starts_with("dir") {
                let parts: Vec<&str> = line.split(" ").collect();
                if dir_map.contains_key(&working_dir) {
                    let dir_contents = dir_map.get_mut(&working_dir).unwrap();
                    dir_contents.push(DirectoryContents::Dir(parts[1].to_string()));
                } else {
                    dir_map.insert(working_dir.clone(), vec![DirectoryContents::Dir(parts[1].to_string())]);
                }
            } else {
                let parts: Vec<&str> = line.split(" ").collect();
                let size = parts[0].parse::<u64>().unwrap();
                if dir_map.contains_key(&working_dir) {
                    let vec = dir_map.get_mut(&working_dir).unwrap();
                    vec.push(DirectoryContents::File(size));
                } else {
                    dir_map.insert(working_dir.clone(), vec![DirectoryContents::File(size)]);
                }
            }
        }  
    }

    let mut sum_files = 0;
    for dir in dir_map.clone() {
        let res = dir_size(&dir_map, &dir);
        if res <= 100000 {
            sum_files += res;
        }
    }
    
    println!("Sum of files below 100000 is: {}", sum_files);

    let mut used_diskspace = 0;
    let mut dirs = vec![];

    for dir in dir_map.clone() {
        let res = dir_size(&dir_map, &dir);
        if dir.0 == vec!["/"] {
            used_diskspace = res;
        }
        dirs.push((dir.0, res));
    }

    let free_space = 70000000 - used_diskspace;
    let mut sorted_dirs = dirs.into_iter()
        .filter(|dir| free_space + dir.1 >= 30000000)
        .collect::<Vec<_>>();

    sorted_dirs.sort_by(|lhs, rhs| lhs.1.cmp(&rhs.1));
    let folder_size = sorted_dirs.first().unwrap().1;
    println!("Size of smallest sufficient folder: {}", folder_size);
        

    Ok(())
}