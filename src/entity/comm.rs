use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone,Debug,PartialEq, DeriveEntityModel,Serialize)]
#[sea_orm (table_name="comm")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub id:i32,
    pub text: String,
    pub post_id:i32,
    pub user_id: i32,
}

#[derive(Copy,Clone,Debug,EnumIter, DeriveRelation)]
pub enum Relation{
    #[sea_orm(
        belongs_to = "super::post::Entity",
        from= "Column::PostId",
        to = "super::post::Column::Id",
        on_delete = "Cascade"
    )]
    Post,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_delete = "Cascade"
    )]
    User,
}
impl Related<super::post::Entity> for Entity{
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}
impl Related<super::user::Entity> for Entity{
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
// Вот все эти методы нужно чтобы собирать данные для базы и передавать их в неё спомощью ORM
// belongs_to мы говорим сущнасть А ссылается на В
// Related::User.def() связь из comm к user
// если суфикс rev() то из User к comm
impl ActiveModelBehavior for ActiveModel{}