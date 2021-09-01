use clap::Clap;
use std::sync::Arc;

mod client;
mod database;
mod message;
mod networking;
mod request;
mod response;
mod server;

/// Args is a struct that contains the command line arguments.
/// The purpose of this struct is to make it easy to add new
/// command line arguments using Clap.
#[derive(Debug, Clap)]
#[clap(name = "Rust Chatter")]
pub struct Args {
    #[clap(short = 's', long = "server")]
    is_server: bool,

    #[clap(short, long, required = false, default_value = "127.0.0.1")]
    address: String,

    #[clap(short, long, required = false, default_value = "23432")]
    port: String,

    #[clap(short, long, required = false, default_value = "unknown")]
    username: String,
}

/// The main function is the entry point for the program.
#[tokio::main]
async fn main() {
    let args: Arc<Args> = Arc::new(Args::parse());
    println!("{:?}", args);

    // We only run a server or client. Client by default.
    if args.is_server {
        server::setup_server(args).await
    } else {
        client::client(args).await
    }
}
