mod handler;
mod entity;
mod Struct;
use std::env;
use sea_orm::Database;
use migration::{Migrator,MigratorTrait};
use actix_web::{web, App, HttpServer, Responder};

use handler::create::{create_post,create_user,create_comm};
use handler::read::{get_all_posts,get_id_post};
use crate::handler::read::get_post_with_comments;

async fn hello() -> impl Responder {
    "Hello from Actix + SeaORM!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let db = Database::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();

    Migrator::up(&db, None).await.expect("Failed to apply migration");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(create_post)
            .service(create_user)
            .service(create_comm)
            .service(get_all_posts)
            .service(get_id_post)
            .service(get_post_with_comments)
    })
        .bind(("127.0.0.1",8080))?
        .run()
        .await
}
