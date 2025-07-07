use std::fs::exists;
use actix_web::{post, web, HttpResponse, Responder};
use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, QueryFilter, ColumnTrait, DbErr};
use crate::entity::{comm, post, user};
use crate::Struct::structur::{CreateCommDTO, CreatePostDto, CreateUserDTO};

#[post("/posts")]
pub async fn create_post(
    db: web::Data<DatabaseConnection>,
    data: web::Json<CreatePostDto>
) -> impl Responder{
    let existing = post::Entity::find()
        .filter(post::Column::Title.eq(&data.title))
        .filter(post::Column::UserId.eq(data.user_id))
        .one(db.get_ref())
        .await;
    match existing {
        Ok(Some(_))=>{
            HttpResponse::Conflict().body("Post with same title already exists for this user")
        }
        Ok(None)=>{
            let new_post = post::ActiveModel{
                title: Set(data.title.clone()),
                text: Set(data.text.clone()),
                user_id: Set(data.user_id),
                ..Default::default ()
            };
            match new_post.insert(db.get_ref()).await {
                Ok(saves_post)=> HttpResponse::Ok().json(saves_post),
                Err(err)=>{
                    eprintln!("DB error: {:?}",err);
                    HttpResponse::InternalServerError().body("Error saving post")
                }
            }
        }
        Err(err)=>{
            eprintln!("DB query error : {:?}", err);
            HttpResponse::InternalServerError().body("Error Checking existing post")
        }
    }
}




#[post("/create_user")]
pub async fn create_user(
    db: web::Data<DatabaseConnection>,
    data: web::Json<CreateUserDTO>
)-> impl Responder{
    let existing_user = user::Entity::find()
        .filter(user::Column::Name.eq(&data.name))
        .filter(user::Column::Surname.eq(&data.surname))
        .one(db.get_ref())
        .await;
    match existing_user {
        Ok(Some(_)) => {
            HttpResponse::Conflict().body("User already exists")
        }
        Ok(None) => {
            let new_user = user::ActiveModel {
                name: Set(data.name.clone()),
                surname: Set(data.surname.clone()),
                ..Default::default()
            };
            match new_user.insert(db.get_ref()).await {
                Ok(saved_user) => HttpResponse::Ok().json(saved_user),
                Err(err) => {
                    eprintln!("DB error: {:?}", err);
                    HttpResponse::InternalServerError().body("Error create user")
                }
            }
        }
        Err(err) => {
            eprintln!("Query error: {:?}", err);
            HttpResponse::InternalServerError().body("Error checking user")
        }
    }
}
#[post("/create_comm")]
pub async fn create_comm(
    db: web::Data<DatabaseConnection>,
    data: web::Json<CreateCommDTO>
)-> impl Responder{
    let post_exists = post::Entity::find_by_id(data.post_id)
        .one(db.get_ref())
        .await;
    match post_exists {
        Ok(Some(_))=> {
            let existing_comm = comm::Entity::find()
                .filter(comm::Column::Text.eq(&data.text))
                .filter(comm::Column::UserId.eq(data.user_id))
                .filter(comm::Column::PostId.eq(data.post_id))
                .one(db.get_ref())
                .await;
            match existing_comm {
                Ok(Some(_))=>{
                    HttpResponse::Conflict().body("This comment already exists")
                }
                Ok(None)=> {
                    let new_comm= comm::ActiveModel{
                        text:Set(data.text.clone()),
                        user_id:Set(data.user_id),
                        post_id:Set(data.post_id),
                        ..Default::default()
                    };
                    match new_comm.insert(db.get_ref()).await {
                        Ok(saved)=>HttpResponse::Ok().json(saved),
                        Err(e)=>{
                            eprintln!("DB insert error {:?}",e);
                            HttpResponse::InternalServerError().body("Failed to save comment")
                        }
                    }
                }
                Err(e)=>{
                    eprintln!("Comment query error: {:?}",e);
                    HttpResponse::InternalServerError().body("Failed to check comment")
                }
            }
        }
        Ok(None)=>{
            HttpResponse::NotFound().body("Post not found")
        }
        Err(e)=>{
            eprintln!("Post check error: {:?}",e);
            HttpResponse::InternalServerError().body("Failed to check post")
        }
    }


}
