mod api;
mod app;
mod ui;
use app::{InputMode, LApp};
use std::env;
use std::io;
use std::io::Read;
use termion::{async_stdin, event::Key, input::TermRead, raw::IntoRawMode};
use tui::backend::TermionBackend;
use tui::Terminal;
use ui::draw_normal;

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
            .draw(|mut frame| {
                if let InputMode::PostView = app.input_mode {
                    ui::draw_post(&mut app, &mut frame)
                } else {
                    draw_normal(&mut app, &mut frame);
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
                    app.comments = Vec::new();
                    app.input_mode = InputMode::Normal;
                } else if let Key::Char('q') = k.as_ref().unwrap() {
                    terminal.clear()?;
                    return Ok(());
                } else if let Key::Char('c') = k.as_ref().unwrap() {
                    app.comments = api::get_comments(format!(
                        "{}/api/v3/post?id={}",
                        &app.instance,
                        app.posts[app.state.selected().unwrap()].post.id
                    ))
                    .unwrap();
                } else if let Key::Up = k.as_ref().unwrap() {
                    app.c_previous()
                } else if let Key::Down = k.as_ref().unwrap() {
                    app.c_next()
                }
            }
        }
    }
}
