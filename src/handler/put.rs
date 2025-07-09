use actix_web::{put,web,HttpResponse,Responder};
use sea_orm::{EntityTrait, Set, ActiveModelTrait, ColumnTrait, QueryFilter, DatabaseConnection};
use crate::entity::post;
use crate::Struct::structur::UpdatePostDTO;

#[put("/posts/{id}")]
pub async fn update_post(
    db:web::Data<DatabaseConnection>,
    path:web::Path<i32>,
    data:web::Json<UpdatePostDTO>
)-> impl Responder{
    let post_id=path.into_inner();
    let post= post::Entity::find_by_id(post_id)
        .one(db.get_ref())
        .await;
    match post {
        Ok(Some(post_model))=>{
            let mut post_active: post::ActiveModel = post_model.into();
            if let Some(title)= &data.title{
                post_active.title=Set(title.clone());
            }
            if let Some(text)= &data.text{
                post_active.text=Set(text.clone());
            }
            match post_active.update(db.get_ref()).await {
                Ok(update_post)=> HttpResponse::Ok().json(update_post),
                Err(err)=>{
                    eprintln!("Error updating post: {:?}",err);
                    HttpResponse::InternalServerError().body("Failed to update post")
                }
            }
        }
        Ok(None)=>HttpResponse::NotFound().body("Post not found"),
        Err(err)=>{
            eprintln!("DB error: {:?}",err);
            HttpResponse::InternalServerError().body("Database error")
        }
    }
}