use clap::Parser;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};
use std::{io, panic};

mod sacct;
mod ui;

use sacct::SacctData;

fn setup_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
        original_hook(panic_info);
    }));
}

#[derive(Parser)]
#[command(name = "sacct-tui")]
#[command(about = "A TUI for exploring Slurm sacct output")]
struct Cli {
    #[arg(long, help = "Additional sacct arguments")]
    sacct_args: Option<String>,
}

#[derive(Default)]
struct App {
    jobs: Vec<SacctData>,
    selected: usize,
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        Self::default()
    }

    async fn load_jobs(&mut self, args: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        self.jobs = sacct::fetch_sacct_data(args).await?;
        Ok(())
    }

    fn next(&mut self) {
        if !self.jobs.is_empty() {
            self.selected = (self.selected + 1) % self.jobs.len();
        }
    }

    fn previous(&mut self) {
        if !self.jobs.is_empty() {
            self.selected = if self.selected == 0 {
                self.jobs.len() - 1
            } else {
                self.selected - 1
            };
        }
    }

    fn quit(&mut self) {
        self.should_quit = true;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    setup_panic_hook();
    
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let result = app.load_jobs(cli.sacct_args).await;
    
    let app_result = match result {
        Ok(()) => run_app(&mut terminal, &mut app).await,
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
    };

    // Always restore terminal state
    let _ = disable_raw_mode();
    let _ = execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    );
    let _ = terminal.show_cursor();

    app_result?;
    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => app.quit(),
                KeyCode::Down | KeyCode::Char('j') => app.next(),
                KeyCode::Up | KeyCode::Char('k') => app.previous(),
                _ => {}
            }
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}