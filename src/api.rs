use serde::Deserialize;
use std::collections::HashMap;
//Structs for lemmy Api
#[derive(Deserialize, Debug)]
pub struct Post {
    pub id: i32,
    pub name: String,
    pub url: Option<String>,
    pub body: Option<String>,
    pub creator_id: i32,
    pub community_id: i32,
    pub removed: bool,
    pub locked: bool,
    pub published: Option<String>,
    pub updated: Option<String>,
    pub deleted: bool,
    pub nsfw: bool,
    pub stickied: bool,
    pub embed_title: Option<String>,
    pub embed_description: Option<String>,
    pub embed_html: Option<String>,
    pub thumbnail_url: Option<String>,
    pub ap_id: Option<String>,
    pub local: bool,
}
#[derive(Deserialize, Debug)]
pub struct Posts {
    pub post: Post,
    #[serde(skip)]
    creator: Option<HashMap<String, String>>,
    #[serde(skip)]
    counts: Option<HashMap<String, String>>,
    creator_banned_from_community: bool,
    subscribed: bool,
    saved: bool,
    read: bool,
    creator_blocked: bool,
    my_vote: Option<u32>,
}
#[derive(Deserialize, Debug)]
pub struct Obj {
    pub posts: Vec<Posts>,
}

//Api Fetching Functions

pub fn getposts(url: String) -> Result<Vec<Posts>, reqwest::Error> {
    let response = reqwest::blocking::get(url)?;
    return Ok(response.json::<Obj>()?.posts);
}
