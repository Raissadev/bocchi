use tokio;
use dotenvy::dotenv;
use std::env;
use jobs::yt_job::YTJob as job;
use services::lib_service::LibService as sv;
mod config {
    pub mod database;
}
mod repositories {
    pub mod lib_repository;
}
mod models {
    pub mod lib_model;
    pub mod  ig_model;
}
mod services {
    pub mod lib_service;
    pub mod ig_service;
}
mod jobs {
    pub mod yt_job;
}
mod utils{
    pub mod fmt;
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Starting Application... 42");

    let args: Vec<String> = env::args().skip(1).collect();
    let mut argsi = args.iter();

    if !args.is_empty() {
        while let Some(arg) = argsi.next() {
            match arg.as_str() {
                "-v" => {
                    if let Some(link) = argsi.next() {
                        if let Err(e) = sv::get_fvideo(link).await {
                            eprintln!("Error fetching playlist: {}", e);
                        }
                    } else {
                        println!("Err: expected link of video");
                    }
                }
                "-r" => {
                    let _ = sv::ls_playlist().await;
                }
                "-vr" => {
                    if let Err(e) = sv::random_video().await {
                        log::error!("Failed to get random video: {:?}", e);
                    }
                }
                _ => {
                    println!("Arg unknown: {}", arg);
                }
            }   
        }
        return;
    }

    let _ = job::init().await;
}
