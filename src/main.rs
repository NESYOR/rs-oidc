use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let private_key = include_bytes!("../private_key.pem").to_vec();
    let config = Config::new();
    let server = Server::new(private_key,config);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone()))
            .route("/jwks.json", web::get().to(Server::jwks))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
