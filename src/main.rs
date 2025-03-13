use std::io;

mod client;
mod server;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    color_backtrace::install();

    let mode = get_user_input();
    if mode == "server" {
        return server::serve();
    }
    client::main();

    Ok(())
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
