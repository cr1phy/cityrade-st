use std::time::Duration;
use crossterm::{event::{self, Event, KeyCode}, terminal::disable_raw_mode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use color_eyre::Result;

// Импортируем нашу структуру для автоматической очистки терминала
use crate::dropguard::TerminalCleanup;

#[tokio::main]
pub async fn run() -> Result<()> {
    // Создаём guard для терминала
    let mut term_guard = TerminalCleanup::new().unwrap();
    let terminal = term_guard.terminal();

    // Заранее заданный список серверов
    let servers = vec![
        "127.0.0.1:8080".to_string(),
        "127.0.0.1:8081".to_string(),
        "192.168.1.10:8080".to_string(),
    ];
    let mut selected = 0;
    let mut input = String::new(); // для будущих команд

    loop {
        terminal.draw(|f| {
            let size = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),  // Заголовок
                        Constraint::Min(5),     // Список серверов
                        Constraint::Length(3),  // Панель ввода
                    ]
                    .as_ref(),
                )
                .split(size);

            let header = Paragraph::new("Выберите сервер (↑/↓, Enter для подключения, q для выхода)")
                .block(Block::default().borders(Borders::ALL).title("Клиент"));
            f.render_widget(header, chunks[0]);

            let items: Vec<ListItem> = servers
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    let content = if i == selected {
                        format!("> {}", s)
                    } else {
                        format!("  {}", s)
                    };
                    ListItem::new(content)
                })
                .collect();
            let server_list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Сервера"));
            f.render_widget(server_list, chunks[1]);

            let input_paragraph = Paragraph::new(input.clone())
                .block(Block::default().borders(Borders::ALL).title("Команда"));
            f.render_widget(input_paragraph, chunks[2]);
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        // Выход из TUI
                        break;
                    }
                    KeyCode::Down => {
                        if selected < servers.len() - 1 {
                            selected += 1;
                        }
                    }
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        }
                    }
                    KeyCode::Enter => {
                        let chosen_server = &servers[selected];
                        // Выходим из TUI и переходим к подключению
                        disable_raw_mode()?;
                        println!("Подключение к серверу: {}", chosen_server);
                        // Здесь можно добавить вызов функции подключения
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
    }

    // Если цикл завершился (нажато 'q'), завершаем программу
    // Благодаря Drop для term_guard, терминал будет восстановлен корректно.
    std::process::exit(0);
}
