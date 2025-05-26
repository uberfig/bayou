use actix_cors::Cors;
use actix_web::{rt::spawn, web::Data, App, HttpServer};
use tokio::try_join;

use crate::{config::Config, live_server::server::ChatServer, routes::get_routes};

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

    let (chat_server, server_tx) = ChatServer::new();
    let chat_server = spawn(chat_server.run());

    let bind = config.bind_address.clone();
    let port = config.port;

    let http_server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .max_age(60*60);

        App::new()
            .wrap(cors)
            .app_data(Data::new(conn.clone()))
            .app_data(Data::new(config.to_owned()))
            .app_data(Data::new(server_tx.clone()))
            .service(get_routes())
    })
    .bind((bind, port))?
    .run();
    
    try_join!(http_server, async move { chat_server.await.unwrap() })?;
    Ok(())
}
