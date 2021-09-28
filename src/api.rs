use super::utils;
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
#[derive(Deserialize)]
pub struct PostInfo {
    pub post: Post,
    pub creator: Creator,
    pub community: Community,
    pub counts: PostCounts,
    //There are more fields but we don't care
}
#[derive(Deserialize)]
pub struct PostObj {
    pub posts: Vec<PostInfo>,
}
//Structs for Comments
#[derive(Deserialize, Debug, Default, Clone)]
pub struct Comment {
    pub id: i32,
    pub content: String,
    pub parent_id: Option<i32>,
}

#[derive(Deserialize, Default, Clone)]
pub struct CommentInfo {
    pub comment: Option<Comment>,
    pub creator: Creator,
    //There are more fields but we don't care
}

#[derive(Deserialize)]
pub struct CommentObj {
    pub comments: Vec<CommentInfo>,
    //There are more fields but we don't care
}
#[derive(Default, Clone)]
pub struct CommentTree {
    pub comment: CommentInfo,
    pub children: Vec<CommentTree>,
}
impl CommentTree {
    fn new(comment: &CommentInfo) -> Self {
        Self {
            comment: comment.clone(),
            children: vec![],
        }
    }

    fn fill_children(mut self, comments: &Vec<CommentInfo>) -> Self {
        for i in 0..comments.len() {
            let clone = comments.clone();
            if comments[i]
                .comment
                .as_ref()
                .unwrap_or(&Comment::default())
                .parent_id
                .unwrap_or_default()
                == self
                    .comment
                    .comment
                    .as_ref()
                    .unwrap_or(&Comment::default())
                    .id
            {
                self.children
                    .push(CommentTree::new(&comments[i]).fill_children(&clone));
            }
        }
        return self;
    }
}
#[derive(Deserialize)]
struct LoginResponse {
    jwt: Option<String>,
}
#[derive(serde::Serialize)]
pub struct LoginForm {
    username_or_email: String,
    password: String,
}
impl LoginForm {
    pub fn new(login: String, pass: String) -> Self {
        Self {
            username_or_email: login,
            password: pass,
        }
    }
}
#[derive(Deserialize, Default, Clone)]
pub struct Creator {
    pub name: String,
}
#[derive(Deserialize, Default, Clone)]
pub struct Community {
    pub name: String,
}
#[derive(Deserialize, Default, Clone)]
pub struct PostCounts {
    pub comments: i64,
}
//Api Fetching Functions

pub fn get_posts(url: String, auth: &str, config: &str) -> Result<Vec<PostInfo>, reqwest::Error> {
    let response;
    if auth.is_empty() {
        response = reqwest::blocking::get(url + config)?;
    } else {
        response = reqwest::blocking::get(url + "auth=" + &auth + config)?
    }
    return Ok(response.json::<PostObj>()?.posts);
}
pub fn get_comments(url: String, auth: &str) -> Result<Vec<CommentTree>, reqwest::Error> {
    let response;
    if auth.is_empty() {
        response = reqwest::blocking::get(url)?;
    } else {
        response = reqwest::blocking::get(url + "auth=" + &auth)?
    }
    let comments = response.json::<CommentObj>()?.comments;
    let clone = comments.clone();
    let filtered_comments: Vec<CommentInfo> = comments
        .into_iter()
        .filter(|c| {
            !c.comment
                .as_ref()
                .unwrap_or(&Comment::default())
                .parent_id
                .is_some()
        })
        .collect();
    let result = utils::map_tree(filtered_comments);
    //result.iter().map(|r|r.fill_children(&clone)).collect()
    return Ok(result
        .into_iter()
        .map(|r| r.fill_children(&clone))
        .collect());
}

pub fn login(url: String, login: String, pass: String) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let response = client.post(url).json(&LoginForm::new(login, pass)).send()?;
    return Ok(response.json::<LoginResponse>()?.jwt.unwrap_or_default());
}
