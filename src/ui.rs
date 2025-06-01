use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::App;

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    draw_header(f, chunks[0]);
    draw_job_list(f, chunks[1], app);
    draw_footer(f, chunks[2]);

    if !app.jobs.is_empty() {
        draw_job_details(f, app);
    }
}

fn draw_header(f: &mut Frame, area: Rect) {
    let header = Paragraph::new("Slurm sacct TUI")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title("Header"));
    f.render_widget(header, area);
}

fn draw_job_list(f: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = app
        .jobs
        .iter()
        .enumerate()
        .map(|(i, job)| {
            let style = if i == app.selected {
                Style::default().bg(Color::DarkGray).fg(Color::White)
            } else {
                Style::default()
            };
            ListItem::new(job.display_line()).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Jobs ({}/{})", app.selected + 1, app.jobs.len())),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(list, area);
}

fn draw_footer(f: &mut Frame, area: Rect) {
    let footer = Paragraph::new("q: quit | ↑/k: up | ↓/j: down | Enter: details")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title("Help"));
    f.render_widget(footer, area);
}

fn draw_job_details(f: &mut Frame, app: &App) {
    if let Some(job) = app.jobs.get(app.selected) {
        let area = centered_rect(80, 80, f.area());
        f.render_widget(Clear, area);

        let details = format!(
            "Job ID: {}\n\
             Job Name: {}\n\
             Partition: {}\n\
             Account: {}\n\
             Allocated CPUs: {}\n\
             State: {}\n\
             Exit Code: {}\n\
             Start Time: {}\n\
             End Time: {}\n\
             Elapsed: {}\n\
             Time Limit: {}\n\
             Submit Time: {}\n\
             User: {}\n\
             Work Directory: {}",
            job.job_id,
            job.job_name,
            job.partition,
            job.account,
            job.alloc_cpus,
            job.state,
            job.exit_code,
            job.start,
            job.end,
            job.elapsed,
            job.time_limit,
            job.submit,
            job.user,
            job.work_dir
        );

        let popup = Paragraph::new(details)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Job Details")
                    .style(Style::default().fg(Color::Yellow)),
            )
            .style(Style::default().fg(Color::White));

        f.render_widget(popup, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}