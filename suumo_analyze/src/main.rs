use actix_web::{get, App, HttpResponse, HttpServer, Responder};

// #[get("/{id}/{name}/index.html")]
// async fn index(params: web::Path<(u32, String)>) -> impl Responder {
//     let (id, name) = params.into_inner();
//     format!("Hello {}! id:{}", name, id)
// }

#[get("/hc")]
async fn hc() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hc))
        .bind(("127.0.0.1", 9000))?
        .run()
        .await
}