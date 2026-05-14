//! System Extension Manager - Main Entry Point
//!
//! A TUI application for managing Login Items, Launch Agents, Launch Daemons,
//! and System Extensions on macOS.

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::Terminal;
use std::io;
use system_extension_manager::{TuiApp, AppError};
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Log to a file so output doesn't corrupt the TUI display
    let log_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/extman.log")
        .ok();
    if let Some(file) = log_file {
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().with_writer(std::sync::Mutex::new(file)))
            .with(tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("system_extension_manager=info".parse()?))
            .init();
    }

    info!("Starting System Extension Manager");

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create and initialize the app
    let mut app = TuiApp::new();
    app.init();

    // Main event loop
    let res = run_app(&mut terminal, &mut app);

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    if let Err(e) = res {
        error!("Application error: {}", e);
        eprintln!("Error: {}", e);
    }

    info!("System Extension Manager exited");
    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut TuiApp,
) -> Result<(), AppError> {
    loop {
        // Draw the UI
        terminal.draw(|f| {
            app.render(f);
        })?;

        // Poll for events
        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Ignore key releases
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                let key_str = match key.code {
                    KeyCode::Char(c) => {
                        if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                            match c {
                                'c' | 'C' => "ctrl-c".to_string(),
                                'd' | 'D' => "ctrl-d".to_string(),
                                _ => continue,
                            }
                        } else {
                            c.to_string()
                        }
                    }
                    KeyCode::Up => "up".to_string(),
                    KeyCode::Down => "down".to_string(),
                    KeyCode::Left => "left".to_string(),
                    KeyCode::Right => "right".to_string(),
                    KeyCode::Enter => "enter".to_string(),
                    KeyCode::Esc => "escape".to_string(),
                    KeyCode::Char(' ') => "space".to_string(),
                    KeyCode::Backspace => "backspace".to_string(),
                    _ => continue,
                };

                if !app.handle_key(&key_str) {
                    break;
                }
            }
        }
    }

    Ok(())
}