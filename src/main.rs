mod utils;
mod router;
mod model;
mod controller;
mod service;
mod mapper;
mod entity;

use salvo::prelude::*;

#[tokio::main]
async fn main() {
    // init log
    tracing_subscriber::fmt().init();

    // init db
    mapper::init_db().await;

    // starting
    Server::new(TcpListener::new("127.0.0.1:8080").bind().await).serve(router::init_router()).await;
}
