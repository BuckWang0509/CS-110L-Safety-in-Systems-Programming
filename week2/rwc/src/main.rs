use std::env;
use std::process;
use std::fs::File;
use std::io::{self,BufRead};

fn read_file_lines(filename: &String) -> Result<Vec<String>,io::Error> {
    let file = File::open(filename)?;
    let mut res = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let line_str = line?;
        res.push(line_str);
    }
    Ok(res)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    let result = read_file_lines(filename).expect("the file is not exist");
    let mut line_count = 0;
    let mut word_count = 0;
    let mut char_count = 0;
    for res in result.iter() {
        line_count += 1;
        for line in res.split(" ") {
            word_count += 1;
            char_count += line.len();
        } 
    }
    println!("The file has {} lines",line_count);
    println!("The file has {} words",word_count);
    println!("The file has {} chars",char_count);
}
