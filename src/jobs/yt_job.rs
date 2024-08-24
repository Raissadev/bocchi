use chrono::Utc;
use cron::Schedule;
use std::str::FromStr;
use crate::services::lib_service::LibService as serv;
use anyhow::{Result};

// Jobs to run daily, without the need for separation
pub struct YTJob {
}

impl YTJob {
    pub async fn init() -> Result<()>
    {
        // avoudings
        let (plt_result, random_result, today_result) = tokio::try_join!(
            Self::run_plt(),
            Self::run_random_v(),
            Self::run_today_v()
        )?;

        Ok(())
    }

    async fn run_plt() -> Result<()> 
    {
        println!("Starting cron playlist");
        let schedule = Schedule::from_str("0 0 0 * * *").expect("Invalid cron");
        loop {
            let now = Utc::now();
            let next = schedule
                .upcoming(Utc)
                .next()
                .expect("No upcoming schedule");
            let d_until_next = next
                .signed_duration_since(now)
                .to_std()
                .expect("Invalid duration");
    
            println!("Next execution at: {}", next);
            tokio::time::sleep(d_until_next).await;
    
            if let Err(e) = serv::today_video().await {
                log::error!("Failed to get today's video: {:?}", e);
            }
        }
    }

    async fn run_random_v() -> Result<()>
    {
        println!("Starting cron random");
        let schedule = Schedule::from_str( "0 0 12 * * *").expect("Invalid cron");
        loop {
            let now = Utc::now();
            let next = schedule
                .upcoming(Utc)
                .next()
                .expect("No upcoming schedule");
            let d_until_next = next
                .signed_duration_since(now)
                .to_std()
                .expect("Invalid duration");
    
            println!("Next execution at: {}", next);
            tokio::time::sleep(d_until_next).await;
    
            if let Err(e) = serv::random_video().await {
                log::error!("Failed to get random video: {:?}", e);
            }
        }
    }

    async fn run_today_v() -> Result<()>
    {
        println!("Starting cron today");
        let schedule = Schedule::from_str( "0 0 23 * * *").expect("Invalid cron");
        loop {
            let now = Utc::now();
            let next = schedule
                .upcoming(Utc)
                .next()
                .expect("No upcoming schedule");
            let d_until_next = next
                .signed_duration_since(now)
                .to_std()
                .expect("Invalid duration");
    
            println!("Next execution at: {}", next);
            tokio::time::sleep(d_until_next).await;
    
            if let Err(e) = serv::today_video().await {
                log::error!("Failed to get today's video: {:?}", e);
            }
        }
    }
}
