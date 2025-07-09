use actix_web::{delete, web, HttpResponse, Responder};
use actix_web::http::uri::PathAndQuery;
use sea_orm::{EntityTrait, DatabaseConnection, ColumnTrait, QueryFilter, ModelTrait};
use crate::entity::{post,user,comm};

#[delete("/posts/{id}")]
pub async fn delete_post(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>
)-> impl Responder{
    let post_id=path.into_inner();
    let post = post::Entity::find_by_id(post_id)
        .one(db.get_ref())
        .await;
    match post {
        Ok(Some(post_model))=>{
            match comm::Entity::delete_many()
                .filter(comm::Column::PostId.eq(post_id))
                .exec(db.get_ref())
                .await
            {
                Ok(_)=>{
                    match post_model.delete(db.get_ref()).await{
                        Ok(_)=>HttpResponse::Ok().body("Post and its comments delete"),
                        Err(err)=>{
                            eprintln!("Error deleting post:{:?}",err);
                            HttpResponse::InternalServerError().body("Failed to delete post")
                        }
                    }
            }
                Err(err)=>{
                    eprintln!("Error deleting comments: {:?}",err);
                    HttpResponse::InternalServerError().body("Failed to delete comments")
                }
            }
        }
        Ok(None)=> HttpResponse::NotFound().body("Post not found"),
        Err(err)=> {
            eprintln!("DB error: {:?}",err);
            HttpResponse::InternalServerError().body("Database error")
        }

    }
}
#[delete("/users/{id}")]
pub async fn user_delete(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>
)-> impl Responder{
    let user_id = path.into_inner();
    let user = user::Entity::find_by_id(user_id)
        .one(db.get_ref())
        .await;
    match user {
        Ok(Some(model))=>{
            match model.delete(db.get_ref()).await {
                Ok(_)=> HttpResponse::Ok().body("User delete"),
                Err(err)=>{
                    eprintln!("DB error while deleting {:?}",err);
                    HttpResponse::InternalServerError().body("Error deleting user")
                }
            }
        }
        Ok(None)=> HttpResponse::NotFound().body("User not found"),
        Err(err)=> {
            eprintln!("DB query error: {:?}",err);
            HttpResponse::InternalServerError().body("Database error")
        }
    }
}