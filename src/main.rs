use bayou::{app::start_application, config::get_config};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_BACKTRACE", "full");

    let config = match get_config() {
        Ok(x) => x,
        Err(x) => {
            eprintln!("{:#?}", x);
            return Ok(());
        }
    };

    println!("instance domain: {}", &config.instance_domain);

    println!(
        "starting server at http://{}:{}",
        &config.bind_address, &config.port
    );

    start_application(config).await
}
