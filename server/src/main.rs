use actix_web::{get, web, middleware::Logger,App, HttpResponse, HttpServer, Responder,Result};
use std::sync::Mutex;
use serde::Deserialize;
use log::info;
use env_logger::Env;


struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

#[derive(Deserialize)]
struct Info {
    user_id: u32,
    friend: String,
}

#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn index(info: web::Path<Info>) -> Result<String> {
    info!("{}", "error test");
    Ok(format!(
        "Welcome {}, user_id {}!",
        info.friend, info.user_id
    ))
}


#[get("/")]
async fn hello(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1; // <- access counter inside MutexGuard

    HttpResponse::Ok().body(format!("Request number: {counter}"))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    info!("{}", "error test");
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
        .wrap(Logger::default())
            .app_data(counter.clone())
            .service(hello)
            .service(index)
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}