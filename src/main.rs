use human_panic::{setup_panic, Metadata};

mod client;
mod server;

#[tokio::main]
async fn main() {
    setup_panic!(
        Metadata::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
            .authors("cr1phy <cr1phy@mail.ru>")
            .support("- Open a support request by email to cr1phy@mail.ru")
    );
}
