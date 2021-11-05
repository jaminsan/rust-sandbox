use actix::Actor;
use actix_web::{App, HttpServer, middleware};
use actix_web::web::Data;

mod rooms_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let entrance = domain::actors::entrance::Entrance::default().start();
    let comment_gateway = gateway::comment_gateway::CommentGateway::default().start();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(entrance.clone()))
            .app_data(Data::new(comment_gateway.clone()))
            .wrap(middleware::Logger::default())
            .service(rooms_handler::room_ws_index)
    })
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
