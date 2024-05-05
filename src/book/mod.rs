
use poem_openapi::{OpenApi, Object, payload::Json, ApiResponse, param::Path};
use poem::web::Data;
use sqlx::Row;
pub struct BookApi;

#[derive(Debug, Object)]
struct Book {
    id: i64,
    title: String,
    id_author: i64
}

#[derive(ApiResponse)]
enum FindBookResponse {
    #[oai(status = 200)]
    Ok(Json<Book>),
    #[oai(status = 404)]
    NotFound,
}

#[derive(ApiResponse)]
enum FindBooksResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<Book>>),
}

#[derive(ApiResponse)]
enum UpdateBookResponse {
    #[oai(status = 200)]
    Ok,
    #[oai(status = 404)]
    NotFound,
}

#[derive(ApiResponse)]
enum AddBookResponse {
    #[oai(status = 200)]
    Ok,
    #[oai(status = 404)]
    Failure,
}

#[derive(ApiResponse)]
enum DeleteBookResponse {
    #[oai(status = 200)]
    Ok,
    #[oai(status = 404)]
    NotFound,
}

#[OpenApi]
impl BookApi {
    #[oai(path = "/get_books", method = "get")]
    async fn get_books(&self, pool: Data<&sqlx::PgPool>) -> FindBooksResponse {
        let q = "SELECT id, title, id_author FROM Book";
        let query = sqlx::query(q);

        let rows = query.fetch_all(pool.0).await.unwrap();

        let books = rows.iter().map(|row| { 
            Book {
                id: row.get("id"),
                title: row.get("title"),
                id_author: row.get("id_author")
        }}).collect();

        FindBooksResponse::Ok(Json(books))
    }

    #[oai(path = "/get_book/:id", method = "get")]
    async fn get_book(&self, id:Path<i64>, pool: Data<&sqlx::PgPool>) -> FindBookResponse {
        let q = "SELECT id, title, id_author FROM Book WHERE id = ($1)";
        let query = sqlx::query(q)
            .bind(id.0);

        let row = query.fetch_optional(pool.0).await.unwrap();

        match row {
            Some(r) => FindBookResponse::Ok(Json(Book {
                id: r.get("id"),
                title: r.get("title"),
                id_author: r.get("id_author")
            })),
            None => FindBookResponse::NotFound,
        }
    }

    #[oai(path = "/update_book/:id", method = "put")]
    async fn update_book(&self, id: Path<i64>, pool: Data<&sqlx::PgPool>, book: Json<Book>) -> UpdateBookResponse {
        let query = "UPDATE Book SET title = $1, id_author = $2 WHERE id = $3";

        let result = sqlx::query(query)
            .bind(book.0.title)
            .bind(book.0.id_author)
            .bind(id.0)
            .execute(pool.0)
            .await;

        match result {
            Ok(_) => UpdateBookResponse::Ok,
            Err(_) => UpdateBookResponse::NotFound,
        }
    }
    
    #[oai(path = "/add_book", method = "post")]
    async fn add_book(&self, pool: Data<&sqlx::PgPool>, book: Json<Book>) -> AddBookResponse {
        let query = "INSERT INTO Book (title, id_author) VALUES ($1, $2)";

        let result = sqlx::query(query)
            .bind(book.0.title)
            .bind(book.0.id_author)
            .execute(pool.0)
            .await;

        match result {
            Ok(_) => AddBookResponse::Ok,
            Err(_) => AddBookResponse::Failure,
        }
    }
    #[oai(path = "/delete_book/:id", method = "delete")]
    async fn delete_book(&self, id:Path<i64>, pool: Data<&sqlx::PgPool>) -> DeleteBookResponse {
        let query = "DELETE FROM Book WHERE id = $1";

        let result = sqlx::query(query)
            .bind(id.0)
            .execute(pool.0)
            .await;

        match result {
            Ok(_) => DeleteBookResponse::Ok,
            Err(_) => DeleteBookResponse::NotFound,
        }
    }
}