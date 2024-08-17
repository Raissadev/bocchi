use std::{env, fs::{self}, path::Path};
use reqwest::{Error, Client};
use crate::{
    models::lib_model::{self as lm}
,   utils::fmt::{calc_timing, format_publish_date}
,   models::ig_model::{self as im}
};
use std::process::Command;
use crate::repositories::lib_repository::LibRepository as repo;
use crate::services::ig_service::IGService as igserv;
use tokio::time::sleep;
use std::time::Duration;
use std::error::Error as StdError;

pub struct LibService;

impl LibService {
    pub async fn ls_playlist() -> Result<(), Error>
    {
        let mut agg = Vec::new();
        let mut next_page_token = None;
        let client = Client::new();
        
        // get items of playlist and push to agg
        loop {
            let url = format!(
                "{}/playlistItems?playlistId={}&key={}&maxResults=50{}&part=snippet,contentDetails",
                env::var("API_YT").expect("API_YT"),
                env::var("YT_DLP_ID").expect("YT_DLP_ID"),
                env::var("YT_DLP_KEY").expect("YT_DLP_KEY"),
                next_page_token.map_or(String::new(), |token| format!("&pageToken={}", token))
            );

            let playlist_items: lm::PlaylistItemListResponse = client.get(&url)
                .send()
                .await?
                .json()
                .await?;
    
            agg.extend(playlist_items.items);
            if let Some(token) = playlist_items.nextPageToken {
                next_page_token = Some(token);
            } else {
                break;
            }
        }
    
        repo::set(lm::PlaylistItemListResponse{
            kind: String::new(),
            etag: String::new(),
            nextPageToken: None,
            items: agg,
        }).await;
        Ok(())
    }

    pub async fn get_fvideo(video_id: &str) -> Result<(), Box<dyn StdError>>
    {
        println!("Init get of yt...");
        let url = format!(
            "{}/videos?id={}&part=snippet&key={}",
            env::var("API_YT").expect("API_YT"),
            video_id,
            env::var("YT_DLP_KEY").expect("YT_DLP_KEY"),
        );
        let response = reqwest::get(&url).await?;
        let body = response.text().await?;
        let video_inf: lm::VideoResponse = serde_json::from_str(&body).unwrap();
        if let Some(item) = video_inf.items.into_iter().next() {
            // down, post and put
            if let Ok(outpath) = Self::down(video_id).await {
                let _ = Self::post(&item, &outpath).await?;
                let _ = repo::update(video_id).await;
                let _ = Self::rm(&outpath);
            } else {
                return Err("Failed down".into());
            }
        }  else {
            return Err("Failed get info plt".into());
        }
        Ok(())
    }
    
    pub async fn down(video_id: &str) -> Result<String, Box<dyn StdError>>
    {
        let filename = format!("v.{}.mp4", video_id);
        let out_path = format!("{}/tmp.{}", env::var("PATH_FILES").expect("PATH_FILES"), filename);
        let compress_path = format!("{}/{}", env::var("PATH_FILES").expect("PATH_FILES"), filename);
        let in_path = format!("{}/watch?v={}", env::var("YT_URL").expect("YT_URL"), video_id);

        let yt_dlp = Command::new("yt-dlp")
            .arg("-f")
            .arg("bv[height=1080]+ba/b[height=1080]")
            .arg("--merge-output-format")
            .arg("mp4")
            .arg(&in_path)
            .arg("-o")
            .arg(&out_path)
            .output()
            .expect("Failed to download");


        if !yt_dlp.status.success() {
            let stderr = String::from_utf8_lossy(&yt_dlp.stderr);
            eprintln!("{:?}", stderr);
            return Err(format!("Download failed: {}", stderr).into());
        }

        println!("Download successful, starting compression...");
        let compress_output = Command::new("ffmpeg")
            .arg("-i")
            .arg(&out_path)
            .arg("-ss")
            .arg("00:00:20") 
            .arg("-to")
            .arg("00:01:20") 
            .arg("-vf")
            .arg("scale=-2:720")
            .arg("-c:v")
            .arg("libx264")
            .arg("-profile:v")
            .arg("main")
            .arg("-level:v")
            .arg("3.0")
            .arg("-x264-params")
            .arg("scenecut=0:open_gop=0:min-keyint=72:keyint=72")
            .arg("-c:a")
            .arg("copy")
            .arg("-preset")
            .arg("slow")
            .arg("-crf")
            .arg("23")
            .arg("-r")
            .arg("30")
            .arg("-sn")
            .arg("-f")
            .arg("mp4")
            .arg(&compress_path)
            .output()
            .expect("Failed to compress");

        if compress_output.status.success() {
            println!("Compression successful.");
            Ok(filename)
        } else {
            let stderr = String::from_utf8_lossy(&compress_output.stderr);
            eprintln!("Compression failed: {}", stderr);
            return Err(format!("Compression failed: {}", stderr).into());
        }
    }

    pub fn rm(path: &str) -> ()
    { 
        let dir = env::var("PATH_FILES").expect("PATH_FILES");
        let file = Path::new(&dir).join(path);
        let file_tmp = Path::new(&dir).join(format!("tmp.{}", path));
        let _ = fs::remove_file(file);
        let _ = fs::remove_file(file_tmp);
    }

    pub async fn post(data: &lm::VideoItem, path: &String) -> Result<bool, Box<dyn StdError>>
    {
        println!("Starting post...");
        let client = Client::new();
        let tk = env::var("API_GRAPH_TK").expect("API_GRAPH_TK");
        let tk_fb = env::var("API_FB_GRAPH_TK").expect("API_FB_GRAPH_TK");
        let f_path = format!("{}/{}", env::var("HOST").expect("HOST"), path);
        
        println!("Init upload video...");
        let v_upl = im::MediaUpload {
            media_type: "VIDEO",
            image_url: None,
            video_url: Some(&f_path),
            caption: None,
            is_carousel_item: true,
            upload_type: Some("resumable"),
            access_token: Some(&tk),
        };

        let vc_id = igserv::upload_media(&client,&v_upl).await;
        match &vc_id {
            Ok(id) => println!("Media upload, ID: {}", id),
            Err(e) => {
                eprintln!("Failed to upload: {:?}", e);
                return Err("Failed to upload".into());
            }
        }
        sleep(Duration::from_secs(360)).await;

        println!("Init upload photo...");
        let img_url = if let Some(ref maxres) = data.snippet.thumbnails.maxres {
            &maxres.url
        } else if let Some(ref standard) = data.snippet.thumbnails.standard {
            &standard.url
        } else {
            &data.snippet.thumbnails.high.url
        };
        let img_upl = im::MediaUpload {
            media_type: "IMAGE",
            image_url: Some(&img_url),
            video_url: None,
            caption: None,
            is_carousel_item: true,
            upload_type: None,
            access_token: Some(&tk),
        };
    
        let imgc_id = igserv::upload_media(&client,&img_upl).await;
        match &imgc_id {
            Ok(id) => println!("Media upload, ID: {}", id),
            Err(e) => {
                eprintln!("Failed to upload: {:?}", e);
                return Err("Failed to upload".into());
            }
        }

        sleep(Duration::from_secs(60)).await;

        println!("Init upload group...");
        let description = format!(
            "{}: {}. \nCanal: {}\n{}\n\n\n{}",
            calc_timing(data.snippet.publishedAt.as_ref())
        ,   &data.snippet.title
        ,   &data.snippet.channelTitle.as_deref().unwrap_or("unknown")
        ,   format_publish_date(data.snippet.publishedAt.as_deref())
        ,   String::from("#42 #mscgeek #rapgeek")
        );

        let ax = imgc_id.unwrap();
        let ay = vc_id.unwrap();
        let ls = vec![ax, ay].join(",");
        let c_upl = im::CarouselUpload {
            media_type: "CAROUSEL",
            children: &ls,
            caption: Some(&description),
            is_carousel_item: false,
            access_token: Some(&tk_fb),
            upload_type: None,
        };
        let c_id = igserv::set_carrousselc(&client, &c_upl).await;
        match &c_id {
            Ok(id) => println!("Container, ID: {}", id),
            Err(e) => {
                eprintln!("Failed to upload: {:?}", e);
                return Err("Failed to upload".into());
            }
        }

        sleep(Duration::from_secs(60)).await;

        println!("Init publish...");
        let carousel_publish = im::MediaPublish {
            creation_id: &c_id.unwrap(),
            access_token: Some(&tk_fb),
        };

        let pub_id = igserv::publish(&client, &carousel_publish).await;
        match pub_id {
            Ok(id) => println!("Published successfully, ID: {}", id),
            Err(e) => {
                eprintln!("Failed to pub: {:?}", e);
                return Err("Failed pub".into());
            }
        }   

        Ok(true)
    }

    pub async fn today_video() -> Result<(), Error>
    {
        if let Ok(row) = repo::today_video().await {
            let _  = Self::get_fvideo(&row.id_video).await;
        }
        Ok(())
    }
    
    pub async fn random_video() -> Result<(), Error>
    {
        if let Ok(row) = repo::random_video().await {
            let _  = Self::get_fvideo(&row.id_video).await;
        }
        Ok(())
    }
    
}