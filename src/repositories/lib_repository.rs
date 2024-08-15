use chrono::Utc;
use crate::config::database::Database;
use crate::models::lib_model::{self as lmodel, PltVideo};

pub struct LibRepository {
}

impl LibRepository
{
    pub async fn set(plt: lmodel::PlaylistItemListResponse) -> ()
    {
        let db: Database = Database::new().await;
        let mut tx = db.pool.begin().await.unwrap();
        let query = r#"
            INSERT INTO plt_video (id_video, etag, kind, published_at, channel_id, title)
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT DO NOTHING
        
        "#;
        for item in plt.items {
            let r = sqlx::query(&query)
                .bind(item.contentDetails.videoId)
                .bind(item.etag)
                .bind(item.kind)
                .bind(item.contentDetails.videoPublishedAt)
                .bind(item.snippet.channelId)
                .bind(item.snippet.title)
                .execute(&mut *tx)
                .await;
            if r.is_err() {
                let _ = tx.rollback().await;
                log::error!("Failed to set videos: {:?}", r.err());
                return;
            }
        }
    
        let _ = tx.commit().await;
        println!("Ok");
    }
    
    pub async fn update(video_id: &str) -> ()
    {
        let db: Database = Database::new().await;
        let mut tx = db.pool.begin().await.unwrap();
        let timestamp = Utc::now();
        let query = r#"
            UPDATE plt_video SET
                posted_at = ?
            WHERE id_video = ?
        
        "#;
        let r = sqlx::query(&query)
            .bind(timestamp.to_string())
            .bind(video_id)
            .execute(&mut *tx)
            .await;
        if r.is_err() {
            let _ = tx.rollback().await;
            log::error!("Failed to update plt: {:?}", r.err());
            return;
        }
    
        let _ = tx.commit().await;
        println!("Ok");
    }


    pub async fn today_video() -> Result<PltVideo, Box<dyn std::error::Error>>
    {
        let db: Database = Database::new().await;
        let mut tx = db.pool.begin().await.unwrap();
        let query = r#"
            SELECT
                pv.id
            ,   pv.id_video
            ,   pv.etag
            ,   pv.kind
            ,   published_at
            ,   channel_id
            ,   title
            FROM plt_video pv 
            WHERE pv.posted_at IS NULL AND DATE(pv.published_at) = DATE('now')
            ORDER BY pv.id DESC
            LIMIT 1
        "#;
        let r = sqlx::query_as::<_, lmodel::PltVideo>(&query)
            .fetch_one(&mut *tx)
            .await;

        match r {
            Ok(row) => {
                let _ = tx.commit().await;
                println!("ok");
                Ok(row)
            }
            Err(e) => {
                let _ = tx.rollback().await;
                log::error!("Failed to get today video: {:?}", e);
                Err(Box::new(e))
            }
        }
    }

    pub async fn random_video() -> Result<PltVideo, Box<dyn std::error::Error>>
    {
        let db: Database = Database::new().await;
        let mut tx = db.pool.begin().await.unwrap();
        let query = r#"
            SELECT
                pv.id
            ,   pv.id_video
            ,   pv.etag
            ,   pv.kind
            ,   published_at
            ,   channel_id
            ,   title
            FROM plt_video pv 
            WHERE pv.posted_at IS NULL
            ORDER BY RANDOM()
            LIMIT 1
        "#;
        let r = sqlx::query_as::<_, lmodel::PltVideo>(&query)
            .fetch_one(&mut *tx)
            .await;
    
        match r {
            Ok(row) => {
                let _ = tx.commit().await;
                println!("ok");
                Ok(row)
            }
            Err(e) => {
                let _ = tx.rollback().await;
                log::error!("Failed to get random video: {:?}", e);
                Err(Box::new(e))
            }
        }
    }
}

