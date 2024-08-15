use tokio;
use dotenvy::dotenv;
use jobs::yt_job::YTJob as job;
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

    let _ = job::init().await;
}
