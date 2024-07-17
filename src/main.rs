use std::{collections::HashMap, sync::{Arc, Mutex}};
use axum::{
    extract::{Json, Path, State}, http::StatusCode, response::{IntoResponse, Response}, routing::{get, post}, Router
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use uuid::Uuid;

#[derive(Deserialize, Clone, Serialize, Debug)]
struct Movie {
    id: String,
    name: String,
    year: u16,
    was_good: bool,
}

#[derive(Clone, Debug)]
struct AppState {
    db: Arc<Mutex<HashMap<String, Movie>>>
}

impl AppState {
    fn new() -> Self {
        Self {
            db: Arc::new(Mutex::new(HashMap::new()))
        }
    }
}

struct AppResponse<T>(pub T);

impl<T> IntoResponse for AppResponse<T> 
where 
    Json<T>: IntoResponse
{
    fn into_response(self) -> Response {
        Json(self.0).into_response()
    }
}

#[tokio::main]
async fn main() {

    let state = AppState::new();
    // build our application with a single route
    let app = Router::new()
        .route("/health", get(|| async { "up" }))
        .route("/movie", post(post_movie))
        .route("/movie/:id", get(get_movie))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn post_movie(State(state): State<AppState>, Json(movie): Json<Movie>) -> Json<Value> {
    let movie_id = Uuid::new_v4().to_string();
    state.db.lock().as_mut().unwrap().insert(movie_id.clone(), movie);

    Json(json!({"id": movie_id}))
}

// #[axum_macros::debug_handler]
async fn get_movie(State(state): State<AppState>, Path(movie_id): Path<String>) -> Result<AppResponse<Movie>, StatusCode> {

    let cleaned_id = clean_id(movie_id);

    if let Some(movie) = state.db.lock().unwrap().get(&cleaned_id) {
        println!("Ok");
        Ok(AppResponse(movie.clone()))
    } else {
        println!("Error");
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


fn clean_id(movie_id: String) -> String {
    let mut foo = movie_id.chars();
    foo.next();
    foo.next_back();

    foo.as_str().to_string()

}