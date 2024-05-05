use poem::{listener::TcpListener, Route, Server, Result, EndpointExt};
use poem_openapi::{OpenApi, OpenApiService};
use std::error::Error;

struct Api;
mod book;

#[OpenApi]
impl Api {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let url = "postgres://postgres:bazaDanych@localhost:5432/postgres";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let endpoints = (Api,book::BookApi);
    let api_service =
        OpenApiService::new(endpoints, "Hello World", "1.0").server("http://localhost:3000");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui).data(pool);

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await?;

    Ok(())
}