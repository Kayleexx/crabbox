use axum::{
    extract::{Path, State, Json as AxumJson},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};

mod models;
use models::{CreateUrlRequest, Url};

type UrlMap = Arc<Mutex<HashMap<String, Url>>>;

#[tokio::main]
async fn main() {
    let urls: UrlMap = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/", get(|| async { "Hello from Axum Shortener!" }))
        .route("/shorten", post(shorten_url))
        .route("/expand/:short", get(expand_url))
        .with_state(urls.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Server running at http://{addr}");

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn shorten_url(
    State(urls): State<UrlMap>,
    AxumJson(input): AxumJson<CreateUrlRequest>,
) -> impl IntoResponse {
    let mut urls = urls.lock().await;

    let short_url = format!("https://short.ly/{}", generate_short_url());
    let url = Url {
        original: input.original.clone(),
        short: short_url.clone(),
    };

    urls.insert(short_url.clone(), url.clone());

    Json(url)
}

async fn expand_url(
    State(urls): State<UrlMap>,
    Path(short_url): Path<String>,
) -> impl IntoResponse {
    let urls = urls.lock().await;

    match urls.get(&short_url) {
        Some(url) => Json(serde_json::json!(url)),
        None => Json(serde_json::json!({ "error": "URL not found" })),
    }
}

fn generate_short_url() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let chars: Vec<char> =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    (0..6)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
}
