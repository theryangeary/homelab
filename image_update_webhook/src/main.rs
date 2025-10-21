// src/main.rs
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::process::Command;
use std::sync::Arc;
use tokio::net::TcpListener;

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone)]
struct AppState {
    webhook_secret: String,
}

#[derive(Deserialize)]
struct WebhookPayload {
    repository: Repository,
    #[serde(default)]
    package: Option<Package>,
    #[serde(default)]
    after: Option<String>, // Commit SHA for push events
}

#[derive(Deserialize)]
struct Repository {
    full_name: String,
}

#[derive(Deserialize)]
struct Package {
    package_version: PackageVersion,
}

#[derive(Deserialize)]
struct PackageVersion {
    #[serde(default)]
    container_metadata: Option<ContainerMetadata>,
}

#[derive(Deserialize)]
struct ContainerMetadata {
    tag: Tag,
}

#[derive(Deserialize)]
struct Tag {
    name: String,
}

#[derive(Serialize)]
struct Response {
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

fn verify_signature(secret: &str, payload: &[u8], signature: &str) -> bool {
    let signature = match signature.strip_prefix("sha256=") {
        Some(sig) => sig,
        None => return false,
    };

    let mut mac = match HmacSha256::new_from_slice(secret.as_bytes()) {
        Ok(m) => m,
        Err(_) => return false,
    };
    
    mac.update(payload);
    
    let expected = match hex::decode(signature) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };
    
    mac.verify_slice(&expected).is_ok()
}

async fn health() -> impl IntoResponse {
    Json(Response {
        message: "healthy".to_string(),
        output: None,
        error: None,
    })
}

async fn webhook_deploy(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: axum::body::Bytes,
) -> Result<Json<Response>, (StatusCode, Json<Response>)> {
    // Verify signature
    let signature = headers
        .get("X-Hub-Signature-256")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if !verify_signature(&state.webhook_secret, &body, signature) {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(Response {
                message: "Invalid signature".to_string(),
                output: None,
                error: Some("Signature verification failed".to_string()),
            }),
        ));
    }

    // Check event type
    let event_type = headers
        .get("X-GitHub-Event")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    // Accept both package and push events
    if event_type != "registry_package" && event_type != "push" {
        return Ok(Json(Response {
            message: format!("Event '{}' ignored", event_type),
            output: None,
            error: None,
        }));
    }

    // Parse payload
    let payload: WebhookPayload = match serde_json::from_slice(&body) {
        Ok(p) => p,
        Err(e) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(Response {
                    message: "Invalid JSON payload".to_string(),
                    output: None,
                    error: Some(e.to_string()),
                }),
            ))
        }
    };

    let repo_name = payload
        .repository
        .full_name
        .split('/')
        .last()
        .unwrap_or("");

    if repo_name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(Response {
                message: "Could not determine repository name".to_string(),
                output: None,
                error: None,
            }),
        ));
    }

    // Extract image tag from payload
    let image_tag = if event_type == "registry_package" {
        // For package events, get the tag from the package metadata
        payload
            .package
            .and_then(|p| p.package_version.container_metadata)
            .map(|m| m.tag.name)
            .unwrap_or_else(|| "latest".to_string())
    } else if event_type == "push" {
        // For push events, use the commit SHA
        payload
            .after
            .map(|sha| {
                // Use short SHA (first 7 chars) or full SHA
                if sha.len() >= 7 {
                    sha[..7].to_string()
                } else {
                    sha
                }
            })
            .unwrap_or_else(|| "latest".to_string())
    } else {
        "latest".to_string()
    };

    println!("Updating service with tag: {}", image_tag);

    // Update Docker service
    let image = format!("ghcr.io/{}:{}", payload.repository.full_name, image_tag);
    let service = format!("homelab_{}", repo_name);

    let output = Command::new("docker")
        .args([
            "service",
            "update",
            "--force",
            "--with-registry-auth",
            "--image",
            &image,
            &service,
        ])
        .output();

    match output {
        Ok(result) if result.status.success() => Ok(Json(Response {
            message: format!("Successfully updated {}", service),
            output: Some(String::from_utf8_lossy(&result.stdout).to_string()),
            error: None,
        })),
        Ok(result) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Response {
                message: "Update failed".to_string(),
                output: Some(String::from_utf8_lossy(&result.stdout).to_string()),
                error: Some(String::from_utf8_lossy(&result.stderr).to_string()),
            }),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Response {
                message: "Failed to execute command".to_string(),
                output: None,
                error: Some(e.to_string()),
            }),
        )),
    }
}

#[tokio::main]
async fn main() {
    let webhook_secret_file = std::env::var("WEBHOOK_SECRET_FILE").unwrap_or("".to_string());
    let webhook_secret = std::fs::read_to_string(webhook_secret_file)
        .or_else(|_| std::env::var("WEBHOOK_SECRET"))
        .expect("WEBHOOK_SECRET_FILE or WEBHOOK_SECRET must be set via environment variable")
        .trim()
        .to_string();

    let state = Arc::new(AppState { webhook_secret });

    let app = Router::new()
        .route("/health", get(health))
        .route("/webhook/deploy", post(webhook_deploy))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind to port 8080");

    println!("Webhook receiver listening on port 8080");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
