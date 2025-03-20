mod client;
mod server;

use clap::{Parser, arg};

#[derive(Debug, Parser)]
#[command(name = "cityrade", version, about, long_about)]
struct Args {
    #[arg(short, long)]
    serve: bool,
    #[arg(short, long)]
    web: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mode = match (args.serve, args.web) {
        (true, false) => server::serve().await,
        // (false, true) => client::web().await,
        (false, false) => client::run().await,
        _ => anyhow::bail!("Serving in web? Really?"),
    };

    if let Err(e) = mode {
        eprintln!("An error occurred: {}", e);
        std::process::exit(1);
    };

    Ok(())
}
