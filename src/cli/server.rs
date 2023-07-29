use crate::server;
use std::env::{self, Args};

fn usage() {
    println!(
        "
Usage: {} server
",
        env::args().next().unwrap()
    );
}

pub async fn server(mut args: Args) {
    if args.any(|arg| arg == "--help" || arg == "-h") {
        return usage();
    };
    server::run().await;
}
