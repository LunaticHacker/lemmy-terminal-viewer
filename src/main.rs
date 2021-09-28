mod api;
mod app;
mod auth;
mod config;
mod event;
mod ui;
mod utils;
use app::{InputMode, LApp};
use directories::ProjectDirs;
use event::{Event, Events};
use std::env;
use std::fs;
use std::io;
use termion::{event::Key, raw::IntoRawMode};
use tui::backend::TermionBackend;
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    let mut app = LApp::default();
    let args: Vec<String> = env::args().collect();
    let mut conf: config::Config = config::Config::default();

    if let Some(proj_dirs) = ProjectDirs::from("dev", "ltv", "ltv") {
        let config_dir = proj_dirs.config_dir();

        let config_file = fs::read_to_string(config_dir.join("ltv.toml"));

        conf = match config_file {
            Ok(file) => toml::from_str(&file).unwrap(),
            Err(_) => config::Config::default(),
        };
        app.instance = utils::prepend_https(conf.default_instance.clone());
        app.theme = utils::colorify(conf.theme.clone());
    }

    match args.len() {
        2 => {
            if &args[1] == "login" {
                match auth::login() {
                    Ok(tuple) => {
                        app.instance = tuple.0;
                        app.auth = tuple.1;
                    }
                    Err(e) => return Err(e),
                };
            } else {
                app.instance = utils::prepend_https(args[1].clone());
            }
        }
        3 => {
            app.instance = utils::prepend_https(args[1].clone());
            if let Some(proj_dirs) = ProjectDirs::from("dev", "ltv", "ltv") {
                let config: auth::AuthConfig = toml::from_str(
                    &fs::read_to_string(&proj_dirs.config_dir().join("ltv.toml"))
                        .unwrap_or_default(),
                )
                .unwrap_or_default();
                app.auth = config.instancelist.instances[&app.instance].userlist[&args[2]].clone();
            }
        }
        _ => {}
    }
    app.posts = api::get_posts(
        format!("{}/api/v3/post/list?", &app.instance),
        &app.auth,
        &conf.clone().stringify(),
    )
    .unwrap_or_default();
    // Set up terminal output
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    //init event handler
    let events = Events::new();

    terminal.clear()?;
    'outer: loop {
        // Lock the terminal and start a drawing session.
        terminal.autoresize()?;
        terminal
            .draw(|mut frame| {
                if let InputMode::PostView = app.input_mode {
                    ui::draw_post(&mut app, &mut frame)
                } else if let InputMode::CommentView = app.input_mode {
                    ui::draw_comment(&mut app, &mut frame)
                } else {
                    ui::draw_normal(&mut app, &mut frame);
                }
            })
            .unwrap();
        //Event handling, TODO: Refactor this abomination
        if let Event::Input(input) = events.next().unwrap() {
            if let InputMode::Normal = &app.input_mode {
                if let Key::Char('q') = input {
                    break 'outer;
                } else if let Key::Char('i') = input {
                    app.unselect();
                    app.input_mode = InputMode::Editing;
                } else if let Key::Up = input {
                    app.previous()
                } else if let Key::Down = input {
                    app.next()
                } else if let Key::Right = input {
                    if !app.posts.is_empty() {
                        app.input_mode = InputMode::PostView
                    }
                } else if let Key::Left = input {
                    app.posts = api::get_posts(
                        format!("{}/api/v3/post/list?", &app.instance),
                        &app.auth,
                        &conf.clone().stringify(),
                    )
                    .unwrap_or_default();
                }
            } else if let InputMode::Editing = &app.input_mode {
                if let Key::Left = input {
                    app.input_mode = InputMode::Normal;
                } else if let Key::Right = input {
                    app.posts = api::get_posts(
                        format!(
                            "{}/api/v3/post/list?community_name={}",
                            &app.instance, app.input
                        ),
                        &app.auth,
                        &conf.clone().stringify(),
                    )
                    .unwrap_or_default();
                    app.input_mode = InputMode::Normal;
                } else if let termion::event::Key::Char(c) = input {
                    app.input.push(c);
                } else if let Key::Backspace = input {
                    app.input.pop();
                }
            } else if let InputMode::PostView = &app.input_mode {
                if let Key::Left = input {
                    app.comments = Vec::new();
                    app.input_mode = InputMode::Normal;
                } else if let Key::Char('q') = input {
                    break 'outer;
                } else if let Key::Down = input {
                    app.c_unselect();
                    let comments = api::get_comments(
                        format!(
                            "{}/api/v3/post?id={}&",
                            &app.instance,
                            app.posts[app.state.selected().unwrap_or_default()].post.id
                        ),
                        &app.auth,
                    )
                    .unwrap_or_default();
                    app.comments = comments;
                    app.input_mode = InputMode::CommentView;
                }
            } else if let InputMode::CommentView = &app.input_mode {
                if let Key::Up = input {
                    if !app.comments.is_empty() {
                        app.c_previous()
                    }
                } else if let Key::Down = input {
                    if !app.comments.is_empty() {
                        app.c_next()
                    }
                } else if let Key::Left = input {
                    if app.cursor.len() == 1 {
                        app.r_unselect();
                        app.replies = Vec::new();
                        app.cursor.pop();
                    } else if app.cursor.len() > 1 {
                        app.cursor.pop();
                        app.r_unselect();
                        app.replies = utils::get_comments(app.cursor.clone(), app.comments.clone());
                    } else {
                        app.r_unselect();
                        app.replies = Vec::new();
                        app.input_mode = InputMode::PostView;
                    }
                } else if let Key::Right = input {
                    if !app.comments.is_empty() {
                        if app.cursor.is_empty()
                            && !app.comments[app.comment_state.selected().unwrap_or_default()]
                                .children
                                .is_empty()
                        {
                            app.replies = app.comments[app.comment_state.selected().unwrap_or(0)]
                                .children
                                .clone();
                            app.cursor
                                .push(app.comment_state.selected().unwrap_or_default());
                        } else {
                            if !app.replies.is_empty() {
                                if !app.replies[app.replies_state.selected().unwrap_or_default()]
                                    .children
                                    .is_empty()
                                {
                                    app.cursor
                                        .push(app.replies_state.selected().unwrap_or_default());
                                    app.replies = utils::get_comments(
                                        app.cursor.clone(),
                                        app.comments.clone(),
                                    );
                                    app.r_unselect();
                                }
                            }
                        }
                    }
                } else if let Key::Char('q') = input {
                    break 'outer;
                }
            }
        }
    }
    terminal.clear()
}
