use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize)]
// struct Item {
//     id: u32,
//     name: String,
// }

#[derive(Serialize, Deserialize)]
struct Todo {
    id: u32,
    name: String,
    is_complete: bool,
}

#[derive(Deserialize)]
struct QueryParams {
    id: u32,
}

#[get("/")]
async fn get_todo(query: web::Query<QueryParams>) -> impl Responder {
    let id = query.id;
    HttpResponse::Ok().json(id)
}

#[post("/api/post_todo")]
async fn post_todo() -> impl Responder {
    let todo = Todo {
        id: 3023,
        name: "Homework".to_string(),
        is_complete: false,
    };
    HttpResponse::Ok().json(todo)
}

#[delete("/api/delete_todo")]
async fn delete_todo() -> impl Responder {
    "Delete Method"
}

#[put("/api/update_todo")]
async fn update_todo() -> impl Responder {
    "Put Method"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_todo)
            .service(post_todo)
            .service(delete_todo)
            .service(update_todo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
