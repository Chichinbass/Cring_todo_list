use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone,Debug,PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm (table_name="user")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub id:i32,
    pub name: String,
    pub surname: String,
}

#[derive(Copy,Clone,Debug,EnumIter, DeriveRelation)]
pub enum Relation{}
impl Related<super::post::Entity> for Entity{
    fn to()-> RelationDef{
        super::post::Relation::User.def().rev()
    }
}
impl Related<super::comm::Entity> for Entity{
    fn to()-> RelationDef{
        super::comm::Relation::User.def().rev()
    }
}
impl ActiveModelBehavior for ActiveModel{}