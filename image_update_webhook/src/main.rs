// src/main.rs
use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::process::Command;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone)]
struct AppState {
    webhook_secret: String,
}

#[derive(Deserialize, Debug)]
struct WebhookPayload {
    repository: Repository,
    #[serde(default)]
    package: Option<Package>,
    #[serde(default)]
    after: Option<String>, // Commit SHA for push events
}

#[derive(Deserialize, Debug)]
struct Repository {
    full_name: String,
}

#[derive(Deserialize, Debug)]
struct Package {
    package_version: PackageVersion,
}

#[derive(Deserialize, Debug)]
struct PackageVersion {
    #[serde(default)]
    container_metadata: Option<ContainerMetadata>,
}

#[derive(Deserialize, Debug)]
struct ContainerMetadata {
    tag: Tag,
}

#[derive(Deserialize, Debug)]
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

    tracing::debug!("Verifying signature: {}", signature);

    let mut mac = match HmacSha256::new_from_slice(secret.as_bytes()) {
        Ok(m) => m,
        Err(e) => {
            tracing::error!("Failed to create HMAC: {}", e);
            return false;
        }
    };

    tracing::debug!("Updating HMAC with payload of length: {}", payload.len());

    mac.update(payload);

    let expected = match hex::decode(signature) {
        Ok(bytes) => bytes,
        Err(e) => {
            tracing::error!("Failed to decode signature: {}", e);
            return false;
        }
    };

    mac.verify_slice(&expected).is_ok()
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "service": "image_update_webhook"
    }))
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

    tracing::debug!("Received webhook with signature: {}", signature);

    if !verify_signature(&state.webhook_secret, &body, signature) {
        tracing::warn!("Signature verification failed");
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

    tracing::debug!("Received event type: {}", event_type);

    if event_type != "package" {
        tracing::info!("Event type '{}' ignored", event_type);
        return Ok(Json(Response {
            message: format!("Event '{}' ignored", event_type),
            output: None,
            error: None,
        }));
    }

    tracing::debug!("Event type '{}' accepted", event_type);

    // Parse payload
    let payload: WebhookPayload = match serde_json::from_slice(&body) {
        Ok(p) => p,
        Err(e) => {
            tracing::error!("Failed to parse JSON payload: {}: {:?}", e, body);
            return Err((
                StatusCode::BAD_REQUEST,
                Json(Response {
                    message: "Invalid JSON payload".to_string(),
                    output: None,
                    error: Some(e.to_string()),
                }),
            ));
        }
    };

    tracing::debug!("Processing payload: {:?}", payload);

    let repo_name = payload.repository.full_name.split('/').last().unwrap_or("");

    if repo_name.is_empty() {
        tracing::error!("Could not determine repository name from full_name: {}", payload.repository.full_name);
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
    let image_tag = if event_type == "package" {
        tracing::debug!("Extracting image tag for registry_package event");
        // For package events, get the tag from the package metadata
        payload
            .package
            .and_then(|p| p.package_version.container_metadata)
            .map(|m| m.tag.name)
            .unwrap_or_else(|| "latest".to_string())
    } else {
        tracing::debug!("Extracting image tag for unknown event type: defaulting to 'latest'");
        "latest".to_string()
    };

    // Update Docker service
    let image = format!("ghcr.io/{}:{}", payload.repository.full_name, image_tag);
    let service = format!("homelab_{}", repo_name);

    tracing::info!("Updating service {} to image: {}", service, image);

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
        Ok(result) if result.status.success() => {
            tracing::info!("Service {} updated successfully", service);
            Ok(Json(Response {
                message: format!("Successfully updated {}", service),
                output: Some(String::from_utf8_lossy(&result.stdout).to_string()),
                error: None,
            }))
        },
        Ok(result) => {
            tracing::error!("Service {} update failed: {}", service, String::from_utf8_lossy(&result.stderr));
            Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Response {
                message: "Update failed".to_string(),
                output: Some(String::from_utf8_lossy(&result.stdout).to_string()),
                error: Some(String::from_utf8_lossy(&result.stderr).to_string()),
            }),
        ))},
        Err(e) => {
            tracing::error!("Failed to execute docker command: {}", e);
            Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Response {
                message: "Failed to execute command".to_string(),
                output: None,
                error: Some(e.to_string()),
            }),
        ))},
    }
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let webhook_secret_file = std::env::var("WEBHOOK_SECRET_FILE").unwrap_or("".to_string());
    let webhook_secret = std::fs::read_to_string(webhook_secret_file)
        .or_else(|_| std::env::var("WEBHOOK_SECRET"))
        .expect("WEBHOOK_SECRET_FILE or WEBHOOK_SECRET must be set via environment variable")
        .trim()
        .to_string();

    let state = Arc::new(AppState { webhook_secret });

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/webhook/deploy", post(webhook_deploy))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind to port 8080");

    println!("Webhook receiver listening on port 8080");

    axum::serve(listener, app).await.expect("Server error");
}
