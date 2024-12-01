use actix_web::{get, App, HttpServer, Responder};
use rand::seq::SliceRandom;
use serde::Serialize;

#[derive(Serialize)]
struct Fact {
    fact: String,
}

#[get("/fact")]
async fn get_fact() -> impl Responder {
    let facts = vec![
        "Rust ensures memory safety without garbage collection.",
        "Rust was created by Graydon Hoare in 2006.",
        "Rust is open-source and managed by Mozilla.",
        "The Rust compiler is called `rustc`.",
        "Rust is designed for concurrency and speed.",
    ];

    let fact = facts.choose(&mut rand::thread_rng()).unwrap();
    actix_web::web::Json(Fact {
        fact: fact.to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_fact))
        .bind("0.0.0.0:8008")?
        .run()
        .await
}
