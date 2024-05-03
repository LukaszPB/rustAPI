use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};
use sqlx::query;
use sqlx::Connection;
use sqlx::Row;
use std::error::Error;

struct Api;
#[derive(Debug)]
struct Book {
    title: String,
    author: String
}
async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO book (title, author) VALUES ($1, $2)";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .execute(pool)
        .await?;

    Ok(())
}
async fn read(pool: &sqlx::PgPool) -> Result<Book, Box<dyn Error>> {
    let q = "SELECT title, author FROM book";
    let query = sqlx::query(q);

    let row = query.fetch_one(pool).await?;

    let book =  Book {
        title: row.get("title"),
        author: row.get("author")
    };

    print!("title = {}",book.title);

    Ok(book)
}

#[OpenApi]
impl Api {
    /// Hello world
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello World")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let url = "postgres://postgres:bazaDanych@localhost:5432/postgres";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let res = sqlx::query("SELECT 12 + 4 as sum")
        .fetch_one(&pool)
        .await?;

    let book =  Book {
        title: "Hobbit".to_string(),
        author: "Tolkien".to_string()
    };
    create(&book, &pool).await?;
    let b = read(&pool).await?;
    print!("{:?}",b);

    let sum:i32 = res.get("sum");
    print!("{}",sum);

    // let api_service =
    //     OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:3000");
    // let ui = api_service.swagger_ui();
    // let app = Route::new().nest("/", api_service).nest("/docs", ui);

    // Server::new(TcpListener::bind("127.0.0.1:3000"))
    //     .run(app)
    //     .await?;

    Ok(())
}