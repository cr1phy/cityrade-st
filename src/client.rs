use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message};
use serde::{Serialize, Deserialize};
use std::io::{self, Write};
use std::net::SocketAddr;

use crate::get_user_input;

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

#[tokio::main]
pub async fn main() {
    // Устанавливаем соединение с сервером по WebSocket
    let server_address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let (mut socket, _response) = connect(format!("ws://{}", server_address)).expect("Failed to connect");

    // Пример регистрации пользователя
    println!("Введите имя пользователя:");
    let mut username = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();

    println!("Введите пароль:");
    let mut password = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).unwrap();

    let user_credentials = UserCredentials {
        username: username.trim().to_string(),
        password: password.trim().to_string(),
    };

    // Отправка данных на сервер для авторизации/регистрации
    let serialized_data = serde_json::to_string(&user_credentials).unwrap();
    socket.send(Message::Text(serialized_data.into())).expect("Failed to send data");

    // Получаем ответ от сервера
    let response = socket.read().expect("Failed to read response");
    let response_data: ServerResponse = serde_json::from_str(&response.to_string()).unwrap();

    if response_data.success {
        println!("Успешный вход: {}", response_data.message);
    } else {
        println!("Ошибка: {}", response_data.message);
    }

    loop {
        let input = get_user_input();
        match input.as_str() {
            "exit" => break,
            _ => send_user_action(&mut socket, input),
        }
    }
}

fn send_user_action(socket: &mut tungstenite::protocol::WebSocket<MaybeTlsStream<std::net::TcpStream>>, action: String) {
    let message = format!("Action: {}", action);
    socket.send(Message::Text(message.into())).expect("Failed to send action");
}
