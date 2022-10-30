use crate::users::{User, Users};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use actix_web::HttpRequest;

use chrono::Utc;
use chrono::DateTime;

#[get("/users")]
async fn find_all(req: HttpRequest) -> Result<HttpResponse, CustomError> {
    let peer = req.peer_addr();
    let requ = req.headers();
    let readi: DateTime<Utc> = Utc::now();
    println!("[{:?} INFO ] - squirrel tActix - /users POST function request - {:?} - {:?}", readi, peer, requ);
    let users = Users::find_all()?;
    Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{id}")]
async fn find(req: HttpRequest, id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let peer = req.peer_addr();
    let requ = req.headers();
    let readi: DateTime<Utc> = Utc::now();
    println!("[{:?} INFO ] - squirrel tActix - /users/{id} GET function request - {:?} - {:?}", readi, peer, requ);


    let user = Users::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/users")]
async fn create(req: HttpRequest, user: web::Json<User>) -> Result<HttpResponse, CustomError> {
    let peer = req.peer_addr();
    let requ = req.headers();
    let readi: DateTime<Utc> = Utc::now();
    println!("[{:?} INFO ] - squirrel tActix - /users POST function request - {:?} - {:?}", readi, peer, requ);

    let user = Users::create(user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[put("/users/{id}")]
async fn update(
    id: web::Path<i32>,
    user: web::Json<User>,
) -> Result<HttpResponse, CustomError> {
    let user = Users::update(id.into_inner(), user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{id}")]
async fn delete(req: HttpRequest, id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let peer = req.peer_addr();
    let requ = req.headers();
    let readi: DateTime<Utc> = Utc::now();
    println!("[{:?} INFO ] - squirrel tActix - /users/{id} DELETE function request - {:?} - {:?}", readi, peer, requ);

    let deleted_user = Users::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_user })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}
