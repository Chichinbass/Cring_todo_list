use dotenvy::dotenv;
use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    dotenv().ok(); // загрузит .env из корня, если ты запускаешь из корня проекта
    cli::run_cli(migration::Migrator).await;
}
