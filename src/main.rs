use clap::Parser;

use std::sync::Arc;

mod client;
mod database;
mod message;
mod network;
mod request;
mod response;
mod server;

/// Args is a struct that contains the command line arguments.
#[derive(Debug, Parser)]
#[command(name = "Chatter")]
pub struct Args {
    #[arg(short = 's', long = "server")]
    is_server: bool,

    #[arg(short, long, required = false, default_value = "127.0.0.1")]
    address: String,

    #[arg(short, long, required = false, default_value = "23432")]
    port: String,

    #[arg(short = 'u', long, required = false, default_value = "unknown")]
    username: String,
}

/// The main function is the entry point for the program.

fn main() {
    let args: Arc<Args> = Arc::new(Args::parse());
    println!("{:?}", args);

    // spawn(async { eframe::run_native(Box::new(app), native_options) });

    // We only run a server or client. Client by default.
    if args.is_server {
        server::setup_server(args);
    } else {
        client::client(args);
    }
}
