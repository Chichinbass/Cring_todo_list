use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct CreatePostDto{
    pub title: String,
    pub text: String,
    pub user_id: i32,
}
#[derive(Deserialize)]
pub  struct CreateUserDTO{
    pub name: String,
    pub surname: String,
}

#[derive(Deserialize)]
pub  struct CreateCommDTO{
    pub text: String,
    pub post_id: i32,
    pub user_id: i32,
}

#[derive(Serialize)]
pub struct CommentDTO {
    pub id: i32,
    pub text: String,
    pub user_id: i32,
    pub post_id: i32,
}

#[derive(Serialize)]
pub struct PostWithComments {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub user_id: i32,
    pub comments: Vec<CommentDTO>,
}