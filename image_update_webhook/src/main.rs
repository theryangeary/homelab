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
        tracing::error!(
            "Could not determine repository name from full_name: {}",
            payload.repository.full_name
        );
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
    let image_tag = match payload
        .package
        .and_then(|p| p.package_version.container_metadata)
        .map(|m| m.tag.name)
        .as_deref()
    {
        Some("") | None => {
            tracing::error!(
                "No image tag found in payload for repository: {}",
                payload.repository.full_name
            );
            return Err((
                StatusCode::OK,
                Json(Response {
                    message: "No image tag found in payload. Ignored.".to_string(),
                    output: None,
                    error: None,
                }),
            ));
        }
        Some(tag_name) => {
            tracing::debug!("Found image tag: {}", tag_name);
            tag_name.to_string()
        }
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
        }
        Ok(result) => {
            tracing::error!(
                "Service {} update failed: {}",
                service,
                String::from_utf8_lossy(&result.stderr)
            );
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Response {
                    message: "Update failed".to_string(),
                    output: Some(String::from_utf8_lossy(&result.stdout).to_string()),
                    error: Some(String::from_utf8_lossy(&result.stderr).to_string()),
                }),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to execute docker command: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Response {
                    message: "Failed to execute command".to_string(),
                    output: None,
                    error: Some(e.to_string()),
                }),
            ))
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_deserialization() {
        let json_data = r#"{
  "action": "published",
  "package": {
    "id": 9056324,
    "name": "www",
    "namespace": "theryangeary",
    "description": "",
    "ecosystem": "CONTAINER",
    "package_type": "CONTAINER",
    "html_url": "https://github.com/theryangeary/packages/9056324",
    "created_at": "2025-09-19T19:21:05Z",
    "updated_at": "2025-11-03T13:50:26Z",
    "owner": {
      "login": "theryangeary",
      "id": 7076013,
      "node_id": "MDQ6VXNlcjcwNzYwMTM=",
      "avatar_url": "https://avatars.githubusercontent.com/u/7076013?v=4",
      "gravatar_id": "",
      "url": "https://api.github.com/users/theryangeary",
      "html_url": "https://github.com/theryangeary",
      "followers_url": "https://api.github.com/users/theryangeary/followers",
      "following_url": "https://api.github.com/users/theryangeary/following{/other_user}",
      "gists_url": "https://api.github.com/users/theryangeary/gists{/gist_id}",
      "starred_url": "https://api.github.com/users/theryangeary/starred{/owner}{/repo}",
      "subscriptions_url": "https://api.github.com/users/theryangeary/subscriptions",
      "organizations_url": "https://api.github.com/users/theryangeary/orgs",
      "repos_url": "https://api.github.com/users/theryangeary/repos",
      "events_url": "https://api.github.com/users/theryangeary/events{/privacy}",
      "received_events_url": "https://api.github.com/users/theryangeary/received_events",
      "type": "User",
      "user_view_type": "public",
      "site_admin": false
    },
    "package_version": {
      "id": 564296960,
      "version": "sha256:89160c587d0c36bf54090497a465445c14980ba48efbcf3b0d03c49b76f3feb8",
      "name": "sha256:89160c587d0c36bf54090497a465445c14980ba48efbcf3b0d03c49b76f3feb8",
      "description": "",
      "summary": "",
      "manifest": "",
      "html_url": "https://github.com/users/theryangeary/packages/container/www/564296960",
      "target_commitish": "main",
      "target_oid": "0e2a89b191332bb87b4f2fefef2f597eba2197b3",
      "created_at": "0001-01-01T00:00:00Z",
      "updated_at": "0001-01-01T00:00:00Z",
      "metadata": [

      ],
      "container_metadata": {
        "tag": {
          "name": "0e2a89b",
          "digest": "sha256:89160c587d0c36bf54090497a465445c14980ba48efbcf3b0d03c49b76f3feb8"
        },
        "labels": {
          "description": "",
          "source": "",
          "revision": "",
          "image_url": "",
          "licenses": "",
          "all_labels": {
            "github.internal.platforms": "[{\"digest\":\"sha256:e312a84da724b2de60d64c3241b69e6c89c3ce7ec82e55cd9793ddc1978008c1\",\"architecture\":\"amd64\",\"os\":\"linux\"},{\"digest\":\"sha256:184b604b46eb2c6a21d80f89471a3f562dd42a8c9d1ba45021bc9f3a4b10f0e9\",\"architecture\":\"arm64\",\"os\":\"linux\"}]"
          }
        },
        "manifest": {
          "digest": "sha256:89160c587d0c36bf54090497a465445c14980ba48efbcf3b0d03c49b76f3feb8",
          "media_type": "application/vnd.docker.distribution.manifest.list.v2+json",
          "uri": "repositories/theryangeary/www/manifests/sha256:89160c587d0c36bf54090497a465445c14980ba48efbcf3b0d03c49b76f3feb8",
          "size": 685,
          "config": {
            "digest": "",
            "media_type": "",
            "size": 0
          },
          "layers": [

          ]
        }
      },
      "package_files": [

      ],
      "author": {
        "login": "theryangeary",
        "id": 7076013,
        "node_id": "MDQ6VXNlcjcwNzYwMTM=",
        "avatar_url": "https://avatars.githubusercontent.com/u/7076013?v=4",
        "gravatar_id": "",
        "url": "https://api.github.com/users/theryangeary",
        "html_url": "https://github.com/theryangeary",
        "followers_url": "https://api.github.com/users/theryangeary/followers",
        "following_url": "https://api.github.com/users/theryangeary/following{/other_user}",
        "gists_url": "https://api.github.com/users/theryangeary/gists{/gist_id}",
        "starred_url": "https://api.github.com/users/theryangeary/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/theryangeary/subscriptions",
        "organizations_url": "https://api.github.com/users/theryangeary/orgs",
        "repos_url": "https://api.github.com/users/theryangeary/repos",
        "events_url": "https://api.github.com/users/theryangeary/events{/privacy}",
        "received_events_url": "https://api.github.com/users/theryangeary/received_events",
        "type": "User",
        "user_view_type": "public",
        "site_admin": false
      },
      "installation_command": "docker pull ghcr.io/theryangeary/www:0e2a89b",
      "package_url": "ghcr.io/theryangeary/www:0e2a89b"
    },
    "registry": {
      "about_url": "https://docs.github.com/packages/learn-github-packages/introduction-to-github-packages",
      "name": "GitHub CONTAINER registry",
      "type": "CONTAINER",
      "url": "https://CONTAINER.pkg.github.com/theryangeary",
      "vendor": "GitHub Inc"
    }
  },
  "repository": {
    "id": 1060841726,
    "node_id": "R_kgDOPzso_g",
    "name": "www",
    "full_name": "theryangeary/www",
    "private": false,
    "owner": {
      "login": "theryangeary",
      "id": 7076013,
      "node_id": "MDQ6VXNlcjcwNzYwMTM=",
      "avatar_url": "https://avatars.githubusercontent.com/u/7076013?v=4",
      "gravatar_id": "",
      "url": "https://api.github.com/users/theryangeary",
      "html_url": "https://github.com/theryangeary",
      "followers_url": "https://api.github.com/users/theryangeary/followers",
      "following_url": "https://api.github.com/users/theryangeary/following{/other_user}",
      "gists_url": "https://api.github.com/users/theryangeary/gists{/gist_id}",
      "starred_url": "https://api.github.com/users/theryangeary/starred{/owner}{/repo}",
      "subscriptions_url": "https://api.github.com/users/theryangeary/subscriptions",
      "organizations_url": "https://api.github.com/users/theryangeary/orgs",
      "repos_url": "https://api.github.com/users/theryangeary/repos",
      "events_url": "https://api.github.com/users/theryangeary/events{/privacy}",
      "received_events_url": "https://api.github.com/users/theryangeary/received_events",
      "type": "User",
      "user_view_type": "public",
      "site_admin": false
    },
    "html_url": "https://github.com/theryangeary/www",
    "description": "www.ryangeary.dev",
    "fork": false,
    "url": "https://api.github.com/repos/theryangeary/www",
    "forks_url": "https://api.github.com/repos/theryangeary/www/forks",
    "keys_url": "https://api.github.com/repos/theryangeary/www/keys{/key_id}",
    "collaborators_url": "https://api.github.com/repos/theryangeary/www/collaborators{/collaborator}",
    "teams_url": "https://api.github.com/repos/theryangeary/www/teams",
    "hooks_url": "https://api.github.com/repos/theryangeary/www/hooks",
    "issue_events_url": "https://api.github.com/repos/theryangeary/www/issues/events{/number}",
    "events_url": "https://api.github.com/repos/theryangeary/www/events",
    "assignees_url": "https://api.github.com/repos/theryangeary/www/assignees{/user}",
    "branches_url": "https://api.github.com/repos/theryangeary/www/branches{/branch}",
    "tags_url": "https://api.github.com/repos/theryangeary/www/tags",
    "blobs_url": "https://api.github.com/repos/theryangeary/www/git/blobs{/sha}",
    "git_tags_url": "https://api.github.com/repos/theryangeary/www/git/tags{/sha}",
    "git_refs_url": "https://api.github.com/repos/theryangeary/www/git/refs{/sha}",
    "trees_url": "https://api.github.com/repos/theryangeary/www/git/trees{/sha}",
    "statuses_url": "https://api.github.com/repos/theryangeary/www/statuses/{sha}",
    "languages_url": "https://api.github.com/repos/theryangeary/www/languages",
    "stargazers_url": "https://api.github.com/repos/theryangeary/www/stargazers",
    "contributors_url": "https://api.github.com/repos/theryangeary/www/contributors",
    "subscribers_url": "https://api.github.com/repos/theryangeary/www/subscribers",
    "subscription_url": "https://api.github.com/repos/theryangeary/www/subscription",
    "commits_url": "https://api.github.com/repos/theryangeary/www/commits{/sha}",
    "git_commits_url": "https://api.github.com/repos/theryangeary/www/git/commits{/sha}",
    "comments_url": "https://api.github.com/repos/theryangeary/www/comments{/number}",
    "issue_comment_url": "https://api.github.com/repos/theryangeary/www/issues/comments{/number}",
    "contents_url": "https://api.github.com/repos/theryangeary/www/contents/{+path}",
    "compare_url": "https://api.github.com/repos/theryangeary/www/compare/{base}...{head}",
    "merges_url": "https://api.github.com/repos/theryangeary/www/merges",
    "archive_url": "https://api.github.com/repos/theryangeary/www/{archive_format}{/ref}",
    "downloads_url": "https://api.github.com/repos/theryangeary/www/downloads",
    "issues_url": "https://api.github.com/repos/theryangeary/www/issues{/number}",
    "pulls_url": "https://api.github.com/repos/theryangeary/www/pulls{/number}",
    "milestones_url": "https://api.github.com/repos/theryangeary/www/milestones{/number}",
    "notifications_url": "https://api.github.com/repos/theryangeary/www/notifications{?since,all,participating}",
    "labels_url": "https://api.github.com/repos/theryangeary/www/labels{/name}",
    "releases_url": "https://api.github.com/repos/theryangeary/www/releases{/id}",
    "deployments_url": "https://api.github.com/repos/theryangeary/www/deployments",
    "created_at": "2025-09-20T17:47:10Z",
    "updated_at": "2025-11-03T13:47:26Z",
    "pushed_at": "2025-11-03T13:47:22Z",
    "git_url": "git://github.com/theryangeary/www.git",
    "ssh_url": "git@github.com:theryangeary/www.git",
    "clone_url": "https://github.com/theryangeary/www.git",
    "svn_url": "https://github.com/theryangeary/www",
    "homepage": null,
    "size": 226,
    "stargazers_count": 0,
    "watchers_count": 0,
    "language": "Rust",
    "has_issues": true,
    "has_projects": true,
    "has_downloads": true,
    "has_wiki": true,
    "has_pages": false,
    "has_discussions": false,
    "forks_count": 0,
    "mirror_url": null,
    "archived": false,
    "disabled": false,
    "open_issues_count": 0,
    "license": null,
    "allow_forking": true,
    "is_template": false,
    "web_commit_signoff_required": false,
    "topics": [

    ],
    "visibility": "public",
    "forks": 0,
    "open_issues": 0,
    "watchers": 0,
    "default_branch": "main"
  },
  "sender": {
    "login": "theryangeary",
    "id": 7076013,
    "node_id": "MDQ6VXNlcjcwNzYwMTM=",
    "avatar_url": "https://avatars.githubusercontent.com/u/7076013?v=4",
    "gravatar_id": "",
    "url": "https://api.github.com/users/theryangeary",
    "html_url": "https://github.com/theryangeary",
    "followers_url": "https://api.github.com/users/theryangeary/followers",
    "following_url": "https://api.github.com/users/theryangeary/following{/other_user}",
    "gists_url": "https://api.github.com/users/theryangeary/gists{/gist_id}",
    "starred_url": "https://api.github.com/users/theryangeary/starred{/owner}{/repo}",
    "subscriptions_url": "https://api.github.com/users/theryangeary/subscriptions",
    "organizations_url": "https://api.github.com/users/theryangeary/orgs",
    "repos_url": "https://api.github.com/users/theryangeary/repos",
    "events_url": "https://api.github.com/users/theryangeary/events{/privacy}",
    "received_events_url": "https://api.github.com/users/theryangeary/received_events",
    "type": "User",
    "user_view_type": "public",
    "site_admin": false
  }
}"#;

        let payload: WebhookPayload =
            serde_json::from_str(&json_data).expect("Failed to deserialize WebhookPayload");
        assert_eq!(payload.repository.full_name, "theryangeary/www");
        assert_eq!(
            payload
                .package
                .unwrap()
                .package_version
                .container_metadata
                .unwrap()
                .tag
                .name,
            "0e2a89b"
        );
    }
}
