use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "post")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub title: String,
    pub text: String,
    pub user_id: i32,
}
#[derive(Copy,Clone,Debug,EnumIter, DeriveRelation)]
pub enum Relation{
#[sea_orm(
    belongs_to= "super::user::Entity"
    from = "Column::UserId",
    to = "super::user::Column::Id",
    on_update = "Cascade",
    on_delete = "Cascade"
)]
User,
}
impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
impl Related<super::comm::Entity> for Entity {
    fn to() -> RelationDef {
        super::comm::Relation::Post.def().rev()
    }
}
impl ActiveModelBehavior for ActiveModel{}