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
        .route("/movie/:movie_id", get(get_movie))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// I know that the unwraps after the as_mut() once getting the lock for the state is not good and should have
// proper error handling. Due to time I left them in but ideally would create error handling workflow similar to 
// this for custom error types: https://docs.rs/axum/latest/axum/response/trait.IntoResponse.html#implementing-intoresponse
// and have enum variants for different cases
async fn post_movie(State(state): State<AppState>, Json(movie): Json<Movie>) -> Json<Value> {
    let movie_id = Uuid::new_v4().to_string();
    state.db.lock().as_mut().unwrap().insert(movie_id.clone(), movie);

    Json(json!({"id": movie_id}))
}

#[axum_macros::debug_handler]
async fn get_movie(State(state): State<AppState>, Path(movie_id): Path<String>) -> Result<AppResponse<Movie>, StatusCode> {

    let cleaned_id = clean_id(movie_id);

    if let Some(movie) = state.db.lock().unwrap().get(&cleaned_id.to_string()) {
        Ok(AppResponse(movie.clone()))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

// I had to create his method to clean up the movie_id path variable as it came with leading and trailing '\'
// and would not match to what was saved in state. 
fn clean_id(movie_id: String) -> String {
    let mut result = movie_id.chars();
    result.next();
    result.next_back();

    result.as_str().to_string()

}