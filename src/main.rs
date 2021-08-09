use std::{
    fs::read_dir,
    fs::File,
    io::{prelude::*, BufReader},

    path::Path,
};
use rayon::prelude::*;
use regex::Regex;
use std::ops::Index;

struct WordCount {
    word: String,
    count: i32
}

fn find_all_words(words: &str) -> Vec<String>{
    let mut list_string : Vec<String> = vec![];
    let re = Regex::new(r"(?m)\w+").unwrap();
    let result = re.captures_iter(words);
    for mat in result {
        list_string.push(mat.index(0).to_owned())
    }

    return list_string;


}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let path_str = "./files";
    let paths = read_dir(path_str).unwrap();

    paths.par_bridge()
        .map( |entry| entry.unwrap().path().file_name().unwrap().to_string_lossy().into_owned())
        .map(|filename|path_str.to_owned()+"/"+filename.as_str())
        .map(|fullpath|lines_from_file(fullpath))
        .reduce(|| Vec::new(),
                 |mut a: Vec<String>,mut b: Vec<String>| { a.append(&mut b); a })
        .par_iter().map(|sentences| find_all_words(sentences))
        .reduce(|| Vec::new(),
                |mut a: Vec<String>,mut b: Vec<String>| { a.append(&mut b); a })
        .par_iter().map(|words| WordCount{ word:words.to_string(), count:1})
        .for_each(|words | println!("{},{}",words.word,words.count));
}
