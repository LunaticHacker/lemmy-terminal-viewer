use super::api::{CommentInfo, CommentTree};
use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::str::FromStr;
use tui::style::Color;
pub enum SortType {
    Hot,
    Old,
    New,
}
pub fn map_tree(list: Vec<CommentInfo>) -> Vec<CommentTree> {
    list.into_iter()
        .map(|ct| CommentTree {
            comment: ct,
            children: vec![],
        })
        .collect()
}
pub fn prepend_https(mut str: String) -> String {
    if str.starts_with("localhost:") || str.starts_with("127.0.0.1") {
        str.insert_str(0, "http://");
        return str;
    }
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
fn parse_theme(theme_item: &str) -> Result<Color, std::num::ParseIntError> {
    let color = match theme_item {
        "Reset" => Color::Reset,
        "Black" => Color::Black,
        "Red" => Color::Red,
        "Green" => Color::Green,
        "Yellow" => Color::Yellow,
        "Blue" => Color::Blue,
        "Magenta" => Color::Magenta,
        "Cyan" => Color::Cyan,
        "Gray" => Color::Gray,
        "DarkGray" => Color::DarkGray,
        "LightRed" => Color::LightRed,
        "LightGreen" => Color::LightGreen,
        "LightYellow" => Color::LightYellow,
        "LightBlue" => Color::LightBlue,
        "LightMagenta" => Color::LightMagenta,
        "LightCyan" => Color::LightCyan,
        "White" => Color::White,
        _ => {
            let colors = theme_item.split(',').collect::<Vec<&str>>();
            if let (Some(r), Some(g), Some(b)) = (colors.get(0), colors.get(1), colors.get(2)) {
                Color::Rgb(
                    r.trim().parse::<u8>()?,
                    g.trim().parse::<u8>()?,
                    b.trim().parse::<u8>()?,
                )
            } else {
                Color::Black
            }
        }
    };

    Ok(color)
}
pub fn colorify(list: HashMap<String, String>) -> HashMap<String, Color> {
    let mut result = HashMap::new();

    for (key, value) in list {
        result.insert(key, parse_theme(&value).unwrap_or(Color::Black));
    }
    result
}

pub fn sort(st: SortType, ct: &mut Vec<CommentTree>) {
    match st {
        SortType::New => {
            ct.sort_by(|b, a| {
                NaiveDateTime::from_str(&a.comment.comment.published)
                    .unwrap()
                    .cmp(&NaiveDateTime::from_str(&b.comment.comment.published).unwrap())
            });
            for c in ct {
                sort(SortType::New, &mut c.children);
            }
        }
        SortType::Old => {
            ct.sort_by(|a, b| {
                NaiveDateTime::from_str(&a.comment.comment.published)
                    .unwrap()
                    .cmp(&NaiveDateTime::from_str(&b.comment.comment.published).unwrap())
            });
            for c in ct {
                sort(SortType::Old, &mut c.children);
            }
        }
        _ => {}
    }
}
