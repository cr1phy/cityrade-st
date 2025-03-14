use std::io;

mod client;
mod server;
mod dropguard;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    color_backtrace::install();

    println!("Введите режим (server или client): ");
    let mode = get_user_input();

    if mode.eq_ignore_ascii_case("server") {
        server::serve()?;
    } else if mode.eq_ignore_ascii_case("client") {
        client::run()?;
    } else {
        println!("Неизвестный режим '{}'. Запускаем клиент по умолчанию.", mode);
        client::run()?;
    }

    Ok(())
}

fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Не удалось прочитать строку");
    input.trim().to_string()
}
