use serde::Deserialize;
use std::collections::HashMap;
use std::io;
use std::io::Read;
use termion::{async_stdin, event::Key, input::TermRead, raw::IntoRawMode};
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use tui::Terminal;
enum InputMode {
    Normal,
    Editing,
    PostView,
}
#[derive(Deserialize, Debug)]
pub struct Post {
    id: i32,
    name: String,
    url: Option<String>,
    body: Option<String>,
    creator_id: i32,
    community_id: i32,
    removed: bool,
    locked: bool,
    published: Option<String>,
    updated: Option<String>,
    deleted: bool,
    nsfw: bool,
    stickied: bool,
    embed_title: Option<String>,
    embed_description: Option<String>,
    embed_html: Option<String>,
    thumbnail_url: Option<String>,
    ap_id: Option<String>,
    local: bool,
}
#[derive(Deserialize, Debug)]
struct Posts {
    post: Post,
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
struct Obj {
    posts: Vec<Posts>,
}
/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    //List of Posts
    posts: Vec<Posts>,
    //State for indexing the list
    state: ListState,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            posts: Vec::new(),
            state: ListState::default(),
        }
    }
}

impl App {
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

fn main() -> Result<(), reqwest::Error> {
    // Set up terminal output
    let stdout = io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::default();

    // Create a separate thread to poll stdin.
    // This provides non-blocking input support.
    let mut asi = async_stdin();

    terminal.clear().unwrap();
    loop {
        // Lock the terminal and start a drawing session.
        terminal
            .draw(|frame| {
                if let InputMode::PostView = app.input_mode {
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [Constraint::Percentage(100)],
                        )
                        .split(frame.size());
                    let body = &app
                        .posts
                        .get(app.state.selected().unwrap())
                        .unwrap()
                        .post
                        .body;
                    let text = vec![Spans::from(body.as_ref().unwrap().clone())];
                    let para =Paragraph::new(text)
                        .block(Block::default().title("Paragraph").borders(Borders::ALL))
                        .style(Style::default().fg(Color::White).bg(Color::Black));
                    frame.render_widget(para,chunks[0])
                } else {
                    // Create a layout into which to place our blocks.
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [Constraint::Percentage(15), Constraint::Percentage(85)].as_ref(),
                        )
                        .split(frame.size());

                    // Create a block...
                    let input_block = Paragraph::new(tui::text::Text::from(app.input.clone()))
                        .style(match app.input_mode {
                            InputMode::Normal => Style::default(),
                            InputMode::Editing => Style::default().fg(Color::Yellow),
                            InputMode::PostView => Style::default(),
                        })
                        .block(Block::default().borders(Borders::ALL));

                    // Render into the first chunk of the layout.
                    frame.render_widget(input_block, chunks[0]);

                    // The text lines for our text box.
                    let mut items = vec![];
                    for post in &app.posts {
                        items.push(ListItem::new(post.post.name.as_ref()))
                    }
                    let list = List::new(items)
                        .block(Block::default().title("List").borders(Borders::ALL))
                        .style(Style::default().fg(Color::White))
                        .highlight_symbol("*");

                    // Render into the second chunk of the layout.
                    frame.render_stateful_widget(list, chunks[1], &mut app.state);
                }
            })
            .unwrap();

        for k in asi.by_ref().keys() {
            if let InputMode::Normal = &app.input_mode {
                if let Key::Char('q') = k.as_ref().unwrap() {
                    terminal.clear().unwrap();
                    return Ok(());
                } else if let Key::Char('i') = k.as_ref().unwrap() {
                    app.unselect();
                    app.input_mode = InputMode::Editing;
                } else if let Key::Up = k.as_ref().unwrap() {
                    app.previous()
                } else if let Key::Down = k.as_ref().unwrap() {
                    app.next()
                } else if let Key::Char('\n') = k.as_ref().unwrap() {
                    app.input_mode = InputMode::PostView
                }
            } else if let InputMode::Editing = &app.input_mode {
                if let Key::Esc = k.as_ref().unwrap() {
                    app.input_mode = InputMode::Normal;
                } else if let Key::Char('\n') = k.as_ref().unwrap() {
                    let response = reqwest::blocking::get("https://lemmy.ml/api/v3/post/list")?;
                    app.posts = response.json::<Obj>()?.posts;
                } else if let termion::event::Key::Char(c) = k.as_ref().unwrap() {
                    app.input.push(*c);
                } else if let Key::Backspace = k.as_ref().unwrap() {
                    app.input.pop();
                }
            } else if let InputMode::PostView = &app.input_mode {
                if let Key::Esc = k.as_ref().unwrap() {
                    app.input_mode = InputMode::Normal;
                }
            }
        }
    }
}
