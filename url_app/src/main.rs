use url_app::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init(); // handles error into a log file     
    run().await

}
