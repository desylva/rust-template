use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "welcome.html")]
struct WelcomeTemplate<'a> {
    title: &'a str,
    body: &'a str,
}

#[get("/")]
async fn hello() -> impl Responder {
    let template = WelcomeTemplate {
        title: "Welcome",
        body: "To The Bookstore!",
    };
    HttpResponse::Ok().body(template.body)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 6001))?
    .run()
    .await
}