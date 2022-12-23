use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::{env, str::FromStr};
use wol::{send_wol, MacAddr};

#[derive(Deserialize)]
struct Info {
    mac: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <port>", args[0]);
        std::process::exit(1);
    }

    let port = args[1].parse::<u16>().unwrap_or_else(|_| {
        println!("Port must be a number");
        std::process::exit(1);
    });

    HttpServer::new(|| App::new().service(ping).service(wake_on_lan))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[post("/wake")]
async fn wake_on_lan(info: web::Query<Info>) -> impl Responder {
    if let Ok(_) = send_wol(MacAddr::from_str(&info.mac).unwrap(), None, None) {
        HttpResponse::Ok().body("waking up")
    } else {
        HttpResponse::InternalServerError().body("error")
    }
}
