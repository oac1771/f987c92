use axum::{
    routing::{get, post},
    extract::Json,
    Router,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Movie {
    id: String,
    name: String,
    year: u16,
    was_good: bool
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
    .route("/health", get(|| async { "up" }))
    .route("/movie", post(post_movie));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn post_movie(Json(movie): Json<Movie>) {
    println!("{:?}", movie);
}