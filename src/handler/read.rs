use actix_web::{get, web, HttpResponse, Responder};
use sea_orm::{EntityTrait, DatabaseConnection, QueryFilter, ColumnTrait};
use crate::entity::{comm, post};

// метод find().all() выводит все данные из выбранной таблицы
#[get("/posts")]
pub async fn get_all_posts(
    db: web::Data<DatabaseConnection>
)-> impl Responder{
    match post::Entity::find().all(db.get_ref()).await {
        Ok(posts)=> HttpResponse::Ok().json(posts),
        Err(err)=>{
            eprintln!("DB error: {:?}", err);
            HttpResponse::InternalServerError().body("Error fetching posts")
        }
    }
}
// здесь уже есть метод find_by_id() он помогает нам определить по id
#[get("/posts/{id}")]
pub async fn get_id_post(db: web::Data<DatabaseConnection>,
                         path:web::Path<i32>) -> impl  Responder{
    let post_id= path.into_inner();
    match post::Entity::find_by_id(post_id)
        .one(db.get_ref())
        .await
    {
        Ok(Some(post))=> HttpResponse::Ok().json(post),
        Ok(None)=> HttpResponse::NotFound().body("Post not found"),
        Err(err)=>{
            eprintln!("Db error: {:?}",err);
            HttpResponse::InternalServerError().body("Error fetching post")
        }
    }
}

#[get("/posts/{id}/with_comment")]
pub async fn get_post_with_comments(
    db:web::Data<DatabaseConnection>,
    path:web::Path<i32>
)-> impl Responder{
    let post_id=path.into_inner();
    let post_item= post::Entity::find_by_id(post_id)
        .one(db.get_ref()) //вытаскивает одну запись из бд
        .await;
    let Some(post_model)=post_item.ok().flatten() else {
        return HttpResponse::NotFound().body("Post not found")
    };
    let comments = comm::Entity::find()
        .filter(comm::Column::PostId.eq(post_id))
        .all(db.get_ref())
        .await
        .unwrap_or_default();
    let comment_dtos = comments
        .into_iter()
        .map(|c| crate::Struct::structur::CommentDTO {
            id: c.id,
            text: c.text,
            user_id: c.user_id,
            post_id: c.post_id,
        })
        .collect::<Vec<_>>();
    let result = crate::Struct::structur::PostWithComments{
        id: post_model.id,
        title: post_model.title,
        text: post_model.text,
        user_id: post_model.user_id,
        comments: comment_dtos,
    };
    HttpResponse::Ok().json(result)
}
