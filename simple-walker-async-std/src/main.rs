use async_std::fs::read_dir;
use async_std::prelude::{Future, StreamExt};
use async_std::task;
use std::env;
use std::pin::Pin;
use std::process::exit;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(folder) = args.get(1) {
        //warmup
        task::block_on(walk(folder.to_string()));
        let start = Instant::now();
        task::block_on(walk(folder.to_string()));
        let total = start.elapsed();
        println!("Time {:#?}", total);
    } else {
        eprintln!("A folder must be specified as an cmd line argument");
        exit(-1);
    }
}

fn walk<'a>(path: String) -> Pin<Box<dyn Future<Output = ()> + 'a + Send>> {
    return Box::pin(async move {
        for mut dir in read_dir(path).await {
            let mut handles = vec![];
            while let Some(Ok(entry)) = dir.next().await {
                if entry.file_name().to_str().unwrap().starts_with(".") {
                    continue;
                }
                if entry.file_type().await.unwrap().is_dir() {
                    let path_string = entry.path().to_str().unwrap().to_string();
                    handles.push(async_std::task::spawn(walk(path_string)));
                } else {
                    //TODO: Do actual work
                }
            }
            futures::future::join_all(handles).await;
        }
    });
}
