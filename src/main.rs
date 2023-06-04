use dotenvy_macro::dotenv;
use intro_axum::run;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file at compile time
    let database_uri = dotenv!("DATABASE_URL");

    run(database_uri).await
}
