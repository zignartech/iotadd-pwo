#![allow(non_snake_case)]
use actix_web::{App, HttpServer};
mod actix_handler;
mod actix_utils;
mod app_controller;
mod models;
mod streams_utils;
use crate::app_controller::createAuthor;
use crate::app_controller::addressSendOne;
use crate::app_controller::addressFetchAll;
use crate::app_controller::createSubscriber;
use actix_cors::Cors;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv::from_path("./development.env").ok();
  println!(
    "NODE is set to: {:?}",
    std::env::var("NODE").expect("NODE not defined as environment var")
  );
  let server = HttpServer::new(move || {
    App::new()
      .wrap(Cors::permissive())
       .service(createAuthor)
       .service(addressSendOne)
       .service(createSubscriber)
       .service(addressFetchAll)
  })
  .bind("0.0.0.0:3030")?
  .run()
  .await?;
  Ok(server)
}
