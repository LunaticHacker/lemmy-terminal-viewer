use super::api::{CommentTree, PostInfo};
use std::collections::HashMap;
use tui::widgets::ListState;
//Enum for Different Modes
pub enum InputMode {
    Normal,
    Editing,
    PostView,
    CommentView,
}
/// App holds the state of the application
pub struct LApp {
    /// Current value of the input box
    pub input: String,
    /// Current input mode
    pub input_mode: InputMode,
    //List of Posts
    pub posts: Vec<PostInfo>,
    //State for indexing the list
    pub state: ListState,
    //List of Comments
    pub comments: Vec<CommentTree>,
    //State for indexing comments
    pub comment_state: ListState,
    //State for indexing replies
    pub replies_state: ListState,
    //List of replies
    pub replies: Vec<CommentTree>,
    //instance url
    pub instance: String,
    //cursor to navigate nested comments
    pub cursor: Vec<usize>,
    //jwt key
    pub auth: String,
    //theme
    pub theme: HashMap<String, tui::style::Color>,
}

impl Default for LApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            posts: Vec::new(),
            comments: Vec::new(),
            replies: Vec::new(),
            state: ListState::default(),
            comment_state: ListState::default(),
            replies_state: ListState::default(),
            instance: String::from("https://lemmy.ml"),
            cursor: Vec::new(),
            auth: String::from(""),
            theme: HashMap::new(),
        }
    }
}

impl LApp {
    // Select the next item. This will not be reflected until the widget is drawn in the
    // `Terminal::draw` callback using `Frame::render_stateful_widget`.
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.posts.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.posts.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    // Unselect the currently selected item if any. The implementation of `ListState` makes
    // sure that the stored offset is also reset.
    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
impl LApp {
    // TODO: Refactor this into one function.
    pub fn c_next(&mut self) {
        if self.replies.is_empty() {
            let i = match self.comment_state.selected() {
                Some(i) => {
                    if i >= self.comments.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.comment_state.select(Some(i));
        } else {
            let i = match self.replies_state.selected() {
                Some(i) => {
                    if i >= self.replies.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.replies_state.select(Some(i));
        }
    }
    pub fn c_previous(&mut self) {
        if self.replies.is_empty() {
            let i = match self.comment_state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.comments.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.comment_state.select(Some(i));
        } else {
            let i = match self.replies_state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.replies.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.replies_state.select(Some(i));
        }
    }

    // Unselect the currently selected item if any. The implementation of `ListState` makes
    // sure that the stored offset is also reset.
    pub fn c_unselect(&mut self) {
        self.comment_state.select(None);
    }
    pub fn r_unselect(&mut self) {
        self.replies_state.select(None);
    }
}
