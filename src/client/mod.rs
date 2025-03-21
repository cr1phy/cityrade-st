mod i18n;

use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::CrosstermBackend,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Wrap},
};
use std::{io::stdout, time::Duration};
use tokio::{sync::mpsc, task};

#[derive(Default)]
struct GameState {
    resources: Option<cityrade_types::resources::Resources>,
    buildings: Vec<String>,
    chat_messages: Vec<String>,
    current_tab: usize,
    input: String,
    input_cursor: usize,
    connection_status: ConnectionStatus,
    logs: Vec<String>,
    command_history: Vec<String>,
    command_index: usize,
}

#[derive(Default, PartialEq)]
enum ConnectionStatus {
    #[default]
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

#[derive(PartialEq)]
enum InputMode {
    Normal,
    Editing,
}

enum Message {
    ServerMessage(String),
    GameUpdate {
        resources: Option<cityrade_types::resources::Resources>,
    },
    ConnectionStatus(ConnectionStatus),
}

pub struct App {
    state: GameState,
    input_mode: InputMode,
    tx: mpsc::Sender<Message>,
    rx: mpsc::Receiver<Message>,
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);
        Self {
            state: GameState::default(),
            input_mode: InputMode::Normal,
            tx,
            rx,
            should_quit: false,
        }
    }

    fn handle_input(&mut self, key: KeyEvent) -> Result<()> {
        if key.kind != KeyEventKind::Press {
            return Ok(());
        }
        match self.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('q') => {
                    self.should_quit = true;
                }
                KeyCode::Char('e') => {
                    self.input_mode = InputMode::Editing;
                }
                KeyCode::Tab => {
                    self.state.current_tab = (self.state.current_tab + 1) % 4;
                }
                KeyCode::BackTab => {
                    self.state.current_tab = (self.state.current_tab + 3) % 4;
                }
                _ => {}
            },
            InputMode::Editing => match key.code {
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                }
                KeyCode::Enter => {
                    let command = self.state.input.clone();
                    if !command.trim().is_empty() {
                        self.log(&format!("Executing: {}", command));
                        self.state.command_history.push(command.clone());
                        self.state.command_index = self.state.command_history.len();
                        self.handle_command(&command)?;
                    }
                    self.state.input = String::new();
                    self.state.input_cursor = 0;
                }
                KeyCode::Char(c) => {
                    self.state.input.insert(self.state.input_cursor, c);
                    self.state.input_cursor += 1;
                }
                KeyCode::Backspace => {
                    if self.state.input_cursor > 0 {
                        self.state.input_cursor -= 1;
                        self.state.input.remove(self.state.input_cursor);
                    }
                }
                KeyCode::Left => {
                    if self.state.input_cursor > 0 {
                        self.state.input_cursor -= 1;
                    }
                }
                KeyCode::Right => {
                    if self.state.input_cursor < self.state.input.len() {
                        self.state.input_cursor += 1;
                    }
                }
                _ => {}
            },
        }
        Ok(())
    }

    fn handle_command(&mut self, command: &str) -> Result<()> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }
        match parts[0] {
            "connect" => {
                if parts.len() < 2 {
                    self.log("Usage: connect <server>");
                    return Ok(());
                }
                let server = parts[1];
                self.connect(server)?;
            }
            "login" => {
                if parts.len() < 3 {
                    self.log("Usage: login <username> <password>");
                    return Ok(());
                }
                let username = parts[1];
                let password = parts[2];
                self.login(username, password)?;
            }
            "build" => {
                if parts.len() < 4 {
                    self.log("Usage: build <name> <type> <x> <y>");
                    return Ok(());
                }
                self.log("Building functionality not yet implemented");
            }
            "chat" => {
                if parts.len() < 2 {
                    self.log("Usage: chat <message>");
                    return Ok(());
                }
                let message = parts[1..].join(" ");
                self.send_chat_message(&message)?;
            }
            "quit" | "exit" => {
                self.should_quit = true;
            }
            "help" => {
                self.log("Available commands:");
                self.log("  connect <server>                - Connect to server");
                self.log("  login <username> <password>     - Login to server");
                self.log("  build <name> <type> <x> <y>     - Build a building");
                self.log("  chat <message>                  - Send chat message");
                self.log("  help                            - Show this help");
                self.log("  quit, exit                      - Exit the game");
                self.log("");
                self.log("Keyboard shortcuts:");
                self.log("  Tab/Shift+Tab - Switch between tabs");
                self.log("  'e' - Enter edit mode");
                self.log("  'q' - Quit (in normal mode)");
                self.log("  Esc - Exit edit mode");
                self.log("  Up/Down - Navigate command history");
            }
            _ => {
                self.log(&format!("Unknown command: {}", parts[0]));
            }
        }
        Ok(())
    }

    fn log(&mut self, message: &str) {
        self.state.logs.push(format!(
            "[{}] {}",
            chrono::Local::now().format("%H:%M:%S"),
            message
        ));
    }

    fn connect(&mut self, server: &str) -> Result<()> {
        self.log(&format!("Connecting to {}...", server));
        self.state.connection_status = ConnectionStatus::Connecting;
        let tx = self.tx.clone();
        let server_addr = server.to_string();
        task::spawn(async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let _ = tx
                .send(Message::ConnectionStatus(ConnectionStatus::Connected))
                .await;
            let _ = tx
                .send(Message::ServerMessage(format!(
                    "Connected to {}",
                    server_addr
                )))
                .await;
        });
        Ok(())
    }

    fn login(&mut self, username: &str, password: &str) -> Result<()> {
        match self.state.connection_status {
            ConnectionStatus::Connected => {
                self.log(&format!("Logging in as {}...", username));
                let tx = self.tx.clone();
                let username = username.to_string();
                task::spawn(async move {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    let _ = tx
                        .send(Message::ServerMessage(format!(
                            "Login successful as {}!",
                            username
                        )))
                        .await;
                    let resources = Some(cityrade_types::resources::Resources::new());
                    let _ = tx.send(Message::GameUpdate { resources }).await;
                });
            }
            _ => {
                self.log("Not connected to server. Use 'connect <server>' first.");
            }
        }
        Ok(())
    }

    fn send_chat_message(&mut self, message: &str) -> Result<()> {
        match self.state.connection_status {
            ConnectionStatus::Connected => {
                self.state.chat_messages.push(format!("You: {}", message));
                let tx = self.tx.clone();
                let message = message.to_string();
                task::spawn(async move {
                    tokio::time::sleep(Duration::from_millis(500)).await;
                    let _ = tx
                        .send(Message::ServerMessage(format!(
                            "Server: Got your message: {}",
                            message
                        )))
                        .await;
                });
            }
            _ => {
                self.log("Not connected to server. Use 'connect <server>' first.");
            }
        }
        Ok(())
    }

    fn process_messages(&mut self) -> Result<()> {
        while let Ok(message) = self.rx.try_recv() {
            match message {
                Message::ServerMessage(msg) => {
                    if self.state.current_tab == 2 {
                        self.state.chat_messages.push(msg.clone());
                    }
                    self.log(&msg);
                }
                Message::GameUpdate { resources } => {
                    self.state.resources = resources;
                    self.log("Received updated resource data");
                }
                Message::ConnectionStatus(status) => {
                    self.state.connection_status = status;
                }
            }
        }
        Ok(())
    }

    fn draw(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(f.area());
        let titles = ["Resources", "Buildings", "Chat", "Logs"]
            .iter()
            .map(|t| {
                let selected = self.state.current_tab
                    == match *t {
                        "Resources" => 0,
                        "Buildings" => 1,
                        "Chat" => 2,
                        "Logs" => 3,
                        _ => 0,
                    };
                if selected {
                    Span::styled(
                        format!("[{}]", t),
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    Span::styled(format!(" {} ", t), Style::default().fg(Color::White))
                }
            })
            .collect::<Vec<_>>();
        let tabs = Tabs::new(titles)
            .block(
                Block::default()
                    .title("Cityrade Client")
                    .borders(Borders::ALL),
            )
            .highlight_style(Style::default().fg(Color::Yellow))
            .select(self.state.current_tab);
        f.render_widget(tabs, chunks[0]);
        match self.state.current_tab {
            0 => self.draw_resources(f, chunks[1]),
            1 => self.draw_buildings(f, chunks[1]),
            2 => self.draw_chat(f, chunks[1]),
            3 => self.draw_logs(f, chunks[1]),
            _ => {}
        }
        let status = match &self.state.connection_status {
            ConnectionStatus::Disconnected => "Disconnected",
            ConnectionStatus::Connecting => "Connecting...",
            ConnectionStatus::Connected => "Connected",
            ConnectionStatus::Error(e) => e,
        };
        let status_style = match &self.state.connection_status {
            ConnectionStatus::Disconnected => Style::default().fg(Color::Red),
            ConnectionStatus::Connecting => Style::default().fg(Color::Yellow),
            ConnectionStatus::Connected => Style::default().fg(Color::Green),
            ConnectionStatus::Error(_) => Style::default().fg(Color::Red),
        };
        let mode_text = if let InputMode::Editing = self.input_mode {
            "[EDIT]"
        } else {
            "[NORMAL]"
        };
        let status_text = format!("{} | Status: {} | ", mode_text, status);
        let cursor_indicator = if let InputMode::Editing = self.input_mode {
            "_"
        } else {
            ""
        };
        let input_display = if self.input_mode == InputMode::Editing {
            format!("{}{}", self.state.input, cursor_indicator)
        } else {
            format!("Press 'e' to enter edit mode, 'q' to quit")
        };
        let input = Paragraph::new(format!("{}{}", status_text, input_display))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(input, chunks[2]);
    }

    fn draw_resources(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .title("Resources")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));
        let inner = block.inner(area);
        f.render_widget(block, area);
        if let Some(resources) = &self.state.resources {
            let resources_text = Text::from(vec![
                Line::from(Span::styled(
                    "City Resources",
                    Style::default().add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(vec![
                    Span::raw("Gold: "),
                    Span::styled("100", Style::default().fg(Color::Yellow)),
                ]),
                Line::from(vec![
                    Span::raw("Wood: "),
                    Span::styled("200", Style::default().fg(Color::Green)),
                ]),
                Line::from(vec![
                    Span::raw("Stone: "),
                    Span::styled("300", Style::default().fg(Color::Gray)),
                ]),
                Line::from(""),
                Line::from(Span::styled(
                    "Production per minute:",
                    Style::default().add_modifier(Modifier::BOLD),
                )),
                Line::from(vec![
                    Span::raw("Gold: "),
                    Span::styled("+5", Style::default().fg(Color::Yellow)),
                ]),
                Line::from(vec![
                    Span::raw("Wood: "),
                    Span::styled("+10", Style::default().fg(Color::Green)),
                ]),
                Line::from(vec![
                    Span::raw("Stone: "),
                    Span::styled("+8", Style::default().fg(Color::Gray)),
                ]),
            ]);
            let resource_para = Paragraph::new(resources_text).wrap(Wrap { trim: true });
            f.render_widget(resource_para, inner);
        } else {
            let text =
                Paragraph::new("No resource data available.\nConnect to a server and login first.")
                    .style(Style::default().fg(Color::Gray));
            f.render_widget(text, inner);
        }
    }

    fn draw_buildings(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .title("Buildings")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));
        let inner = block.inner(area);
        f.render_widget(block, area);
        if self.state.buildings.is_empty() {
            let text = if self.state.connection_status == ConnectionStatus::Connected {
                "No buildings. Use 'build' command to create one."
            } else {
                "Not connected. Connect to server first."
            };
            let para = Paragraph::new(text).style(Style::default().fg(Color::Gray));
            f.render_widget(para, inner);
        } else {
            let items: Vec<ListItem> = self
                .state
                .buildings
                .iter()
                .map(|b| ListItem::new(b.clone()))
                .collect();
            let list = List::new(items).highlight_style(Style::default().fg(Color::Yellow));
            f.render_widget(list, inner);
        }
    }

    fn draw_chat(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .title("Chat")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));
        let inner = block.inner(area);
        f.render_widget(block, area);
        if self.state.chat_messages.is_empty() {
            let text = if self.state.connection_status == ConnectionStatus::Connected {
                "No messages. Use 'chat <message>' to send one."
            } else {
                "Not connected. Connect to server first."
            };
            let para = Paragraph::new(text).style(Style::default().fg(Color::Gray));
            f.render_widget(para, inner);
        } else {
            let items: Vec<ListItem> = self
                .state
                .chat_messages
                .iter()
                .map(|m| {
                    let (sender, content) = if let Some(idx) = m.find(':') {
                        let (sender, content) = m.split_at(idx + 1);
                        (sender, content)
                    } else {
                        (m.as_str(), "")
                    };
                    let style = if sender.starts_with("You:") {
                        Style::default().fg(Color::Green)
                    } else if sender.starts_with("Server:") {
                        Style::default().fg(Color::Blue)
                    } else {
                        Style::default()
                    };
                    ListItem::new(Line::from(vec![
                        Span::styled(sender, style.add_modifier(Modifier::BOLD)),
                        Span::raw(content),
                    ]))
                })
                .collect();
            let list = List::new(items);
            f.render_widget(list, inner);
        }
    }

    fn draw_logs(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .title("Logs")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));
        let inner = block.inner(area);
        f.render_widget(block, area);
        if self.state.logs.is_empty() {
            let text = Paragraph::new("No logs");
            f.render_widget(text, inner);
        } else {
            let logs = self
                .state
                .logs
                .iter()
                .map(|l| ListItem::new(l.clone()))
                .collect::<Vec<_>>();
            let list = List::new(logs).style(Style::default().fg(Color::Gray));
            f.render_widget(list, inner);
        }
    }
}

pub async fn run() -> Result<()> {
    enable_raw_mode().context("Failed to enable raw mode")?;
    execute!(stdout(), EnterAlternateScreen).context("Failed to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend).context("Failed to create terminal")?;
    terminal.clear()?;
    let mut app = App::new();
    app.log("Welcome to Cityrade!");
    app.log("Type 'help' for a list of commands");
    loop {
        terminal.draw(|f| app.draw(f))?;
        app.process_messages()?;
        if app.should_quit {
            break;
        }
        if event::poll(Duration::from_millis(100)).context("Event poll failed")? {
            if let Event::Key(key) = event::read().context("Event read failed")? {
                app.handle_input(key)?;
            }
        }
    }
    disable_raw_mode().context("Failed to disable raw mode")?;
    execute!(stdout(), LeaveAlternateScreen).context("Failed to leave alternate screen")?;
    Ok(())
}
