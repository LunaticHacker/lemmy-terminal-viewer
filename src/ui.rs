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
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(20),
            Constraint::Percentage(70),
        ])
        .split(frame.size());
    let body = &app.posts[app.state.selected().unwrap_or(0)]
        .post
        .body;
    let url = &app.posts[app.state.selected().unwrap_or(0)].post.url;
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
    let mut items = vec![];

    for comment in &app.comments {
        //Comment can be null :(
        if let Some(comment) = comment.comment.as_ref() {
            if let Some(content) = &comment.content {
                if let None = &comment.parent_id {
                    items.push(ListItem::new(content.as_ref()))
                }
            }
        }
    }

    let list = List::new(items)
        .block(Block::default().title("Comments").borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .highlight_symbol(">>");
    frame.render_stateful_widget(list, chunks[2], &mut app.comment_state);
}
