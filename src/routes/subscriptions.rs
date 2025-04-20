use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgConnection;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    connection: web::Data<PgConnection>,
) -> HttpResponse {
    // we are binding dynamic data to our INSERT query. $1 refers to the first argument
    // passed to query! after the query itself, $2 to the second and so forth. query!
    // verifies at compile-time that the provided number of arguments matches what the
    // query expects as well as that their types are compatible (e.g. you can’t pass a number as id);
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        // Generate random UUID
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // We use `get_ref` to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`
    .execute(connection.get_ref())
    .await;
    HttpResponse::Ok().finish()
}
