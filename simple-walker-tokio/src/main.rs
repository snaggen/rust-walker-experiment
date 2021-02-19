use std::env;
use std::process::exit;
use std::path::Path;
use std::time::Instant;
use std::pin::Pin;
use std::future::Future;
use tokio::fs::read_dir;
use tokio::runtime::Runtime;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(folder) = args.get(1) {
        let rt = Runtime::new().unwrap();
        //warmup
        rt.block_on(walk(Path::new(folder)));
        let start = Instant::now();
        rt.block_on(walk(Path::new(folder)));
        let total = start.elapsed();
        println!("Time {:#?}", total);
    } else {
        eprintln!("A folder must be specified as an cmd line argument");
        exit(-1);
    }
}

fn walk<'a>(path: &'a Path) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
    return Box::pin(async move {
        for mut dir in read_dir(path).await {
            while let Ok(Some(entry)) = dir.next_entry().await {
                if entry.file_name().to_str().unwrap().starts_with(".") {
                    continue;
                }
                let path = entry.path();
                if path.is_dir() {
                    walk(&path).await;
                }
                if path.is_file() {
                    //TODO: Do actual work
                }
            }
        }
    });
}
