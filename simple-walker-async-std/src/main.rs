use std::env;
use std::process::exit;
use std::time::Instant;
use std::pin::Pin;
use std::future::Future;
use async_std::fs::read_dir;
use async_std::task;
use async_std::path::PathBuf;
use futures::StreamExt;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(folder) = args.get(1) {
        //warmup
        task::block_on(walk(&PathBuf::from(folder)));
        let start = Instant::now();
        task::block_on(walk(&PathBuf::from(folder)));
        let total = start.elapsed();
        println!("Time {:#?}", total);
    } else {
        eprintln!("A folder must be specified as an cmd line argument");
        exit(-1);
    }
}

fn walk<'a>(path: &'a PathBuf) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
    return Box::pin(async move {
        for mut dir in read_dir(path).await {
            while let Some(Ok(entry)) = dir.next().await {
                if entry.file_name().to_str().unwrap().starts_with(".") {
                    continue;
                }
                let path = entry.path();
                if path.is_dir().await {
                    walk(&path).await;
                }
                if path.is_file().await {
                    //TODO: Do actual work
                }
            }
        }
    });
}

