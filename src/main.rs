mod api;
mod app;
use app::{InputMode, LApp};
use std::env;
use std::io;
use std::io::Read;
use termion::{async_stdin, event::Key, input::TermRead, raw::IntoRawMode};
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::Text;
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    //Collecting the instance from cl agrs defaults to lemmy.ml if not provided
    let mut app = LApp::default();
    let mut args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        if !(&args[1][0..7] == "https://") {
            args[1].insert_str(0, "https://")
        }
        app.instance = args[1].clone();
    }

    // Set up terminal output
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create a separate thread to poll stdin.
    // This provides non-blocking input support.
    let mut asi = async_stdin();

    terminal.clear()?;
    loop {
        // Lock the terminal and start a drawing session.
        terminal
            .draw(|frame| {
                if let InputMode::PostView = app.input_mode {
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Percentage(15), Constraint::Percentage(15)])
                        .split(frame.size());
                    let body = &app
                        .posts
                        .get(app.state.selected().unwrap())
                        .unwrap()
                        .post
                        .body;
                    let url = &app.posts[app.state.selected().unwrap()].post.url;
                    if let Some(str) = body.as_ref() {
                        let lines = Text::styled(str, Style::default());
                        let para = Paragraph::new(lines)
                            .block(Block::default().borders(Borders::ALL))
                            .style(Style::default().fg(Color::White).bg(Color::Black))
                            .wrap(Wrap { trim: true });
                        frame.render_widget(para, chunks[1])
                    }
                    if let Some(url) = url.as_ref() {
                        let lines = Text::styled(url, Style::default());
                        let para = Paragraph::new(lines)
                            .block(Block::default().borders(Borders::ALL))
                            .style(Style::default().fg(Color::White).bg(Color::Black));
                        frame.render_widget(para, chunks[0])
                    }
                } else {
                    // Create a layout into which to place our blocks.
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Length(3), Constraint::Length(7)].as_ref())
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
                        .block(Block::default().title("Posts").borders(Borders::ALL))
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
                    terminal.clear()?;
                    return Ok(());
                } else if let Key::Char('i') = k.as_ref().unwrap() {
                    app.unselect();
                    app.input_mode = InputMode::Editing;
                } else if let Key::Up = k.as_ref().unwrap() {
                    app.previous()
                } else if let Key::Down = k.as_ref().unwrap() {
                    app.next()
                } else if let Key::Char('\n') = k.as_ref().unwrap() {
                    if let None = app.state.selected() {
                        app.state.select(Some(0))
                    }
                    app.input_mode = InputMode::PostView
                }
            } else if let InputMode::Editing = &app.input_mode {
                if let Key::Esc = k.as_ref().unwrap() {
                    app.input_mode = InputMode::Normal;
                } else if let Key::Char('\n') = k.as_ref().unwrap() {
                    app.posts = api::get_posts(format!(
                        "{}/api/v3/post/list?community_name={}",
                        &app.instance, app.input
                    ))
                    .unwrap_or_default();
                } else if let termion::event::Key::Char(c) = k.as_ref().unwrap() {
                    app.input.push(*c);
                } else if let Key::Backspace = k.as_ref().unwrap() {
                    app.input.pop();
                }
            } else if let InputMode::PostView = &app.input_mode {
                if let Key::Esc = k.as_ref().unwrap() {
                    app.input_mode = InputMode::Normal;
                } else if let Key::Char('q') = k.as_ref().unwrap() {
                    terminal.clear()?;
                    return Ok(());
                }
            }
        }
    }
}
