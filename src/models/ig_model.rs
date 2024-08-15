use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaResponse {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerResponse {
    pub id: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PublishResponse {
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaUpload<'a> {
    pub media_type: &'a str,
    pub image_url: Option<&'a str>,
    pub video_url: Option<&'a str>,
    pub caption: Option<&'a str>,
    pub is_carousel_item: bool,
    pub upload_type: Option<&'a str>, // resumable, none
    pub access_token: Option<&'a str>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CarouselChild<'a> {
    pub media_type: &'a str,
    pub media_id: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CarouselUpload<'a> {
    pub media_type: &'a str,
    pub children: &'a str,
    pub is_carousel_item: bool,
    pub upload_type: Option<&'a str>, // resumable, none
    pub caption: Option<&'a str>,
    pub access_token: Option<&'a str>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaPublish<'a> {
    pub creation_id: &'a str,
    pub access_token: Option<&'a str>,
}
