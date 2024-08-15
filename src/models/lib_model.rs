use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistItemSnippet {
    pub publishedAt: String,
    pub channelId: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistItem {
    pub kind: String,
    pub etag: String,
    pub id: String,
    pub snippet: PlaylistItemSnippet,
    pub contentDetails: ContentDetails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistItemListResponse {
    pub kind: String,
    pub etag: String,
    pub nextPageToken: Option<String>,
    pub items: Vec<PlaylistItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoResponse {
    pub items: Vec<VideoItem>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct VideoItem {
    pub id: String,
    pub snippet: Snippet,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Snippet {
    pub publishedAt: Option<String>,
    pub channelId: Option<String>,
    pub title: String,
    pub description: String,
    pub thumbnails: Thumbnails,
    pub channelTitle: Option<String>,
    pub tags: Option<Vec<String>>,
    pub categoryId: Option<String>,
    pub liveBroadcastContent: Option<String>,
    pub localized: Localized,
    pub defaultAudioLanguage: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentDetails {
    pub videoId: String,
    pub startAt: Option<String>,
    pub endAt: Option<String>,
    pub note: Option<String>,
    pub videoPublishedAt: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thumbnails {
    pub default: Thumbnail,
    pub medium: Option<Thumbnail>,
    pub high: Thumbnail,
    pub standard: Option<Thumbnail>,
    pub maxres: Option<Thumbnail>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thumbnail {
    pub url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Localized {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct PltVideo {
    pub id: i32,
    pub id_video: String,
    pub etag: String,
    pub kind: String,
    pub published_at: String,
    pub channel_id: String,
    pub title: String,
}
