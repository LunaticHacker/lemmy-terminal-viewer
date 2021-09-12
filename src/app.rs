use super::api::{CommentTree, PostInfo};
use tui::widgets::ListState;
//Enum for Different Modes
pub enum InputMode {
    Normal,
    Editing,
    PostView,
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
    //instance url
    pub instance: String,
}

impl Default for LApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            posts: Vec::new(),
            comments: Vec::new(),
            state: ListState::default(),
            comment_state: ListState::default(),
            instance: String::from("https://lemmy.ml"),
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
    }
    pub fn c_previous(&mut self) {
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
    }

    // Unselect the currently selected item if any. The implementation of `ListState` makes
    // sure that the stored offset is also reset.
     pub fn c_unselect(&mut self) {
         self.comment_state.select(None);
     }
}
