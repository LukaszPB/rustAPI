use poem_openapi::{OpenApi, Object, payload::Json, ApiResponse, param::Path};
use poem::web::Data;
use sqlx::Row;
pub struct AuthorApi;

#[derive(Debug, Object)]
struct Author {
    id: i64,
    first_name: String,
    last_name: String,
    age: i32
}

#[derive(ApiResponse)]
enum FindAuthorResponse {
    #[oai(status = 200)]
    Ok(Json<Author>),
    #[oai(status = 404)]
    NotFound,
}

#[derive(ApiResponse)]
enum FindAuthorsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<Author>>),
}

#[derive(ApiResponse)]
enum UpdateAuthorResponse {
    #[oai(status = 200)]
    Ok,
    #[oai(status = 404)]
    NotFound,
}

#[derive(ApiResponse)]
enum AddAuthorResponse {
    #[oai(status = 200)]
    Ok,
    #[oai(status = 404)]
    Failure,
}

#[derive(ApiResponse)]
enum DeleteAuthorResponse {
    #[oai(status = 200)]
    Ok,
    #[oai(status = 404)]
    NotFound,
}

#[OpenApi]
impl AuthorApi {
    #[oai(path = "/get_authors", method = "get")]
    async fn get_authors(&self, pool: Data<&sqlx::PgPool>) -> FindAuthorsResponse {
        let q = "SELECT id, first_name, last_name, age FROM Author";
        let query = sqlx::query(q);

        let rows = query.fetch_all(pool.0).await.unwrap();

        let authors = rows.iter().map(|row| { 
            Author {
                id: row.get("id"),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                age: row.get("age")
        }}).collect();

        FindAuthorsResponse::Ok(Json(authors))
    }

    #[oai(path = "/get_author/:id", method = "get")]
    async fn get_author(&self, id:Path<i64>, pool: Data<&sqlx::PgPool>) -> FindAuthorResponse {
        let q = "SELECT id, first_name, last_name, age FROM Author WHERE id = ($1)";
        let query = sqlx::query(q)
            .bind(id.0);

        let row = query.fetch_optional(pool.0).await.unwrap();

        match row {
            Some(r) => FindAuthorResponse::Ok(Json(Author {
                id: r.get("id"),
                first_name: r.get("first_name"),
                last_name: r.get("last_name"),
                age: r.get("age")
            })),
            None => FindAuthorResponse::NotFound,
        }
    }

    #[oai(path = "/update_author/:id", method = "put")]
    async fn update_author(&self, id: Path<i64>, pool: Data<&sqlx::PgPool>, author: Json<Author>) -> UpdateAuthorResponse {
        let query = "UPDATE Author SET first_name = $1, last_name = $2, age = $3 WHERE id = $4";

        let result = sqlx::query(query)
            .bind(author.0.first_name)
            .bind(author.0.last_name)
            .bind(author.0.age)
            .bind(id.0)
            .execute(pool.0)
            .await;

        match result {
            Ok(_) => UpdateAuthorResponse::Ok,
            Err(_) => UpdateAuthorResponse::NotFound,
        }
    }
    
    #[oai(path = "/add_author", method = "post")]
    async fn add_author(&self, pool: Data<&sqlx::PgPool>, author: Json<Author>) -> AddAuthorResponse {
        let query = "INSERT INTO Author (first_name, last_name, age) VALUES ($1, $2, $3)";

        let result = sqlx::query(query)
            .bind(author.0.first_name)
            .bind(author.0.last_name)
            .bind(author.0.age)
            .execute(pool.0)
            .await;

        match result {
            Ok(_) => AddAuthorResponse::Ok,
            Err(_) => AddAuthorResponse::Failure,
        }
    }
    #[oai(path = "/delete_author/:id", method = "delete")]
    async fn delete_author(&self, id:Path<i64>, pool: Data<&sqlx::PgPool>) -> DeleteAuthorResponse {
        let query = "DELETE FROM Author WHERE id = $1";

        let result = sqlx::query(query)
            .bind(id.0)
            .execute(pool.0)
            .await;

        match result {
            Ok(_) => DeleteAuthorResponse::Ok,
            Err(_) => DeleteAuthorResponse::NotFound,
        }
    }
}