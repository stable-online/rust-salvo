mod utils;
mod router;
mod model;
mod controller;
mod service;

use salvo::prelude::*;

#[handler]
async fn test() -> &'static str {
    "Hello World"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    Server::new(TcpListener::new("127.0.0.1:8080").bind().await).serve(router::init_router()).await;
}


