use url_app::Application;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = Application::build("localhost:3000").await?;
    app.run_until_stopped().await
}