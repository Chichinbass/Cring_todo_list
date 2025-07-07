use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, TODO: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        TODO
            .create_table(
                Table::create()
                    .table(User::Table).if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(string(User::Name).not_null())
                    .col(string(User::Surname).not_null())
                    .to_owned(),
            )
            .await?;
        TODO
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(pk_auto(Post::Id))
                    .col(string(Post::Title))
                    .col(string(Post::Text))
                    .col(integer(Post::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Post::Table, Post::UserId)
                            .to(User::Table, User::Id)
                    )
                    .to_owned(),
            ).await?;
        TODO
            .create_table(
                Table::create()
                    .table(Comm::Table)
                    .if_not_exists()
                    .col(pk_auto(Comm::Id))
                    .col(string(Comm::Text))
                    .col(integer(Comm::PostId))
                    .col(integer(Comm::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Comm::Table, Comm::PostId)
                            .to(Post::Table, Post::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Comm::Table,Comm::UserId)
                            .to(User::Table, User::Id)
                    )
                    .to_owned(),
            ).await

    }

    async fn down(&self, TODO: &SchemaManager) -> Result<(), DbErr> {
        TODO.drop_table(Table::drop().table(Comm::Table).to_owned()).await?;
        TODO.drop_table(Table::drop().table(Post::Table).to_owned()).await?;
        TODO.drop_table(Table::drop().table(User::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
    UserId,
}

#[derive(DeriveIden)]
enum Comm {
    Table,
    Id,
    Text,
    PostId,
    UserId,
}
#[derive(DeriveIden)]
enum User{
    Table,
    Id,
    Name,
    Surname,
}

