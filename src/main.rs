use axum::{
    Form, Router,
    extract::{DefaultBodyLimit, State},
    http::StatusCode,
    routing::{get, post},
};
use clap::Parser;
use serde::Deserialize;
use std::env;
use std::net::{Ipv4Addr, SocketAddr};
use std::process::Command;
use std::sync::OnceLock;

const VERSION: &str = env!("CARGO_PKG_VERSION");

static ACCEPTABLE_TAGS: OnceLock<Vec<String>> = OnceLock::new();

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    bind: Option<Ipv4Addr>,

    #[arg(short, long, default_value_t = 8989)]
    port: u16,
}

#[derive(Clone)]
struct AppState;

#[derive(Debug, Deserialize)]
struct JobRequest {
    tagid: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    ACCEPTABLE_TAGS
        .set(load_acceptable_tags())
        .expect("acceptable tags already initialized");

    let port = args.port;
    let app = Router::new()
        .route("/version", get(version))
        .route(
            "/restart-bluetooth",
            post(restart_bluetooth).layer(DefaultBodyLimit::max(1024)),
        )
        .with_state(AppState);

    let addr = SocketAddr::from((
        args.bind.unwrap_or(Ipv4Addr::LOCALHOST),
        port,
    ));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind server socket");

    println!("listening on http://{}", addr);
    axum::serve(listener, app).await.expect("server failed");
}

async fn restart_bluetooth(
    State(_state): State<AppState>,
    Form(payload): Form<JobRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let tagid = payload.tagid.to_ascii_uppercase();
    let acceptable_tags = ACCEPTABLE_TAGS
        .get()
        .expect("acceptable tags not initialized");

    if !acceptable_tags.iter().any(|tag| tag == &tagid) {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("invalid tagid")));
    }

    let output = Command::new("sudo")
        .args(["systemctl", "restart", "bluetooth"])
        .output()
        .map_err(|error| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to execute process: {error}"),
            )
        })?;

    if !output.status.success() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("systemctl failed with status {}", output.status),
        ));
    }

    Ok(StatusCode::OK)
}

async fn version() -> (StatusCode, &'static str) {
    (StatusCode::OK, VERSION)
}

fn load_acceptable_tags() -> Vec<String> {
    env::var("ALLOWED_TAGS")
        .unwrap_or_else(|_err| {
            eprintln!("WARNING: environment variable ALLOWED_TAGS is not defined and will be treated as empty");
            "".to_string()
        })
        .split(|c: char| c == ',' || c.is_whitespace())
        .map(|tag| tag.trim().to_ascii_uppercase())
        .filter(|tag| !tag.is_empty())
        .collect()
}
