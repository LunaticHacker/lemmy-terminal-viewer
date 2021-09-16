use super::api::{CommentInfo, CommentTree};
pub fn map_tree(list: Vec<CommentInfo>) -> Vec<CommentTree> {
    list.into_iter()
        .map(|ct| CommentTree {
            comment: ct,
            children: vec![],
        })
        .collect()
}
pub fn prepend_https(mut str: String) -> String {
    if !str.starts_with("https://") {
        str.insert_str(0, "https://");
        return str;
    } else {
        str
    }
}

pub fn get_comments(cursor: Vec<usize>, list: Vec<CommentTree>) -> Vec<CommentTree> {
    let mut result = list;
    for item in cursor {
        result = result[item].children.clone();
    }
    return result;
}
