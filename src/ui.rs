use super::app::{InputMode, LApp};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text, WrappedText};
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
        let mut t = WrappedText::new(frame.size().width - 1);
        t.extend(Text::from(vec![
            Spans::from(vec![Span::from(post.creator.name.as_ref())]),
            Spans::from(post.post.name.as_ref()),
        ]));
        items.push(ListItem::new(t))
    }
    let list = List::new(items)
        .block(Block::default().title("Posts").borders(Borders::ALL))
        .highlight_symbol(tui::symbols::line::VERTICAL)
        .repeat_highlight_symbol(true);

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
        let para_ = Paragraph::new(lines).block(Block::default().borders(Borders::ALL));
        let lines = Text::styled(str, Style::default());
        let para = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        frame.render_widget(para_, chunks[0]);
        frame.render_widget(para, chunks[1])
    } else if let (None, Some(url)) = (body.as_ref(), url.as_ref()) {
        chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)])
            .split(frame.size());

        let lines = Text::styled(url, Style::default());
        let para = Paragraph::new(lines).block(Block::default().borders(Borders::ALL));
        frame.render_widget(para, chunks[0])
    } else if let (Some(str), None) = (body.as_ref(), url.as_ref()) {
        chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)])
            .split(frame.size());
        let lines = Text::styled(str, Style::default());
        let para = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        frame.render_widget(para, chunks[0])
    } else {
        chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)])
            .split(frame.size());
        let lines = Text::styled(
            &app.posts[app.state.selected().unwrap_or(0)].post.name,
            Style::default(),
        );
        let para = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        frame.render_widget(para, chunks[0])
    }
}

pub fn draw_comment<B>(app: &mut LApp, frame: &mut Frame<B>)
where
    B: Backend,
{
    let mut items = vec![];
    for comment in &app.comments {
        //Comment can be null :(
        if let Some(c) = comment.comment.comment.as_ref() {
            let mut t = WrappedText::new(frame.size().width - 1);
            t.extend(Text::from(vec![
                Spans::from(vec![Span::styled(
                    &comment.comment.creator.name,
                    Style::default().add_modifier(Modifier::UNDERLINED),
                )]),
                Spans::from(c.content.as_ref()),
            ]));
            items.push(ListItem::new(t))
        }
    }
    if let (_, true) = (&app.comments, app.replies.is_empty()) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)])
            .split(frame.size());
        let list = List::new(items)
            .block(Block::default().title("Comments").borders(Borders::ALL))
            .highlight_symbol(tui::symbols::line::VERTICAL)
            .repeat_highlight_symbol(true);
        frame.render_stateful_widget(list, chunks[0], &mut app.comment_state);
    } else if let (_, false) = (&app.comments, app.replies.is_empty()) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)])
            .split(frame.size());

        let mut items = vec![];

        for comment in &app.replies {
            //Comment can be null :(
            if let Some(c) = comment.comment.comment.as_ref() {
                let mut t = WrappedText::new(frame.size().width - 1);
                t.extend(Text::from(vec![
                    Spans::from(vec![Span::styled(
                        &comment.comment.creator.name,
                        Style::default().add_modifier(Modifier::UNDERLINED),
                    )]),
                    Spans::from(c.content.as_ref()),
                ]));
                items.push(ListItem::new(t))
            }
        }

        let list = List::new(items)
            .block(Block::default().title("Replies").borders(Borders::ALL))
            .highlight_symbol(tui::symbols::line::VERTICAL)
            .repeat_highlight_symbol(true);
        frame.render_stateful_widget(list, chunks[0], &mut app.replies_state);
    }
}
