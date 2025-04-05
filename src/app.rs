use actix_web::{web::Data, App, HttpServer};

use crate::{config::Config, routes::get_routes};

pub async fn start_application(config: Config) -> std::io::Result<()> {
    //init the conn and instance actor
    let conn = config.create_conn();
    if let Err(x) = conn.init().await {
        eprintln!("{}", x);
        return Ok(());
    }
    let _ = conn
        .get_or_init_main_instance(&config.instance_domain)
        .await;
    // conn.get_instance_actor(config.signing_algo).await;

    let bind = config.bind_address.clone();
    let port = config.port;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(conn.clone()))
            .app_data(Data::new(config.to_owned()))
            .service(get_routes())
    })
    .bind((bind, port))?
    .run()
    .await
}
