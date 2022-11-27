use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use actix_files::NamedFile;
use log::error;

async fn stop_horny(req: HttpRequest) -> impl Responder {
    const ALLOWED_METHODS: [&str; 2] = ["GET", "HEAD"];
    let method = req.method().as_str();
    if !ALLOWED_METHODS.contains(&method) {
        return HttpResponse::MethodNotAllowed().insert_header(("Allow", "GET, HEAD")).finish()
    }

    let r = NamedFile::open("stop_horny.jpg");

    match r {
        Ok(n) => n.into_response(&req),
        Err(e) => {
            error!("{e}");
            HttpResponse::InternalServerError()
                                .insert_header(("Content-Type", "text/plain"))
                                .body("an unknown error occured\n")
        },
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .route("/", web::to(stop_horny))
            .route("/{a:.+}", web::to(stop_horny))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
