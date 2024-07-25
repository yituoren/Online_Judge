use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::globals::USER_LIST;
use crate::api::error::HttpError;
use crate::sql::{insert_user, update_user};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User
{
    pub id: usize,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostUser
{
    pub id: Option<usize>,
    pub name: String,
}

#[post("/users")]
pub async fn post_users(post_user: web::Json<PostUser>) -> HttpResponse
{
    let mut lock = USER_LIST.lock().await;
    let max = lock.len();
    if lock.iter().any(|x| x.name == post_user.name)
    {
        return HttpResponse::BadRequest()
            .content_type("application/json")
            .json(HttpError {
                code: 1,
                reason: "ERR_INVALID_ARGUMENT".to_string(),
                message: "User name '".to_string() + &post_user.name + "' already exists."
            });
    }
    //update user
    if let Some(id) = post_user.id
    {
        if id >= max
        {
            return HttpResponse::NotFound()
                .content_type("application/json")
                .json(HttpError {
                    code: 3,
                    reason: "ERR_NOT_FOUND".to_string(),
                    message: "User ".to_string() + &id.to_string() + " not found.",
                });
        }
        else
        {
            lock[id].name = post_user.name.clone();

            if let Err(_) = update_user(&lock[id]).await
            {
                return HttpResponse::InternalServerError()
                        .content_type("application/json")
                        .json(HttpError {
                            code: 5,
                            reason: "ERR_EXTERNAL".to_string(),
                            message: "SQL error".to_string(),
                        })
            }
            return HttpResponse::Ok()
                .content_type("application/json")
                .json(lock[id].clone());
        }
    }
    //new user
    let user = User { id: max, name: post_user.name.clone(), };
    lock.push(user.clone());
    if let Err(_) = insert_user(&user).await
    {
        return HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(HttpError {
                    code: 5,
                    reason: "ERR_EXTERNAL".to_string(),
                    message: "SQL error".to_string(),
                })
    }
    HttpResponse::Ok()
        .content_type("application/json")
        .json(lock[max].clone())
}

#[get("/users")]
pub async fn get_users() -> HttpResponse
{
    HttpResponse::Ok()
        .content_type("application/json")
        .json(USER_LIST.lock().await.clone())
}