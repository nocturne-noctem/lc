use std::fs::File;
use std::io::prelude::*;

pub fn run(filename: &str) {
    let error_msg: String = "Couldn't find file: ".to_owned();
    let mut file = File::open(filename).expect(&error_msg);

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Couldn't read file!");
    let line_count = count_lines(&contents);
    let word_count = count_words(&contents);
    println!("Lines: {}\tWords: {}\t{}", line_count, word_count, filename);
}

fn count_lines(contents: &String) -> i32 {
    let mut line_count: i32 = 0;
    for line in contents.lines() {
        if line.to_owned().len() > 0 {
            line_count = line_count + 1;
        }
    }

    line_count
}

fn count_words(contents: &String) -> usize {
    let word_count: usize = contents.split_whitespace().count();
    
    word_count
}
