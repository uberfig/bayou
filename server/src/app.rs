use std::sync::Mutex;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};

use crate::{
    api::{ap_api::inbox::Inbox, routes::get_routes},
    config::Config,
    db::conn::Conn,
};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn start_application(config: Config) -> std::io::Result<()> {
    //init the conn and instance actor
    let conn = config.create_conn();
    if let Err(x) = conn.init(&config.instance_domain).await {
        eprintln!("{}", x);
        return Ok(());
    }
    conn.get_instance_actor(config.signing_algo).await;

    let bind = config.bind_address.clone();
    let port = config.port;
    let inbox = Data::new(Inbox {
        inbox: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(conn.clone()))
            .app_data(inbox.clone())
            .app_data(Data::new(config.to_owned()))
            .service(get_routes())
    })
    .bind((bind, port))?
    .run()
    .await
}
