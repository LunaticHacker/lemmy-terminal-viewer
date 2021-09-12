use super::app::{InputMode, LApp};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::Text;
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use tui::Frame;
//renders the ui when InputMode is Normal
pub fn draw_normal<B>(app: &mut LApp, frame: &mut Frame<B>)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(7)].as_ref())
        .split(frame.size());
    let input_block = Paragraph::new(tui::text::Text::from(app.input.clone()))
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
            InputMode::PostView => Style::default(),
            InputMode::CommentView => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(input_block, chunks[0]);

    let mut items = vec![];
    for post in &app.posts {
        items.push(ListItem::new(post.post.name.as_ref()))
    }
    let list = List::new(items)
        .block(Block::default().title("Posts").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_symbol("*");

    frame.render_stateful_widget(list, chunks[1], &mut app.state);
}
//renders the ui when InputMode is PostView
pub fn draw_post<B>(app: &mut LApp, frame: &mut Frame<B>)
where
    B: Backend,
{
    let chunks;
    let body = &app.posts[app.state.selected().unwrap_or(0)].post.body;
    let url = &app.posts[app.state.selected().unwrap_or(0)].post.url;
    if let (Some(str), Some(url)) = (body.as_ref(), url.as_ref()) {
        chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(frame.size());

        let lines = Text::styled(url, Style::default());
        let para_ = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::White));
        let lines = Text::styled(str, Style::default());
        let para = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true });
        frame.render_widget(para_, chunks[1]);
        frame.render_widget(para, chunks[0])
    } else if let (None, Some(url)) = (body.as_ref(), url.as_ref()) {
        chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)])
            .split(frame.size());

        let lines = Text::styled(url, Style::default());
        let para = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::White));
        frame.render_widget(para, chunks[0])
    } else if let (Some(str), None) = (body.as_ref(), url.as_ref()) {
        chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)])
            .split(frame.size());
        let lines = Text::styled(str, Style::default());
        let para = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true });
        frame.render_widget(para, chunks[0])
    }
    // let mut items = vec![];

    // for comment in &app.comments {
    //     //Comment can be null :(
    //     if let Some(comment) = comment.comment.comment.as_ref() {
    //             items.push(ListItem::new(comment.content.as_ref()))

    //     }
    // }

    // let list = List::new(items)
    //     .block(Block::default().title("Comments").borders(Borders::ALL))
    //     .style(Style::default().fg(Color::White).bg(Color::Black))
    //     .highlight_symbol(">>");
    // frame.render_stateful_widget(list, chunks[2], &mut app.comment_state);
}

pub fn draw_comment<B>(app: &mut LApp, frame: &mut Frame<B>)
where
    B: Backend,
{
    let mut items = vec![];

    for comment in &app.comments {
        //Comment can be null :(
        if let Some(comment) = comment.comment.comment.as_ref() {
            items.push(ListItem::new(comment.content.as_ref()))
        }
    }
    if let (_, true) = (&app.comments, app.replies.is_empty()) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)])
            .split(frame.size());
        let list = List::new(items)
            .block(Block::default().title("Comments").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_symbol("*");
        frame.render_stateful_widget(list, chunks[0], &mut app.comment_state);
    } else if let (_, false) = (&app.comments, app.replies.is_empty()) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.size());

        if let Some(top_comment) = &app.comments[app.comment_state.selected().unwrap_or(0)]
            .comment
            .comment
            .as_ref()
        {
            let lines = Text::styled(top_comment.content.clone(), Style::default());
            let para = Paragraph::new(lines)
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .wrap(Wrap { trim: true });
            frame.render_widget(para, chunks[0]);
        }

        let mut items = vec![];

        for comment in &app.replies {
            //Comment can be null :(
            if let Some(comment) = comment.comment.comment.as_ref() {
                items.push(ListItem::new(comment.content.as_ref()))
            }
        }

        let list = List::new(items)
            .block(Block::default().title("Comments").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_symbol("*");
        frame.render_widget(list, chunks[1]);
    }
}
