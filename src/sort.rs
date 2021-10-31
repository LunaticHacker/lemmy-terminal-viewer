use super::api::PostInfo;
use chrono::NaiveDateTime;
use std::str::FromStr;

pub enum SortType {
    Top,
    Old,
    New,
}
pub fn sort(st: SortType, posts: &mut Vec<PostInfo>) {
    match st {
        SortType::New => {
            posts.sort_by(|a, b| {
                NaiveDateTime::from_str(&b.post.published)
                    .unwrap()
                    .cmp(&NaiveDateTime::from_str(&a.post.published).unwrap())
            });
        }
        SortType::Old => {
            posts.sort_by(|a, b| {
                NaiveDateTime::from_str(&a.post.published)
                    .unwrap()
                    .cmp(&NaiveDateTime::from_str(&b.post.published).unwrap())
            });
        }
        SortType::Top => posts.sort_by(|b, a| a.counts.score.cmp(&b.counts.score)),
    }
}
