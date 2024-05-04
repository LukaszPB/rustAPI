
use poem_openapi::{payload::PlainText, OpenApi, Object, payload::Json};
use poem::web::Data;
use sqlx::Row;
pub struct BookApi;

#[derive(Debug, Object)]
struct Book {
    title: String,
    author: String
}

#[OpenApi]
impl BookApi {
    #[oai(path = "/get", method = "get")]
    async fn get_books(&self, pool: Data<&sqlx::PgPool>) -> Json<Book> {
        let q = "SELECT title, author FROM book";
        let query = sqlx::query(q);
        
        let mut conn = pool.acquire().await.unwrap();

        let row = query.fetch_one(&mut conn).await.unwrap();

        let book =  Book {
            title: row.get("title"),
            author: row.get("author")
        };

        Json(book)
    }
    
    #[oai(path = "/post", method = "post")]
    async fn add_book(&self) -> PlainText<&'static str> {
        PlainText("Hello World")
    }
    // async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    //     let query = "INSERT INTO book (title, author) VALUES ($1, $2)";

    //     sqlx::query(query)
    //         .bind(&book.title)
    //         .bind(&book.author)
    //         .execute(pool)
    //         .await?;

    //     Ok(())
    // }
}