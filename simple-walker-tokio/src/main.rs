use std::env;
use std::process::exit;
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
        rt.block_on(walk(folder.to_string()));
        let start = Instant::now();
        rt.block_on(walk(folder.to_string()));
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
            let mut handles = vec!();
            while let Ok(Some(entry)) = dir.next_entry().await {
                if entry.file_name().to_str().unwrap().starts_with(".") {
                    continue;
                }
                let path = entry.path();
                let path_string = path.to_str().unwrap().to_string();
                if path.is_dir() {
                    handles.push(tokio::spawn(walk(path_string)));
                }
                if path.is_file() {
                    //TODO: Do actual work
                }
            }
            futures::future::join_all(handles).await;
        }
    });
}
