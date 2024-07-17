use std::collections::HashMap;
use axum::{
    extract::{Json, State},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
struct Movie {
    id: String,
    name: String,
    year: u16,
    was_good: bool,
}

#[derive(Clone, Debug)]
struct AppState {
    db: HashMap<String, Movie>
}

impl AppState {
    fn new() -> Self {
        Self {
            db: HashMap::new()
        }
    }
}

#[tokio::main]
async fn main() {

    let state = AppState::new();
    // build our application with a single route
    let app = Router::new()
        .route("/health", get(|| async { "up" }))
        .route("/movie", post(post_movie))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// #[axum_macros::debug_handler]
async fn post_movie(State(mut state): State<AppState>, Json(movie): Json<Movie>) -> Json<Value> {
    let movie_id = Uuid::new_v4().to_string();
    state.db.insert(movie_id.clone(), movie);

    Json(json!({"id": movie_id}))
}
