use serde::Deserialize;
//Structs for Posts
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
pub struct PostInfo {
    pub post: Post,
    //There are more fields but we don't care
}
#[derive(Deserialize, Debug)]
pub struct PostObj {
    pub posts: Vec<PostInfo>,
}
//Structs for Comments
#[derive(Deserialize, Debug, Default)]
pub struct Comment {
    pub content: Option<String>,
    pub parent_id: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct CommentInfo {
    pub comment: Option<Comment>,
    //There are more fields but we don't care
}

#[derive(Deserialize, Debug)]
pub struct CommentObj {
    pub comments: Vec<CommentInfo>,
    //There are more fields but we don't care
}

//Api Fetching Functions

pub fn get_posts(url: String) -> Result<Vec<PostInfo>, reqwest::Error> {
    let response = reqwest::blocking::get(url)?;
    return Ok(response.json::<PostObj>()?.posts);
}
pub fn get_comments(url: String) -> Result<Vec<CommentInfo>, reqwest::Error> {
    let response = reqwest::blocking::get(url)?;
    return Ok(response.json::<CommentObj>()?.comments);
}
