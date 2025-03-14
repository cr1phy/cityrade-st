use color_eyre::Result;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

type SharedLogs = Arc<Mutex<Vec<String>>>;

#[derive(Serialize, Deserialize, Debug)]
struct UserCredentials {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerResponse {
    message: String,
    success: bool,
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    addr: SocketAddr,
    logs: SharedLogs,
) -> Result<()> {
    let ws_stream = accept_async(stream).await?;
    {
        let mut log = logs.lock().unwrap();
        log.push(format!("Новое соединение от {}", addr));
    }
    let (mut sender, mut receiver) = ws_stream.split();

    // Ожидаем первое сообщение – данные для авторизации
    if let Some(msg) = receiver.next().await {
        let msg = msg?;
        if msg.is_text() {
            let text = msg.into_text()?;
            let credentials: UserCredentials = serde_json::from_str(&text)?;
            {
                let mut log = logs.lock().unwrap();
                log.push(format!(
                    "Получены учётные данные от {}: {:?}",
                    addr, credentials
                ));
            }
            let auth_success =
                credentials.username == "admin" && credentials.password == "password";
            let response = if auth_success {
                ServerResponse {
                    message: "Успешный вход".into(),
                    success: true,
                }
            } else {
                ServerResponse {
                    message: "Неверные учётные данные".into(),
                    success: false,
                }
            };
            let response_json = serde_json::to_string(&response)?;
            sender
                .send(tokio_tungstenite::tungstenite::Message::Text(
                    response_json.into(),
                ))
                .await?;
            if !auth_success {
                {
                    let mut log = logs.lock().unwrap();
                    log.push(format!("Аутентификация не пройдена для {}", addr));
                }
                return Ok(());
            }
        }
    }

    // Обработка последующих сообщений (например, эхо)
    while let Some(msg) = receiver.next().await {
        let msg = msg?;
        if msg.is_text() {
            let text = msg.into_text()?;
            {
                let mut log = logs.lock().unwrap();
                log.push(format!("Получено от {}: {}", addr, text.trim()));
            }
            let reply = format!("Эхо: {}", text.trim());
            sender
                .send(tokio_tungstenite::tungstenite::Message::Text(reply.into()))
                .await?;
        } else if msg.is_close() {
            {
                let mut log = logs.lock().unwrap();
                log.push(format!("Соединение с {} закрыто", addr));
            }
            break;
        }
    }
    Ok(())
}

#[tokio::main]
pub async fn serve() -> Result<()> {
    // Shared лог для TUI
    let logs: SharedLogs = Arc::new(Mutex::new(vec![]));
    // Атомарный флаг завершения
    let running = Arc::new(AtomicBool::new(true));
    let running_server = running.clone();
    let logs_clone = Arc::clone(&logs);

    // Запускаем асинхронный сервер в отдельном потоке
    let server_thread = thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let addr = "127.0.0.1:8080";
            let listener = TcpListener::bind(addr).await.unwrap();
            {
                let mut log = logs_clone.lock().unwrap();
                log.push(format!("Сервер запущен на {}", addr));
            }
            while running_server.load(Ordering::SeqCst) {
                tokio::select! {
                    conn = listener.accept() => {
                        if let Ok((stream, addr)) = conn {
                            let logs_inner = Arc::clone(&logs_clone);
                            tokio::spawn(async move {
                                if let Err(e) = handle_connection(stream, addr, logs_inner).await {
                                    eprintln!("Ошибка при обработке соединения {}: {:?}", addr, e);
                                }
                            });
                        }
                    }
                    // Короткая задержка, чтобы периодически проверять флаг
                    _ = tokio::time::sleep(Duration::from_millis(100)) => {},
                }
            }
            // Завершаем прослушивание
            {
                let mut log = logs_clone.lock().unwrap();
                log.push("Сервер остановлен.".into());
            }
        });
    });

    // Запускаем TUI для отображения логов сервера
    use crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    };
    use ratatui::{
        Terminal,
        backend::CrosstermBackend,
        layout::{Constraint, Direction, Layout},
        widgets::{Block, Borders, List, ListItem},
    };

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let size = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);
            let log_text = {
                let logs = logs.lock().unwrap();
                logs.clone()
            };
            let items: Vec<ListItem> = log_text.iter().map(|l| ListItem::new(l.clone())).collect();
            let list = List::new(items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Логи сервера (нажмите q для выхода)"),
            );
            f.render_widget(list, chunks[0]);
        })?;

        if crossterm::event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    running.store(false, Ordering::SeqCst);
                    break;
                }
            }
        }
    }

    disable_raw_mode().expect("Не удалось выключить raw режим");
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .expect("Не удалось выйти из альтернативного экрана");
    terminal
        .show_cursor()
        .expect("Не удалось отобразить курсор");

    server_thread.join().unwrap();
    Ok(())
}
