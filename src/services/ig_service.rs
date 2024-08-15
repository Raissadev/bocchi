use std::env;
use reqwest::Client;
use crate::models::ig_model::{self as lm};
use std::error::Error as StdError;

pub struct IGService;

impl IGService {
    pub async fn upload_media<'a>(client: &Client, media: &lm::MediaUpload<'a>) -> Result<String, Box<dyn StdError>> 
    {
        let url = format!(
            "{}/{}/media", env::var("API_GRAPH").expect("API_GRAPH"), env::var("IG_ID").expect("IG_ID")
        );

        let res: lm::MediaResponse = client.post(&url)
            .json(&media)
            .send()
            .await?
            .json()
            .await?;
        
        Ok(res.id)
    }

    pub async fn set_carrousselc<'a>(client: &Client, media: &lm::CarouselUpload<'a>) -> Result<String, Box<dyn StdError>> 
    {
        let url = format!(
            "{}/{}/media", env::var("API_GRAPH_FB").expect("API_GRAPH_FB"), env::var("IG_ID").expect("IG_ID")
        );

        let response_text = client.post(&url)
            .query(&media)
            .send()
            .await?
            .text()
            .await?;

        println!("Response Text: {}", response_text);

        let res: lm::ContainerResponse = serde_json::from_str(&response_text)?;

        if let Some(id) = res.id {
            Ok(id)
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Missing `id` in response")))
        }
    }

    pub async fn publish<'a>(client: &Client, media: &lm::MediaPublish<'a>) -> Result<String, Box<dyn StdError>> 
    {
        let url = format!(
            "{}/{}/media_publish",  env::var("API_GRAPH_FB").expect("API_GRAPH_FB"), env::var("IG_ID").expect("IG_ID")
        );

        let response_text = client.post(&url)
            .query(&media)
            .send()
            .await?
            .text()
            .await?;

            println!("Response Text: {}", response_text);
            let res: lm::PublishResponse = serde_json::from_str(&response_text)?;
            if let Some(id) = res.id {
                Ok(id)
            } else {
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Missing `id` in response")))
            }
    }
}