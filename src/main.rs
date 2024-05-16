use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::{self, File},
    io::{stdin, stdout, BufReader, Write},
};

use serde_json::Value;

fn main() {
    println!("Loading words...");
    let words = load_words();
    println!("{} words loaded.", words.values().len());

    let mut lines = stdin().lines();
    loop {
        println!("Please enter the letters:");
        let _ = stdout().flush();
        let letters = lines.next().unwrap().unwrap();
        let sorted = {
            let mut sorted = letters.chars().collect::<Vec<_>>();
            sorted.sort();
            String::from_iter(sorted)
        };
        if let Some(found_words) = find_words_q(&words, &sorted) {
            println!("Found words:");
            for word in found_words {
                println!("{}", word);
            }
        } else {
            println!("No words found.");
        }
    }
}

fn find_words_q<'a>(words: &'a HashMap<String, Vec<String>>, sorted: &str) -> Option<Vec<&'a str>> {
    let mut found_words = HashSet::<&str>::new();
    let mut q = VecDeque::<String>::new();
    q.push_front(sorted.to_string());
    let mut best_len = 0;
    while !q.is_empty() {
        let v = q.pop_back().unwrap();
        if let Some(words) = words.get(&v) {
            if words.first().unwrap().len() < best_len {
                break;
            }
            best_len = words.first().unwrap().len();
            found_words.extend(words.iter().map(|s| s.as_str()));
        }
        for i in 0..v.len() {
            let chars = v.chars().collect::<Vec<_>>();
            let new_str = if i < (chars.len() - 1) {
                (chars[0..i].iter().chain(chars[i + 1..].iter())).collect()
            } else {
                (chars[0..i].iter()).collect()
            };
            q.push_front(new_str);
        }
    }
    let mut result = found_words.iter().cloned().collect::<Vec<_>>();

    result.sort_by(|a, b| match a.len().cmp(&b.len()).reverse() {
        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
        std::cmp::Ordering::Equal => a.cmp(b),
        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
    });
    if !found_words.is_empty() {
        return Some(result);
    }
    None
}

fn load_words() -> HashMap<String, Vec<String>> {
    let mut words = HashMap::<String, Vec<String>>::new();
    let paths = fs::read_dir("./wordset-dictionary/data")
        .unwrap()
        .filter_map(|r| r.ok())
        .collect::<Vec<_>>();
    for path in paths {
        let rdr = BufReader::new(File::open(path.path().as_path()).unwrap());
        let v: Value = serde_json::from_reader(rdr).unwrap();
        match v {
            Value::Object(obj) => {
                for (_, v) in obj {
                    match &v["word"] {
                        Value::String(s) => {
                            let word = s;
                            let mut sorted = s.chars().collect::<Vec<_>>();
                            sorted.sort();
                            let key = String::from_iter(sorted);
                            words
                                .entry(key)
                                .and_modify(|words| words.push(word.to_string()))
                                .or_insert_with(|| vec![word.to_string()]);
                        }
                        _ => println!("Unexpected JSON value"),
                    }
                }
            }
            _ => println!("Unexpected JSON value"),
        }
    }
    words
}
