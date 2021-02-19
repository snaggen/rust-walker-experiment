use std::env;
use std::fs::read_dir;
use std::path::Path;
use std::process::exit;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(folder) = args.get(1) {
        //warmup
        walk(Path::new(folder));
        let start = Instant::now();
        walk(Path::new(folder));
        let total = start.elapsed();
        println!("Time {:#?}", total);
    } else {
        eprintln!("A folder must be specified as an cmd line argument");
        exit(-1);
    }
}

fn walk(path: &Path) {
    for mut dir in read_dir(path) {
        while let Some(Ok(entry)) = dir.next() {
            if entry.file_name().to_str().unwrap().starts_with(".") {
                continue;
            }
            let path = entry.path();
            if path.is_dir() {
                walk(&path);
            }
            if path.is_file() {
                //TODO: Do actual work
            }
        }
    }
}
