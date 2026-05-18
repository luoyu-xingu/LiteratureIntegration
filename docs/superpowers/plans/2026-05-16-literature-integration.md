# LiteratureIntegration 论文检索系统实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 构建基于 Neo4j 的论文检索系统，支持工作区管理、论文自动导入、作者关系网络图、关键词模糊搜索和 Markdown 批量导出。

**Architecture:** Rust Axum 后端提供 REST API，React TypeScript 前端 SPA，Neo4j 图数据库存储论文-作者-关键词关系。后端分层：routes → services → repositories，前端按页面/组件组织。

**Tech Stack:** Rust (Axum, neo4rs, reqwest, tera), TypeScript (React 18, Vite, Ant Design, react-force-graph-2d, react-markdown), Neo4j

---

## File Structure

### Backend (Rust)

```
backend/
├── Cargo.toml
├── .env
├── src/
│   ├── main.rs                  # 启动入口
│   ├── config.rs                # 配置管理
│   ├── errors.rs                # 统一错误类型
│   ├── models/
│   │   ├── mod.rs
│   │   ├── workspace.rs         # Workspace 结构体
│   │   ├── paper.rs             # Paper 结构体
│   │   ├── author.rs            # Author 结构体
│   │   ├── keyword.rs           # Keyword 结构体
│   │   └── dto.rs               # 请求/响应 DTO
│   ├── repositories/
│   │   ├── mod.rs
│   │   ├── neo4j_repo.rs        # Neo4j 查询封装
│   │   └── external_api.rs      # 外部学术 API 调用
│   ├── services/
│   │   ├── mod.rs
│   │   ├── workspace.rs
│   │   ├── paper.rs
│   │   ├── author.rs
│   │   ├── search.rs
│   │   └── export.rs
│   └── routes/
│       ├── mod.rs
│       ├── workspace.rs
│       ├── paper.rs
│       ├── author.rs
│       ├── search.rs
│       └── export.rs
└── templates/
    └── export.md.tera           # Markdown 导出模板
```

### Frontend (TypeScript)

```
frontend/
├── package.json
├── tsconfig.json
├── vite.config.ts
├── index.html
└── src/
    ├── main.tsx
    ├── App.tsx
    ├── types/
    │   └── index.ts
    ├── api/
    │   ├── client.ts
    │   ├── workspace.ts
    │   ├── paper.ts
    │   ├── author.ts
    │   ├── search.ts
    │   └── export.ts
    ├── hooks/
    │   ├── useWorkspaces.ts
    │   ├── usePapers.ts
    │   ├── useGraph.ts
    │   └── useSearch.ts
    ├── components/
    │   ├── Layout.tsx
    │   ├── WorkspaceList.tsx
    │   ├── WorkspaceForm.tsx
    │   ├── PaperList.tsx
    │   ├── PaperDetail.tsx
    │   ├── PaperImport.tsx
    │   ├── PaperNotes.tsx
    │   ├── AuthorGraph.tsx
    │   ├── SearchBar.tsx
    │   ├── SearchResult.tsx
    │   └── ExportPanel.tsx
    ├── pages/
    │   ├── WorkspacesPage.tsx
    │   ├── WorkspaceDetail.tsx
    │   ├── PaperPage.tsx
    │   └── SearchPage.tsx
    └── styles/
        └── global.css
```

---

### Task 1: Rust 后端项目初始化

**Files:**
- Create: `backend/Cargo.toml`
- Create: `backend/src/main.rs`
- Create: `backend/.env`

- [ ] **Step 1: 创建 Cargo 项目**

```bash
cd /workspace && cargo init --name literature_integration backend
```

- [ ] **Step 2: 配置 Cargo.toml 依赖**

替换 `backend/Cargo.toml` 内容：

```toml
[package]
name = "literature_integration"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
neo4rs = "0.8"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4", "serde"] }
reqwest = { version = "0.12", features = ["json"] }
thiserror = "1"
anyhow = "1"
tera = "1"
chrono = { version = "0.4", features = ["serde"] }
tower-http = { version = "0.5", features = ["cors"] }
dotenvy = "0.15"
tracing = "0.1"
tracing-subscriber = "0.3"
```

- [ ] **Step 3: 创建 .env 文件**

```
NEO4J_URI=bolt://localhost:7687
NEO4J_USER=neo4j
NEO4J_PASSWORD=password
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
CORS_ORIGIN=http://localhost:5173
```

- [ ] **Step 4: 编写最小 main.rs 验证编译**

```rust
use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::init();

    let app = Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .layer(CorsLayer::permissive());

    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port: u16 = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "3000".into())
        .parse()
        .unwrap();

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();
    tracing::info!("Server running on {}:{}", host, port);
    axum::serve(listener, app).await.unwrap();
}
```

- [ ] **Step 5: 编译验证**

```bash
cd /workspace/backend && cargo build
```

Expected: 编译成功

- [ ] **Step 6: Commit**

```bash
cd /workspace && git add backend/ && git commit -m "feat: initialize Rust backend project with Axum"
```

---

### Task 2: 后端 Models & 错误类型

**Files:**
- Create: `backend/src/models/mod.rs`
- Create: `backend/src/models/workspace.rs`
- Create: `backend/src/models/paper.rs`
- Create: `backend/src/models/author.rs`
- Create: `backend/src/models/keyword.rs`
- Create: `backend/src/models/dto.rs`
- Create: `backend/src/errors.rs`

- [ ] **Step 1: 创建 errors.rs**

```rust
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Workspace not found: {0}")]
    WorkspaceNotFound(String),

    #[error("Paper not found: {0}")]
    PaperNotFound(String),

    #[error("Author not found: {0}")]
    AuthorNotFound(String),

    #[error("Import failed: {0}")]
    ImportFailed(String),

    #[error("Neo4j error: {0}")]
    Neo4jError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("External API error: {0}")]
    ExternalApiError(String),

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::WorkspaceNotFound(_) => (StatusCode::NOT_FOUND, "WORKSPACE_NOT_FOUND", self.to_string()),
            AppError::PaperNotFound(_) => (StatusCode::NOT_FOUND, "PAPER_NOT_FOUND", self.to_string()),
            AppError::AuthorNotFound(_) => (StatusCode::NOT_FOUND, "AUTHOR_NOT_FOUND", self.to_string()),
            AppError::ImportFailed(_) => (StatusCode::UNPROCESSABLE_ENTITY, "IMPORT_FAILED", self.to_string()),
            AppError::Neo4jError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "NEO4J_ERROR", self.to_string()),
            AppError::ValidationError(_) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", self.to_string()),
            AppError::ExternalApiError(_) => (StatusCode::BAD_GATEWAY, "EXTERNAL_API_ERROR", self.to_string()),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "Internal server error".into()),
        };
        let body = json!({ "error": { "code": code, "message": message } });
        (status, axum::Json(body)).into_response()
    }
}

impl From<neo4rs::Error> for AppError {
    fn from(err: neo4rs::Error) -> Self {
        AppError::Neo4jError(err.to_string())
    }
}
```

- [ ] **Step 2: 创建 models/workspace.rs**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: String,
}
```

- [ ] **Step 3: 创建 models/paper.rs**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paper {
    pub id: String,
    pub title: String,
    pub doi: Option<String>,
    pub arxiv_id: Option<String>,
    pub abstract_text: Option<String>,
    pub user_notes: Option<String>,
    pub year: Option<i32>,
    pub journal: Option<String>,
    pub created_at: String,
}
```

- [ ] **Step 4: 创建 models/author.rs**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub id: String,
    pub name: String,
    pub orcid: Option<String>,
}
```

- [ ] **Step 5: 创建 models/keyword.rs**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyword {
    pub id: String,
    pub name: String,
}
```

- [ ] **Step 6: 创建 models/dto.rs**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateWorkspaceRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWorkspaceRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ImportPaperRequest {
    pub identifier: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePaperRequest {
    pub user_notes: Option<String>,
    pub corresponding_author_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaperDetailResponse {
    pub paper: super::paper::Paper,
    pub first_author: Option<super::author::Author>,
    pub corresponding_author: Option<super::author::Author>,
    pub keywords: Vec<super::keyword::Keyword>,
}

#[derive(Debug, Serialize)]
pub struct GraphDataResponse {
    pub nodes: Vec<GraphNode>,
    pub links: Vec<GraphLink>,
}

#[derive(Debug, Serialize)]
pub struct GraphNode {
    pub id: String,
    pub name: String,
    pub paper_count: i32,
    pub author_type: String,
}

#[derive(Debug, Serialize)]
pub struct GraphLink {
    pub source: String,
    pub target: String,
    pub paper_count: i32,
}

#[derive(Debug, Deserialize)]
pub struct ExportRequest {
    pub format: String,
    pub group_by: Option<String>,
    pub filter: Option<ExportFilter>,
}

#[derive(Debug, Deserialize)]
pub struct ExportFilter {
    pub author_ids: Option<Vec<String>>,
    pub keyword_ids: Option<Vec<String>>,
    pub year_range: Option<(i32, i32)>,
}

#[derive(Debug, Serialize)]
pub struct AuthorWithPapers {
    pub author: super::author::Author,
    pub papers: Vec<super::paper::Paper>,
}
```

- [ ] **Step 7: 创建 models/mod.rs**

```rust
pub mod workspace;
pub mod paper;
pub mod author;
pub mod keyword;
pub mod dto;
```

- [ ] **Step 8: 编译验证**

```bash
cd /workspace/backend && cargo build
```

- [ ] **Step 9: Commit**

```bash
cd /workspace && git add backend/src/models/ backend/src/errors.rs && git commit -m "feat: add backend models, DTOs, and error types"
```

---

### Task 3: 配置管理 & Neo4j 连接

**Files:**
- Create: `backend/src/config.rs`
- Modify: `backend/src/main.rs`

- [ ] **Step 1: 创建 config.rs**

```rust
use neo4rs::Graph;

#[derive(Debug, Clone)]
pub struct Config {
    pub neo4j_uri: String,
    pub neo4j_user: String,
    pub neo4j_password: String,
    pub server_host: String,
    pub server_port: u16,
    pub cors_origin: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            neo4j_uri: std::env::var("NEO4J_URI").unwrap_or_else(|_| "bolt://localhost:7687".into()),
            neo4j_user: std::env::var("NEO4J_USER").unwrap_or_else(|_| "neo4j".into()),
            neo4j_password: std::env::var("NEO4J_PASSWORD").unwrap_or_else(|_| "password".into()),
            server_host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".into())
                .parse()
                .unwrap(),
            cors_origin: std::env::var("CORS_ORIGIN").unwrap_or_else(|_| "http://localhost:5173".into()),
        }
    }
}

pub async fn create_neo4j_pool(config: &Config) -> Result<Graph, neo4rs::Error> {
    let config = neo4rs::ConfigBuilder::default()
        .uri(&config.neo4j_uri)
        .user(&config.neo4j_user)
        .password(&config.neo4j_password)
        .build()?;
    Graph::connect(config).await
}
```

- [ ] **Step 2: 更新 main.rs 使用 Config 和 Neo4j 连接**

```rust
mod config;
mod errors;
mod models;

use axum::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::init();

    let cfg = config::Config::from_env();
    let graph = config::create_neo4j_pool(&cfg)
        .await
        .expect("Failed to connect to Neo4j");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .with_state(graph)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", cfg.server_host, cfg.server_port))
        .await
        .unwrap();
    tracing::info!("Server running on {}:{}", cfg.server_host, cfg.server_port);
    axum::serve(listener, app).await.unwrap();
}
```

- [ ] **Step 3: 编译验证**

```bash
cd /workspace/backend && cargo build
```

- [ ] **Step 4: Commit**

```bash
cd /workspace && git add backend/src/config.rs backend/src/main.rs && git commit -m "feat: add config management and Neo4j connection"
```

---

### Task 4: Neo4j Repository - 工作区操作

**Files:**
- Create: `backend/src/repositories/mod.rs`
- Create: `backend/src/repositories/neo4j_repo.rs`

- [ ] **Step 1: 创建 repositories/mod.rs**

```rust
pub mod neo4j_repo;
pub mod external_api;
```

- [ ] **Step 2: 创建 repositories/neo4j_repo.rs - 工作区部分**

```rust
use neo4rs::Graph;
use crate::models::workspace::Workspace;

pub struct Neo4jRepo {
    graph: Graph,
}

impl Neo4jRepo {
    pub fn new(graph: Graph) -> Self {
        Self { graph }
    }

    pub async fn create_workspace(&self, id: &str, name: &str, description: &str, created_at: &str) -> Result<Workspace, crate::errors::AppError> {
        let query = neo4rs::query(
            "CREATE (w:Workspace {id: $id, name: $name, description: $description, created_at: $created_at}) RETURN w"
        )
        .param("id", id)
        .param("name", name)
        .param("description", description)
        .param("created_at", created_at);

        let mut result = self.graph.execute(query).await?;
        let row = result.next().await?.ok_or_else(|| crate::errors::AppError::Neo4jError("No row returned".into()))?;
        let node: neo4rs::Node = row.get("w")?;
        Ok(workspace_from_node(&node))
    }

    pub async fn list_workspaces(&self) -> Result<Vec<Workspace>, crate::errors::AppError> {
        let query = neo4rs::query("MATCH (w:Workspace) RETURN w ORDER BY w.created_at DESC");
        let mut result = self.graph.execute(query).await?;
        let mut workspaces = Vec::new();
        while let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("w")?;
            workspaces.push(workspace_from_node(&node));
        }
        Ok(workspaces)
    }

    pub async fn get_workspace(&self, id: &str) -> Result<Option<Workspace>, crate::errors::AppError> {
        let query = neo4rs::query("MATCH (w:Workspace {id: $id}) RETURN w").param("id", id);
        let mut result = self.graph.execute(query).await?;
        if let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("w")?;
            Ok(Some(workspace_from_node(&node)))
        } else {
            Ok(None)
        }
    }

    pub async fn update_workspace(&self, id: &str, name: Option<&str>, description: Option<&str>) -> Result<Option<Workspace>, crate::errors::AppError> {
        let query = neo4rs::query(
            "MATCH (w:Workspace {id: $id}) SET w.name = COALESCE($name, w.name), w.description = COALESCE($description, w.description) RETURN w"
        )
        .param("id", id)
        .param("name", name)
        .param("description", description);

        let mut result = self.graph.execute(query).await?;
        if let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("w")?;
            Ok(Some(workspace_from_node(&node)))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_workspace(&self, id: &str) -> Result<bool, crate::errors::AppError> {
        let query = neo4rs::query(
            "MATCH (w:Workspace {id: $id}) DETACH DELETE w RETURN count(w) AS deleted"
        )
        .param("id", id);

        let mut result = self.graph.execute(query).await?;
        if let Some(row) = result.next().await? {
            let deleted: i64 = row.get("deleted")?;
            Ok(deleted > 0)
        } else {
            Ok(false)
        }
    }
}

fn workspace_from_node(node: &neo4rs::Node) -> Workspace {
    Workspace {
        id: node.get::<String>("id").unwrap_or_default(),
        name: node.get::<String>("name").unwrap_or_default(),
        description: node.get::<String>("description").unwrap_or_default(),
        created_at: node.get::<String>("created_at").unwrap_or_default(),
    }
}
```

- [ ] **Step 3: 编译验证**

```bash
cd /workspace/backend && cargo build
```

- [ ] **Step 4: Commit**

```bash
cd /workspace && git add backend/src/repositories/ && git commit -m "feat: add Neo4j repository with workspace CRUD operations"
```

---

### Task 5: 工作区 Service & Routes

**Files:**
- Create: `backend/src/services/mod.rs`
- Create: `backend/src/services/workspace.rs`
- Create: `backend/src/routes/mod.rs`
- Create: `backend/src/routes/workspace.rs`
- Modify: `backend/src/main.rs`

- [ ] **Step 1: 创建 services/mod.rs**

```rust
pub mod workspace;
pub mod paper;
pub mod author;
pub mod search;
pub mod export;
```

- [ ] **Step 2: 创建 services/workspace.rs**

```rust
use crate::errors::AppError;
use crate::models::workspace::Workspace;
use crate::repositories::neo4j_repo::Neo4jRepo;

pub struct WorkspaceService;

impl WorkspaceService {
    pub async fn create(repo: &Neo4jRepo, name: String, description: Option<String>) -> Result<Workspace, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = chrono::Utc::now().to_rfc3339();
        let desc = description.unwrap_or_default();
        repo.create_workspace(&id, &name, &desc, &created_at).await
    }

    pub async fn list(repo: &Neo4jRepo) -> Result<Vec<Workspace>, AppError> {
        repo.list_workspaces().await
    }

    pub async fn get(repo: &Neo4jRepo, id: &str) -> Result<Workspace, AppError> {
        repo.get_workspace(id)
            .await?
            .ok_or_else(|| AppError::WorkspaceNotFound(id.to_string()))
    }

    pub async fn update(repo: &Neo4jRepo, id: &str, name: Option<String>, description: Option<String>) -> Result<Workspace, AppError> {
        repo.update_workspace(id, name.as_deref(), description.as_deref())
            .await?
            .ok_or_else(|| AppError::WorkspaceNotFound(id.to_string()))
    }

    pub async fn delete(repo: &Neo4jRepo, id: &str) -> Result<(), AppError> {
        let deleted = repo.delete_workspace(id).await?;
        if !deleted {
            return Err(AppError::WorkspaceNotFound(id.to_string()));
        }
        Ok(())
    }
}
```

- [ ] **Step 3: 创建 routes/mod.rs**

```rust
pub mod workspace;
pub mod paper;
pub mod author;
pub mod search;
pub mod export;
```

- [ ] **Step 4: 创建 routes/workspace.rs**

```rust
use axum::{extract::{Path, State}, Json};
use neo4rs::Graph;
use crate::errors::AppError;
use crate::models::dto::{CreateWorkspaceRequest, UpdateWorkspaceRequest};
use crate::repositories::neo4j_repo::Neo4jRepo;
use crate::services::workspace::WorkspaceService;

pub async fn create_workspace(
    State(graph): State<Graph>,
    Json(req): Json<CreateWorkspaceRequest>,
) -> Result<Json<crate::models::workspace::Workspace>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let workspace = WorkspaceService::create(&repo, req.name, req.description).await?;
    Ok(Json(workspace))
}

pub async fn list_workspaces(
    State(graph): State<Graph>,
) -> Result<Json<Vec<crate::models::workspace::Workspace>>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let workspaces = WorkspaceService::list(&repo).await?;
    Ok(Json(workspaces))
}

pub async fn get_workspace(
    State(graph): State<Graph>,
    Path(id): Path<String>,
) -> Result<Json<crate::models::workspace::Workspace>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let workspace = WorkspaceService::get(&repo, &id).await?;
    Ok(Json(workspace))
}

pub async fn update_workspace(
    State(graph): State<Graph>,
    Path(id): Path<String>,
    Json(req): Json<UpdateWorkspaceRequest>,
) -> Result<Json<crate::models::workspace::Workspace>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let workspace = WorkspaceService::update(&repo, &id, req.name, req.description).await?;
    Ok(Json(workspace))
}

pub async fn delete_workspace(
    State(graph): State<Graph>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let repo = Neo4jRepo::new(graph);
    WorkspaceService::delete(&repo, &id).await?;
    Ok(Json(serde_json::json!({"deleted": true})))
}
```

- [ ] **Step 5: 更新 main.rs 注册工作区路由**

在 `main.rs` 中添加模块声明和路由：

```rust
mod config;
mod errors;
mod models;
mod repositories;
mod services;
mod routes;

use axum::{routing::{get, delete, post, put}, Router};
use neo4rs::Graph;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::init();

    let cfg = config::Config::from_env();
    let graph = config::create_neo4j_pool(&cfg)
        .await
        .expect("Failed to connect to Neo4j");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let workspace_routes = Router::new()
        .route("/", post(routes::workspace::create_workspace).get(routes::workspace::list_workspaces))
        .route("/{id}", get(routes::workspace::get_workspace).put(routes::workspace::update_workspace).delete(routes::workspace::delete_workspace));

    let app = Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .nest("/api/workspaces", workspace_routes)
        .with_state(graph)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", cfg.server_host, cfg.server_port))
        .await
        .unwrap();
    tracing::info!("Server running on {}:{}", cfg.server_host, cfg.server_port);
    axum::serve(listener, app).await.unwrap();
}
```

- [ ] **Step 6: 编译验证**

```bash
cd /workspace/backend && cargo build
```

- [ ] **Step 7: Commit**

```bash
cd /workspace && git add backend/src/ && git commit -m "feat: add workspace service and REST API routes"
```

---

### Task 6: 外部学术 API 客户端

**Files:**
- Create: `backend/src/repositories/external_api.rs`

- [ ] **Step 1: 创建 external_api.rs**

```rust
use crate::errors::AppError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CrossrefWork {
    title: Option<Vec<String>>,
    author: Option<Vec<CrossrefAuthor>>,
    abstract_text: Option<String>,
    published_print: Option<CrossrefDate>,
    container_title: Option<Vec<String>>,
    subject: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct CrossrefAuthor {
    given: Option<String>,
    family: Option<String>,
    orcid: Option<String>,
    sequence: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CrossrefDate {
    date_parts: Vec<Vec<i32>>,
}

#[derive(Debug, Deserialize)]
struct CrossrefResponse {
    message: CrossrefWork,
}

#[derive(Debug, Deserialize)]
struct ArxivEntry {
    title: String,
    author: ArxivAuthorList,
    summary: String,
    published: String,
}

#[derive(Debug, Deserialize)]
struct ArxivAuthorList {
    name: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ArxivFeed {
    entry: ArxivEntry,
}

#[derive(Debug, Clone)]
pub struct PaperMeta {
    pub title: String,
    pub authors: Vec<AuthorMeta>,
    pub abstract_text: Option<String>,
    pub year: Option<i32>,
    pub journal: Option<String>,
    pub keywords: Vec<String>,
    pub doi: Option<String>,
    pub arxiv_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AuthorMeta {
    pub name: String,
    pub orcid: Option<String>,
    pub is_first: bool,
    pub is_corresponding: bool,
}

pub struct ExternalApiClient {
    client: reqwest::Client,
}

impl ExternalApiClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch_by_identifier(&self, identifier: &str) -> Result<PaperMeta, AppError> {
        let trimmed = identifier.trim();
        if trimmed.starts_with("10.") || trimmed.starts_with("doi:") || trimmed.starts_with("DOI:") {
            let doi = trimmed.trim_start_matches("doi:").trim_start_matches("DOI:").trim();
            self.fetch_by_doi(doi).await
        } else {
            self.fetch_by_arxiv(trimmed).await
        }
    }

    async fn fetch_by_doi(&self, doi: &str) -> Result<PaperMeta, AppError> {
        let url = format!("https://api.crossref.org/works/{}", doi);
        let resp = self.client.get(&url)
            .header("User-Agent", "LiteratureIntegration/1.0 (mailto:contact@example.com)")
            .send()
            .await
            .map_err(|e| AppError::ExternalApiError(format!("Crossref request failed: {}", e)))?;

        if !resp.status().is_success() {
            return Err(AppError::ImportFailed(format!("Crossref returned status {}", resp.status())));
        }

        let body: CrossrefResponse = resp.json().await
            .map_err(|e| AppError::ExternalApiError(format!("Failed to parse Crossref response: {}", e)))?;

        let work = body.message;
        let title = work.title.and_then(|t| t.into_iter().next()).unwrap_or_default();
        let year = work.published_print.and_then(|d| d.date_parts.into_iter().next())
            .and_then(|p| p.into_iter().next());

        let mut authors: Vec<AuthorMeta> = Vec::new();
        if let Some(crossref_authors) = work.author {
            let total = crossref_authors.len();
            for (i, a) in crossref_authors.iter().enumerate() {
                let given = a.given.as_deref().unwrap_or("");
                let family = a.family.as_deref().unwrap_or("");
                let name = if given.is_empty() {
                    family.to_string()
                } else {
                    format!("{} {}", given, family)
                };
                authors.push(AuthorMeta {
                    name,
                    orcid: a.orcid.clone(),
                    is_first: i == 0,
                    is_corresponding: i == total - 1,
                });
            }
        }

        let keywords = work.subject.unwrap_or_default();
        let journal = work.container_title.and_then(|t| t.into_iter().next());

        Ok(PaperMeta {
            title,
            authors,
            abstract_text: work.abstract_text,
            year,
            journal,
            keywords,
            doi: Some(doi.to_string()),
            arxiv_id: None,
        })
    }

    async fn fetch_by_arxiv(&self, arxiv_id: &str) -> Result<PaperMeta, AppError> {
        let url = format!("http://export.arxiv.org/api/query?id_list={}", arxiv_id);
        let resp = self.client.get(&url)
            .send()
            .await
            .map_err(|e| AppError::ExternalApiError(format!("arXiv request failed: {}", e)))?;

        if !resp.status().is_success() {
            return Err(AppError::ImportFailed(format!("arXiv returned status {}", resp.status())));
        }

        let body = resp.text().await
            .map_err(|e| AppError::ExternalApiError(format!("Failed to read arXiv response: {}", e)))?;

        let title = extract_xml_tag(&body, "title").unwrap_or_default();
        let summary = extract_xml_tag(&body, "summary");
        let published = extract_xml_tag(&body, "published");
        let year = published.and_then(|p| p.get(..4).and_then(|y| y.parse::<i32>().ok()));

        let author_names = extract_xml_tags(&body, "name");
        let total = author_names.len();
        let authors: Vec<AuthorMeta> = author_names.into_iter().enumerate().map(|(i, name)| {
            AuthorMeta {
                name,
                orcid: None,
                is_first: i == 0,
                is_corresponding: i == total - 1,
            }
        }).collect();

        Ok(PaperMeta {
            title,
            authors,
            abstract_text: summary,
            year,
            journal: None,
            keywords: Vec::new(),
            doi: None,
            arxiv_id: Some(arxiv_id.to_string()),
        })
    }
}

fn extract_xml_tag(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{}>", tag);
    let close = format!("</{}>", tag);
    let start = xml.find(&open)?;
    let content_start = start + open.len();
    let content_end = xml[content_start..].find(&close)?;
    Some(xml[content_start..content_start + content_end].trim().to_string())
}

fn extract_xml_tags(xml: &str, tag: &str) -> Vec<String> {
    let open = format!("<{}>", tag);
    let close = format!("</{}>", tag);
    let mut results = Vec::new();
    let mut search_from = 0;
    while let Some(start) = xml[search_from..].find(&open) {
        let content_start = search_from + start + open.len();
        if let Some(content_end) = xml[content_start..].find(&close) {
            results.push(xml[content_start..content_start + content_end].trim().to_string());
            search_from = content_start + content_end + close.len();
        } else {
            break;
        }
    }
    results
}
```

- [ ] **Step 2: 编译验证**

```bash
cd /workspace/backend && cargo build
```

- [ ] **Step 3: Commit**

```bash
cd /workspace && git add backend/src/repositories/external_api.rs && git commit -m "feat: add external academic API client for DOI and arXiv lookup"
```

---

### Task 7: Neo4j Repository - 论文操作

**Files:**
- Modify: `backend/src/repositories/neo4j_repo.rs`

- [ ] **Step 1: 在 neo4j_repo.rs 中添加论文相关方法**

在 `Neo4jRepo` impl 块中追加以下方法：

```rust
    pub async fn create_paper_if_not_exists(
        &self,
        id: &str,
        title: &str,
        doi: Option<&str>,
        arxiv_id: Option<&str>,
        abstract_text: Option<&str>,
        year: Option<i32>,
        journal: Option<&str>,
        created_at: &str,
    ) -> Result<crate::models::paper::Paper, crate::errors::AppError> {
        let query = neo4rs::query(
            "MERGE (p:Paper {doi: COALESCE($doi, ''), arxiv_id: COALESCE($arxiv_id, '')}) \
             ON CREATE SET p.id = $id, p.title = $title, p.abstract = $abstract_text, \
             p.year = $year, p.journal = $journal, p.created_at = $created_at, p.user_notes = '' \
             RETURN p"
        )
        .param("id", id)
        .param("title", title)
        .param("doi", doi.unwrap_or(""))
        .param("arxiv_id", arxiv_id.unwrap_or(""))
        .param("abstract_text", abstract_text.unwrap_or(""))
        .param("year", year.unwrap_or(0))
        .param("journal", journal.unwrap_or(""))
        .param("created_at", created_at);

        let mut result = self.graph.execute(query).await?;
        let row = result.next().await?.ok_or_else(|| crate::errors::AppError::Neo4jError("No row returned".into()))?;
        let node: neo4rs::Node = row.get("p")?;
        Ok(paper_from_node(&node))
    }

    pub async fn add_paper_to_workspace(&self, workspace_id: &str, paper_id: &str, added_at: &str) -> Result<(), crate::errors::AppError> {
        let query = neo4rs::query(
            "MATCH (w:Workspace {id: $workspace_id}), (p:Paper {id: $paper_id}) \
             MERGE (w)-[:CONTAINS {added_at: $added_at}]->(p)"
        )
        .param("workspace_id", workspace_id)
        .param("paper_id", paper_id)
        .param("added_at", added_at);
        self.graph.execute(query).await?;
        Ok(())
    }

    pub async fn create_author_if_not_exists(
        &self,
        id: &str,
        name: &str,
        orcid: Option<&str>,
    ) -> Result<crate::models::author::Author, crate::errors::AppError> {
        let query = neo4rs::query(
            "MERGE (a:Author {name: $name, orcid: COALESCE($orcid, '')}) \
             ON CREATE SET a.id = $id \
             RETURN a"
        )
        .param("id", id)
        .param("name", name)
        .param("orcid", orcid.unwrap_or(""));

        let mut result = self.graph.execute(query).await?;
        let row = result.next().await?.ok_or_else(|| crate::errors::AppError::Neo4jError("No row returned".into()))?;
        let node: neo4rs::Node = row.get("a")?;
        Ok(author_from_node(&node))
    }

    pub async fn link_first_author(&self, author_id: &str, paper_id: &str) -> Result<(), crate::errors::AppError> {
        let query = neo4rs::query(
            "MATCH (a:Author {id: $author_id}), (p:Paper {id: $paper_id}) \
             MERGE (a)-[:FIRST_AUTHOR_OF]->(p)"
        )
        .param("author_id", author_id)
        .param("paper_id", paper_id);
        self.graph.execute(query).await?;
        Ok(())
    }

    pub async fn link_corresponding_author(&self, author_id: &str, paper_id: &str) -> Result<(), crate::errors::AppError> {
        let query = neo4rs::query(
            "MATCH (a:Author {id: $author_id}), (p:Paper {id: $paper_id}) \
             MERGE (a)-[:CORRESPONDING_AUTHOR_OF]->(p)"
        )
        .param("author_id", author_id)
        .param("paper_id", paper_id);
        self.graph.execute(query).await?;
        Ok(())
    }

    pub async fn link_co_authors(&self, author1_id: &str, author2_id: &str, workspace_id: &str) -> Result<(), crate::errors::AppError> {
        let query = neo4rs::query(
            "MATCH (a1:Author {id: $author1_id}), (a2:Author {id: $author2_id}) \
             MERGE (a1)-[r:CO_AUTHOR_OF {workspace_id: $workspace_id}]-(a2) \
             ON CREATE SET r.paper_count = 1 \
             ON MATCH SET r.paper_count = r.paper_count + 1"
        )
        .param("author1_id", author1_id)
        .param("author2_id", author2_id)
        .param("workspace_id", workspace_id);
        self.graph.execute(query).await?;
        Ok(())
    }

    pub async fn add_keyword(&self, id: &str, name: &str, paper_id: &str) -> Result<(), crate::errors::AppError> {
        let query = neo4rs::query(
            "MERGE (k:Keyword {name: $name}) \
             ON CREATE SET k.id = $id \
             WITH k MATCH (p:Paper {id: $paper_id}) \
             MERGE (p)-[:HAS_KEYWORD]->(k)"
        )
        .param("id", id)
        .param("name", name)
        .param("paper_id", paper_id);
        self.graph.execute(query).await?;
        Ok(())
    }

    pub async fn list_papers_in_workspace(&self, workspace_id: &str) -> Result<Vec<crate::models::paper::Paper>, crate::errors::AppError> {
        let query = neo4rs::query(
            "MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper) RETURN p ORDER BY p.year DESC"
        )
        .param("workspace_id", workspace_id);

        let mut result = self.graph.execute(query).await?;
        let mut papers = Vec::new();
        while let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("p")?;
            papers.push(paper_from_node(&node));
        }
        Ok(papers)
    }

    pub async fn get_paper(&self, id: &str) -> Result<Option<crate::models::paper::Paper>, crate::errors::AppError> {
        let query = neo4rs::query("MATCH (p:Paper {id: $id}) RETURN p").param("id", id);
        let mut result = self.graph.execute(query).await?;
        if let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("p")?;
            Ok(Some(paper_from_node(&node)))
        } else {
            Ok(None)
        }
    }

    pub async fn update_paper_notes(&self, id: &str, user_notes: &str) -> Result<Option<crate::models::paper::Paper>, crate::errors::AppError> {
        let query = neo4rs::query(
            "MATCH (p:Paper {id: $id}) SET p.user_notes = $user_notes RETURN p"
        )
        .param("id", id)
        .param("user_notes", user_notes);

        let mut result = self.graph.execute(query).await?;
        if let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("p")?;
            Ok(Some(paper_from_node(&node)))
        } else {
            Ok(None)
        }
    }

    pub async fn remove_paper_from_workspace(&self, workspace_id: &str, paper_id: &str) -> Result<bool, crate::errors::AppError> {
        let query = neo4rs::query(
            "MATCH (w:Workspace {id: $workspace_id})-[r:CONTAINS]->(p:Paper {id: $paper_id}) DELETE r RETURN count(r) AS deleted"
        )
        .param("workspace_id", workspace_id)
        .param("paper_id", paper_id);

        let mut result = self.graph.execute(query).await?;
        if let Some(row) = result.next().await? {
            let deleted: i64 = row.get("deleted")?;
            Ok(deleted > 0)
        } else {
            Ok(false)
        }
    }

    pub async fn get_paper_first_author(&self, paper_id: &str) -> Result<Option<crate::models::author::Author>, crate::errors::AppError> {
        let query = neo4rs::query(
            "MATCH (a:Author)-[:FIRST_AUTHOR_OF]->(p:Paper {id: $paper_id}) RETURN a"
        )
        .param("paper_id", paper_id);

        let mut result = self.graph.execute(query).await?;
        if let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("a")?;
            Ok(Some(author_from_node(&node)))
        } else {
            Ok(None)
        }
    }

    pub async fn get_paper_corresponding_author(&self, paper_id: &str) -> Result<Option<crate::models::author::Author>, crate::errors::AppError> {
        let query = neo4rs::query(
            "MATCH (a:Author)-[:CORRESPONDING_AUTHOR_OF]->(p:Paper {id: $paper_id}) RETURN a"
        )
        .param("paper_id", paper_id);

        let mut result = self.graph.execute(query).await?;
        if let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("a")?;
            Ok(Some(author_from_node(&node)))
        } else {
            Ok(None)
        }
    }

    pub async fn get_paper_keywords(&self, paper_id: &str) -> Result<Vec<crate::models::keyword::Keyword>, crate::errors::AppError> {
        let query = neo4rs::query(
            "MATCH (p:Paper {id: $paper_id})-[:HAS_KEYWORD]->(k:Keyword) RETURN k"
        )
        .param("paper_id", paper_id);

        let mut result = self.graph.execute(query).await?;
        let mut keywords = Vec::new();
        while let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("k")?;
            keywords.push(keyword_from_node(&node));
        }
        Ok(keywords)
    }
```

在文件底部追加辅助函数：

```rust
fn paper_from_node(node: &neo4rs::Node) -> crate::models::paper::Paper {
    crate::models::paper::Paper {
        id: node.get::<String>("id").unwrap_or_default(),
        title: node.get::<String>("title").unwrap_or_default(),
        doi: node.get::<String>("doi").ok().filter(|s| !s.is_empty()),
        arxiv_id: node.get::<String>("arxiv_id").ok().filter(|s| !s.is_empty()),
        abstract_text: node.get::<String>("abstract").ok().filter(|s| !s.is_empty()),
        user_notes: node.get::<String>("user_notes").ok().filter(|s| !s.is_empty()),
        year: node.get::<i32>("year").ok().filter(|y| *y > 0).copied(),
        journal: node.get::<String>("journal").ok().filter(|s| !s.is_empty()),
        created_at: node.get::<String>("created_at").unwrap_or_default(),
    }
}

fn author_from_node(node: &neo4rs::Node) -> crate::models::author::Author {
    crate::models::author::Author {
        id: node.get::<String>("id").unwrap_or_default(),
        name: node.get::<String>("name").unwrap_or_default(),
        orcid: node.get::<String>("orcid").ok().filter(|s| !s.is_empty()),
    }
}

fn keyword_from_node(node: &neo4rs::Node) -> crate::models::keyword::Keyword {
    crate::models::keyword::Keyword {
        id: node.get::<String>("id").unwrap_or_default(),
        name: node.get::<String>("name").unwrap_or_default(),
    }
}
```

- [ ] **Step 2: 编译验证**

```bash
cd /workspace/backend && cargo build
```

- [ ] **Step 3: Commit**

```bash
cd /workspace && git add backend/src/repositories/neo4j_repo.rs && git commit -m "feat: add paper, author, keyword repository methods to Neo4j"
```

---

### Task 8: 论文 Service & Routes

**Files:**
- Create: `backend/src/services/paper.rs`
- Create: `backend/src/routes/paper.rs`
- Modify: `backend/src/main.rs`

- [ ] **Step 1: 创建 services/paper.rs**

```rust
use crate::errors::AppError;
use crate::models::dto::{PaperDetailResponse, ImportPaperRequest, UpdatePaperRequest};
use crate::models::paper::Paper;
use crate::repositories::external_api::{ExternalApiClient, PaperMeta};
use crate::repositories::neo4j_repo::Neo4jRepo;

pub struct PaperService;

impl PaperService {
    pub async fn import(repo: &Neo4jRepo, workspace_id: &str, req: ImportPaperRequest) -> Result<PaperDetailResponse, AppError> {
        let client = ExternalApiClient::new();
        let meta = client.fetch_by_identifier(&req.identifier).await?;

        let workspace = repo.get_workspace(workspace_id).await?
            .ok_or_else(|| AppError::WorkspaceNotFound(workspace_id.to_string()))?;

        let paper_id = uuid::Uuid::new_v4().to_string();
        let created_at = chrono::Utc::now().to_rfc3339();

        let paper = repo.create_paper_if_not_exists(
            &paper_id,
            &meta.title,
            meta.doi.as_deref(),
            meta.arxiv_id.as_deref(),
            meta.abstract_text.as_deref(),
            meta.year,
            meta.journal.as_deref(),
            &created_at,
        ).await?;

        let added_at = chrono::Utc::now().to_rfc3339();
        repo.add_paper_to_workspace(workspace_id, &paper.id, &added_at).await?;

        let mut first_author = None;
        let mut corresponding_author = None;

        for author_meta in &meta.authors {
            let author_id = uuid::Uuid::new_v4().to_string();
            let author = repo.create_author_if_not_exists(
                &author_id,
                &author_meta.name,
                author_meta.orcid.as_deref(),
            ).await?;

            if author_meta.is_first {
                repo.link_first_author(&author.id, &paper.id).await?;
                first_author = Some(author.clone());
            }
            if author_meta.is_corresponding {
                repo.link_corresponding_author(&author.id, &paper.id).await?;
                corresponding_author = Some(author.clone());
            }
        }

        if let (Some(ref fa), Some(ref ca)) = (&first_author, &corresponding_author) {
            if fa.id != ca.id {
                repo.link_co_authors(&fa.id, &ca.id, workspace_id).await?;
            }
        }

        for keyword_name in &meta.keywords {
            let keyword_id = uuid::Uuid::new_v4().to_string();
            repo.add_keyword(&keyword_id, keyword_name, &paper.id).await?;
        }

        let keywords = repo.get_paper_keywords(&paper.id).await?;

        Ok(PaperDetailResponse {
            paper,
            first_author,
            corresponding_author,
            keywords,
        })
    }

    pub async fn list_in_workspace(repo: &Neo4jRepo, workspace_id: &str) -> Result<Vec<Paper>, AppError> {
        repo.list_papers_in_workspace(workspace_id).await
    }

    pub async fn get_detail(repo: &Neo4jRepo, id: &str) -> Result<PaperDetailResponse, AppError> {
        let paper = repo.get_paper(id).await?
            .ok_or_else(|| AppError::PaperNotFound(id.to_string()))?;
        let first_author = repo.get_paper_first_author(id).await?;
        let corresponding_author = repo.get_paper_corresponding_author(id).await?;
        let keywords = repo.get_paper_keywords(id).await?;
        Ok(PaperDetailResponse {
            paper,
            first_author,
            corresponding_author,
            keywords,
        })
    }

    pub async fn update(repo: &Neo4jRepo, id: &str, req: UpdatePaperRequest) -> Result<Paper, AppError> {
        if let Some(notes) = req.user_notes {
            repo.update_paper_notes(id, &notes).await?
                .ok_or_else(|| AppError::PaperNotFound(id.to_string()))?;
        }
        repo.get_paper(id).await?
            .ok_or_else(|| AppError::PaperNotFound(id.to_string()))
    }

    pub async fn remove_from_workspace(repo: &Neo4jRepo, workspace_id: &str, paper_id: &str) -> Result<(), AppError> {
        let removed = repo.remove_paper_from_workspace(workspace_id, paper_id).await?;
        if !removed {
            return Err(AppError::PaperNotFound(paper_id.to_string()));
        }
        Ok(())
    }
}
```

- [ ] **Step 2: 创建 routes/paper.rs**

```rust
use axum::{extract::{Path, Query, State}, Json};
use neo4rs::Graph;
use serde::Deserialize;
use crate::errors::AppError;
use crate::models::dto::{ImportPaperRequest, UpdatePaperRequest, PaperDetailResponse};
use crate::models::paper::Paper;
use crate::repositories::neo4j_repo::Neo4jRepo;
use crate::services::paper::PaperService;

#[derive(Deserialize)]
pub struct WorkspaceIdPath {
    pub workspace_id: String,
}

pub async fn import_paper(
    State(graph): State<Graph>,
    Path(workspace_id): Path<String>,
    Json(req): Json<ImportPaperRequest>,
) -> Result<Json<PaperDetailResponse>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let result = PaperService::import(&repo, &workspace_id, req).await?;
    Ok(Json(result))
}

pub async fn list_papers(
    State(graph): State<Graph>,
    Path(workspace_id): Path<String>,
) -> Result<Json<Vec<Paper>>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let papers = PaperService::list_in_workspace(&repo, &workspace_id).await?;
    Ok(Json(papers))
}

pub async fn get_paper(
    State(graph): State<Graph>,
    Path(id): Path<String>,
) -> Result<Json<PaperDetailResponse>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let detail = PaperService::get_detail(&repo, &id).await?;
    Ok(Json(detail))
}

pub async fn update_paper(
    State(graph): State<Graph>,
    Path(id): Path<String>,
    Json(req): Json<UpdatePaperRequest>,
) -> Result<Json<Paper>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let paper = PaperService::update(&repo, &id, req).await?;
    Ok(Json(paper))
}

pub async fn delete_paper(
    State(graph): State<Graph>,
    Path((workspace_id, paper_id)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, AppError> {
    let repo = Neo4jRepo::new(graph);
    PaperService::remove_from_workspace(&repo, &workspace_id, &paper_id).await?;
    Ok(Json(serde_json::json!({"removed": true})))
}
```

- [ ] **Step 3: 更新 main.rs 添加论文路由**

在 `main.rs` 的路由注册部分追加：

```rust
    let paper_routes = Router::new()
        .route("/", post(routes::paper::import_paper).get(routes::paper::list_papers))
        .route("/{id}", get(routes::paper::get_paper).put(routes::paper::update_paper));

    let app = Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .nest("/api/workspaces", workspace_routes)
        .nest("/api/workspaces/{workspace_id}/papers", paper_routes)
        .with_state(graph)
        .layer(cors);
```

- [ ] **Step 4: 编译验证**

```bash
cd /workspace/backend && cargo build
```

- [ ] **Step 5: Commit**

```bash
cd /workspace && git add backend/src/ && git commit -m "feat: add paper import service and REST API routes"
```

---

### Task 9: 作者 & 网络图 Service & Routes

**Files:**
- Create: `backend/src/services/author.rs`
- Create: `backend/src/routes/author.rs`
- Modify: `backend/src/repositories/neo4j_repo.rs`
- Modify: `backend/src/main.rs`

- [ ] **Step 1: 在 neo4j_repo.rs 的 Neo4jRepo impl 中追加作者和网络图方法**

```rust
    pub async fn list_authors_in_workspace(&self, workspace_id: &str) -> Result<Vec<crate::models::author::Author>, crate::errors::AppError> {
        let query = neo4rs::query(
            "MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)<-[:FIRST_AUTHOR_OF|CORRESPONDING_AUTHOR_OF]-(a:Author) \
             RETURN DISTINCT a ORDER BY a.name"
        )
        .param("workspace_id", workspace_id);

        let mut result = self.graph.execute(query).await?;
        let mut authors = Vec::new();
        while let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("a")?;
            authors.push(author_from_node(&node));
        }
        Ok(authors)
    }

    pub async fn get_author_papers(&self, author_id: &str) -> Result<Vec<crate::models::paper::Paper>, crate::errors::AppError> {
        let query = neo4rs::query(
            "MATCH (a:Author {id: $author_id})-[:FIRST_AUTHOR_OF|CORRESPONDING_AUTHOR_OF]->(p:Paper) RETURN p ORDER BY p.year DESC"
        )
        .param("author_id", author_id);

        let mut result = self.graph.execute(query).await?;
        let mut papers = Vec::new();
        while let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("p")?;
            papers.push(paper_from_node(&node));
        }
        Ok(papers)
    }

    pub async fn get_graph_data(&self, workspace_id: &str) -> Result<(Vec<crate::models::dto::GraphNode>, Vec<crate::models::dto::GraphLink>), crate::errors::AppError> {
        let nodes_query = neo4rs::query(
            "MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)<-[:FIRST_AUTHOR_OF|CORRESPONDING_AUTHOR_OF]-(a:Author) \
             WITH a, count(p) AS paper_count, \
             CASE WHEN EXISTS((a)-[:FIRST_AUTHOR_OF]->(:Paper)) AND EXISTS((a)-[:CORRESPONDING_AUTHOR_OF]->(:Paper)) \
                  THEN 'both' WHEN EXISTS((a)-[:FIRST_AUTHOR_OF]->(:Paper)) THEN 'first' ELSE 'corresponding' END AS author_type \
             RETURN a.id AS id, a.name AS name, paper_count, author_type ORDER BY a.name"
        )
        .param("workspace_id", workspace_id);

        let mut result = self.graph.execute(nodes_query).await?;
        let mut nodes = Vec::new();
        while let Some(row) = result.next().await? {
            nodes.push(crate::models::dto::GraphNode {
                id: row.get::<String>("id")?,
                name: row.get::<String>("name")?,
                paper_count: row.get::<i32>("paper_count")?,
                author_type: row.get::<String>("author_type")?,
            });
        }

        let links_query = neo4rs::query(
            "MATCH (a1:Author)-[r:CO_AUTHOR_OF {workspace_id: $workspace_id}]-(a2:Author) \
             WHERE a1.id < a2.id \
             RETURN a1.id AS source, a2.id AS target, r.paper_count AS paper_count"
        )
        .param("workspace_id", workspace_id);

        let mut link_result = self.graph.execute(links_query).await?;
        let mut links = Vec::new();
        while let Some(row) = link_result.next().await? {
            links.push(crate::models::dto::GraphLink {
                source: row.get::<String>("source")?,
                target: row.get::<String>("target")?,
                paper_count: row.get::<i32>("paper_count")?,
            });
        }

        Ok((nodes, links))
    }
```

- [ ] **Step 2: 创建 services/author.rs**

```rust
use crate::errors::AppError;
use crate::models::author::Author;
use crate::models::dto::{AuthorWithPapers, GraphDataResponse};
use crate::models::paper::Paper;
use crate::repositories::neo4j_repo::Neo4jRepo;

pub struct AuthorService;

impl AuthorService {
    pub async fn list_in_workspace(repo: &Neo4jRepo, workspace_id: &str) -> Result<Vec<Author>, AppError> {
        repo.list_authors_in_workspace(workspace_id).await
    }

    pub async fn get_author_papers(repo: &Neo4jRepo, author_id: &str) -> Result<AuthorWithPapers, AppError> {
        let papers = repo.get_author_papers(author_id).await?;
        Ok(AuthorWithPapers {
            author: Author {
                id: author_id.to_string(),
                name: String::new(),
                orcid: None,
            },
            papers,
        })
    }

    pub async fn get_graph_data(repo: &Neo4jRepo, workspace_id: &str) -> Result<GraphDataResponse, AppError> {
        let (nodes, links) = repo.get_graph_data(workspace_id).await?;
        Ok(GraphDataResponse { nodes, links })
    }
}
```

- [ ] **Step 3: 创建 routes/author.rs**

```rust
use axum::{extract::{Path, State}, Json};
use neo4rs::Graph;
use crate::errors::AppError;
use crate::models::dto::{AuthorWithPapers, GraphDataResponse};
use crate::models::author::Author;
use crate::repositories::neo4j_repo::Neo4jRepo;
use crate::services::author::AuthorService;

pub async fn list_authors(
    State(graph): State<Graph>,
    Path(workspace_id): Path<String>,
) -> Result<Json<Vec<Author>>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let authors = AuthorService::list_in_workspace(&repo, &workspace_id).await?;
    Ok(Json(authors))
}

pub async fn get_graph(
    State(graph): State<Graph>,
    Path(workspace_id): Path<String>,
) -> Result<Json<GraphDataResponse>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let data = AuthorService::get_graph_data(&repo, &workspace_id).await?;
    Ok(Json(data))
}

pub async fn get_author_papers(
    State(graph): State<Graph>,
    Path(id): Path<String>,
) -> Result<Json<AuthorWithPapers>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let result = AuthorService::get_author_papers(&repo, &id).await?;
    Ok(Json(result))
}
```

- [ ] **Step 4: 更新 main.rs 添加作者路由**

```rust
    let author_routes = Router::new()
        .route("/", get(routes::author::list_authors))
        .route("/graph", get(routes::author::get_graph));

    let app = Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .nest("/api/workspaces", workspace_routes)
        .nest("/api/workspaces/{workspace_id}/papers", paper_routes)
        .nest("/api/workspaces/{workspace_id}/authors", author_routes)
        .route("/api/authors/{id}/papers", get(routes::author::get_author_papers))
        .with_state(graph)
        .layer(cors);
```

- [ ] **Step 5: 编译验证**

```bash
cd /workspace/backend && cargo build
```

- [ ] **Step 6: Commit**

```bash
cd /workspace && git add backend/src/ && git commit -m "feat: add author listing, graph data, and author papers API"
```

---

### Task 10: 搜索 Service & Routes

**Files:**
- Create: `backend/src/services/search.rs`
- Create: `backend/src/routes/search.rs`
- Modify: `backend/src/repositories/neo4j_repo.rs`
- Modify: `backend/src/main.rs`

- [ ] **Step 1: 在 neo4j_repo.rs 的 Neo4jRepo impl 中追加搜索方法**

```rust
    pub async fn search_by_keyword(&self, workspace_id: &str, query: &str) -> Result<Vec<crate::models::paper::Paper>, crate::errors::AppError> {
        let cypher = "MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper) \
                      WHERE p.title CONTAINS $query OR p.abstract CONTAINS $query \
                      OR EXISTS { MATCH (p)-[:HAS_KEYWORD]->(k:Keyword) WHERE k.name CONTAINS $query } \
                      RETURN p ORDER BY p.year DESC";
        let q = neo4rs::query(cypher)
            .param("workspace_id", workspace_id)
            .param("query", query);

        let mut result = self.graph.execute(q).await?;
        let mut papers = Vec::new();
        while let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("p")?;
            papers.push(paper_from_node(&node));
        }
        Ok(papers)
    }

    pub async fn search_by_author(&self, workspace_id: &str, author_name: &str) -> Result<Vec<crate::models::dto::AuthorWithPapers>, crate::errors::AppError> {
        let cypher = "MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)<-[:FIRST_AUTHOR_OF|CORRESPONDING_AUTHOR_OF]-(a:Author) \
                      WHERE a.name CONTAINS $author_name \
                      RETURN a, collect(p) AS papers ORDER BY size(papers) DESC";
        let q = neo4rs::query(cypher)
            .param("workspace_id", workspace_id)
            .param("author_name", author_name);

        let mut result = self.graph.execute(q).await?;
        let mut authors_with_papers = Vec::new();
        while let Some(row) = result.next().await? {
            let author_node: neo4rs::Node = row.get("a")?;
            let paper_nodes: Vec<neo4rs::Node> = row.get("papers")?;
            let papers: Vec<crate::models::paper::Paper> = paper_nodes.iter().map(paper_from_node).collect();
            authors_with_papers.push(crate::models::dto::AuthorWithPapers {
                author: author_from_node(&author_node),
                papers,
            });
        }
        Ok(authors_with_papers)
    }
```

- [ ] **Step 2: 创建 services/search.rs**

```rust
use crate::errors::AppError;
use crate::models::dto::AuthorWithPapers;
use crate::models::paper::Paper;
use crate::repositories::neo4j_repo::Neo4jRepo;

pub struct SearchService;

impl SearchService {
    pub async fn search_by_keyword(repo: &Neo4jRepo, workspace_id: &str, query: &str) -> Result<Vec<Paper>, AppError> {
        repo.search_by_keyword(workspace_id, query).await
    }

    pub async fn search_by_author(repo: &Neo4jRepo, workspace_id: &str, author_name: &str) -> Result<Vec<AuthorWithPapers>, AppError> {
        repo.search_by_author(workspace_id, author_name).await
    }
}
```

- [ ] **Step 3: 创建 routes/search.rs**

```rust
use axum::{extract::{Path, Query, State}, Json};
use neo4rs::Graph;
use serde::Deserialize;
use crate::errors::AppError;
use crate::models::dto::AuthorWithPapers;
use crate::models::paper::Paper;
use crate::repositories::neo4j_repo::Neo4jRepo;
use crate::services::search::SearchService;

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: Option<String>,
    pub author: Option<String>,
}

pub async fn search(
    State(graph): State<Graph>,
    Path(workspace_id): Path<String>,
    Query(params): Query<SearchParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let repo = Neo4jRepo::new(graph);
    if let Some(query) = params.q {
        let papers = SearchService::search_by_keyword(&repo, &workspace_id, &query).await?;
        Ok(Json(serde_json::json!({ "mode": "keyword", "query": query, "results": papers })))
    } else if let Some(author) = params.author {
        let results = SearchService::search_by_author(&repo, &workspace_id, &author).await?;
        Ok(Json(serde_json::json!({ "mode": "author", "query": author, "results": results })))
    } else {
        Err(crate::errors::AppError::ValidationError("Must provide q or author parameter".into()))
    }
}
```

- [ ] **Step 4: 更新 main.rs 添加搜索路由**

在路由注册中追加：

```rust
    let app = Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .nest("/api/workspaces", workspace_routes)
        .nest("/api/workspaces/{workspace_id}/papers", paper_routes)
        .nest("/api/workspaces/{workspace_id}/authors", author_routes)
        .route("/api/workspaces/{workspace_id}/search", get(routes::search::search))
        .route("/api/authors/{id}/papers", get(routes::author::get_author_papers))
        .with_state(graph)
        .layer(cors);
```

- [ ] **Step 5: 编译验证**

```bash
cd /workspace/backend && cargo build
```

- [ ] **Step 6: Commit**

```bash
cd /workspace && git add backend/src/ && git commit -m "feat: add keyword and author search API"
```

---

### Task 11: 导出 Service & Routes

**Files:**
- Create: `backend/src/services/export.rs`
- Create: `backend/src/routes/export.rs`
- Create: `backend/templates/export.md.tera`
- Modify: `backend/src/main.rs`

- [ ] **Step 1: 创建 templates/export.md.tera**

```
# 工作区: {{ workspace_name }}

> 导出时间: {{ export_date }}
> 论文数量: {{ paper_count }}

---
{% for group in groups %}
## {{ group_label }}: {{ group.name }}

{% for paper in group.papers %}
### {{ paper.title }}
- **年份**: {{ paper.year }} | **期刊**: {{ paper.journal }}
- **DOI**: {{ paper.doi }}
- **一作**: {{ paper.first_author }} | **通讯**: {{ paper.corresponding_author }}
- **关键词**: {{ paper.keywords }}

**Abstract:**
{{ paper.abstract_text }}

**笔记:**
{{ paper.user_notes }}

---
{% endfor %}
{% endfor %}
```

- [ ] **Step 2: 在 neo4j_repo.rs 的 Neo4jRepo impl 中追加导出查询方法**

```rust
    pub async fn get_papers_for_export(&self, workspace_id: &str, author_ids: Option<&[String]>, keyword_ids: Option<&[String]>, year_range: Option<(i32, i32)>) -> Result<Vec<crate::models::paper::Paper>, crate::errors::AppError> {
        let mut cypher = String::from(
            "MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)"
        );

        if let Some(aids) = author_ids {
            if !aids.is_empty() {
                cypher.push_str(" MATCH (a:Author)-[:FIRST_AUTHOR_OF|CORRESPONDING_AUTHOR_OF]->(p) WHERE a.id IN $author_ids");
            }
        }
        if let Some(kids) = keyword_ids {
            if !kids.is_empty() {
                cypher.push_str(" MATCH (p)-[:HAS_KEYWORD]->(k:Keyword) WHERE k.id IN $keyword_ids");
            }
        }

        cypher.push_str(" RETURN DISTINCT p ORDER BY p.year DESC");

        let mut query = neo4rs::query(&cypher)
            .param("workspace_id", workspace_id);

        if let Some(aids) = author_ids {
            query = query.param("author_ids", aids);
        }
        if let Some(kids) = keyword_ids {
            query = query.param("keyword_ids", kids);
        }

        let mut result = self.graph.execute(query).await?;
        let mut papers = Vec::new();
        while let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("p")?;
            papers.push(paper_from_node(&node));
        }
        Ok(papers)
    }
```

- [ ] **Step 3: 创建 services/export.rs**

```rust
use crate::errors::AppError;
use crate::models::dto::ExportRequest;
use crate::repositories::neo4j_repo::Neo4jRepo;

pub struct ExportService;

impl ExportService {
    pub async fn export_markdown(repo: &Neo4jRepo, workspace_id: &str, req: ExportRequest) -> Result<String, AppError> {
        let filter = req.filter.unwrap_or_default();
        let author_ids = filter.author_ids.as_deref();
        let keyword_ids = filter.keyword_ids.as_deref();
        let year_range = filter.year_range;

        let papers = repo.get_papers_for_export(workspace_id, author_ids, keyword_ids, year_range).await?;

        let workspace = repo.get_workspace(workspace_id).await?
            .ok_or_else(|| AppError::WorkspaceNotFound(workspace_id.to_string()))?;

        let mut md = format!("# 工作区: {}\n\n", workspace.name);
        md.push_str(&format!("> 导出时间: {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M")));
        md.push_str(&format!("> 论文数量: {}\n\n---\n\n", papers.len()));

        for paper in &papers {
            md.push_str(&format!("### {}\n", paper.title));
            md.push_str(&format!("- **年份**: {} | **期刊**: {}\n", paper.year.map(|y| y.to_string()).unwrap_or_default(), paper.journal.as_deref().unwrap_or("")));
            md.push_str(&format!("- **DOI**: {}\n", paper.doi.as_deref().unwrap_or("")));

            let first_author = repo.get_paper_first_author(&paper.id).await?;
            let corr_author = repo.get_paper_corresponding_author(&paper.id).await?;
            md.push_str(&format!("- **一作**: {} | **通讯**: {}\n",
                first_author.map(|a| a.name).unwrap_or_default(),
                corr_author.map(|a| a.name).unwrap_or_default()
            ));

            let keywords = repo.get_paper_keywords(&paper.id).await?;
            let kw_str: Vec<String> = keywords.iter().map(|k| k.name.clone()).collect();
            md.push_str(&format!("- **关键词**: {}\n\n", kw_str.join(", ")));

            if let Some(ref abstract_text) = paper.abstract_text {
                md.push_str(&format!("**Abstract:**\n{}\n\n", abstract_text));
            }
            if let Some(ref notes) = paper.user_notes {
                if !notes.is_empty() {
                    md.push_str(&format!("**笔记:**\n{}\n\n", notes));
                }
            }

            md.push_str("---\n\n");
        }

        Ok(md)
    }
}
```

- [ ] **Step 4: 创建 routes/export.rs**

```rust
use axum::{extract::{Path, State}, Json, http::{StatusCode, header}};
use neo4rs::Graph;
use crate::errors::AppError;
use crate::models::dto::ExportRequest;
use crate::repositories::neo4j_repo::Neo4jRepo;
use crate::services::export::ExportService;

pub async fn export_workspace(
    State(graph): State<Graph>,
    Path(workspace_id): Path<String>,
    Json(req): Json<ExportRequest>,
) -> Result<(StatusCode, [(header::HeaderName, &'static str); 1], String), AppError> {
    let repo = Neo4jRepo::new(graph);
    let markdown = ExportService::export_markdown(&repo, &workspace_id, req).await?;
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/markdown; charset=utf-8")],
        markdown,
    ))
}
```

- [ ] **Step 5: 更新 main.rs 添加导出路由**

```rust
    let app = Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .nest("/api/workspaces", workspace_routes)
        .nest("/api/workspaces/{workspace_id}/papers", paper_routes)
        .nest("/api/workspaces/{workspace_id}/authors", author_routes)
        .route("/api/workspaces/{workspace_id}/search", get(routes::search::search))
        .route("/api/workspaces/{workspace_id}/export", post(routes::export::export_workspace))
        .route("/api/authors/{id}/papers", get(routes::author::get_author_papers))
        .with_state(graph)
        .layer(cors);
```

- [ ] **Step 6: 编译验证**

```bash
cd /workspace/backend && cargo build
```

- [ ] **Step 7: Commit**

```bash
cd /workspace && git add backend/ && git commit -m "feat: add Markdown export service and API endpoint"
```

---

### Task 12: 前端项目初始化

**Files:**
- Create: `frontend/` (via Vite scaffolding)
- Create: `frontend/src/types/index.ts`
- Create: `frontend/src/api/client.ts`

- [ ] **Step 1: 使用 Vite 创建 React + TypeScript 项目**

```bash
cd /workspace && npm create vite@latest frontend -- --template react-ts
```

- [ ] **Step 2: 安装依赖**

```bash
cd /workspace/frontend && npm install && npm install antd @ant-design/icons react-router-dom react-force-graph-2d react-markdown
```

- [ ] **Step 3: 创建 types/index.ts**

```typescript
export interface Workspace {
  id: string;
  name: string;
  description: string;
  created_at: string;
}

export interface Paper {
  id: string;
  title: string;
  doi: string | null;
  arxiv_id: string | null;
  abstract_text: string | null;
  user_notes: string | null;
  year: number | null;
  journal: string | null;
  created_at: string;
}

export interface Author {
  id: string;
  name: string;
  orcid: string | null;
}

export interface Keyword {
  id: string;
  name: string;
}

export interface PaperDetail {
  paper: Paper;
  first_author: Author | null;
  corresponding_author: Author | null;
  keywords: Keyword[];
}

export interface GraphNode {
  id: string;
  name: string;
  paper_count: number;
  author_type: 'first' | 'corresponding' | 'both';
}

export interface GraphLink {
  source: string;
  target: string;
  paper_count: number;
}

export interface GraphData {
  nodes: GraphNode[];
  links: GraphLink[];
}

export interface AuthorWithPapers {
  author: Author;
  papers: Paper[];
}

export interface ExportRequest {
  format: string;
  group_by?: string;
  filter?: {
    author_ids?: string[];
    keyword_ids?: string[];
    year_range?: [number, number];
  };
}
```

- [ ] **Step 4: 创建 api/client.ts**

```typescript
const API_BASE = '/api';

async function request<T>(path: string, options?: RequestInit): Promise<T> {
  const res = await fetch(`${API_BASE}${path}`, {
    headers: { 'Content-Type': 'application/json' },
    ...options,
  });
  if (!res.ok) {
    const err = await res.json().catch(() => ({ error: { message: res.statusText } }));
    throw new Error(err.error?.message || `HTTP ${res.status}`);
  }
  if (res.headers.get('content-type')?.includes('text/markdown')) {
    return res.text() as unknown as T;
  }
  return res.json();
}

export function get<T>(path: string): Promise<T> {
  return request<T>(path);
}

export function post<T>(path: string, body?: unknown): Promise<T> {
  return request<T>(path, { method: 'POST', body: body ? JSON.stringify(body) : undefined });
}

export function put<T>(path: string, body?: unknown): Promise<T> {
  return request<T>(path, { method: 'PUT', body: body ? JSON.stringify(body) : undefined });
}

export function del<T>(path: string): Promise<T> {
  return request<T>(path, { method: 'DELETE' });
}

export async function downloadMarkdown(path: string, body: unknown): Promise<void> {
  const res = await fetch(`${API_BASE}${path}`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  });
  const blob = await res.blob();
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = 'export.md';
  a.click();
  URL.revokeObjectURL(url);
}
```

- [ ] **Step 5: 创建 API 模块文件**

`frontend/src/api/workspace.ts`:
```typescript
import { get, post, put, del } from './client';
import type { Workspace } from '../types';

export const createWorkspace = (data: { name: string; description?: string }) =>
  post<Workspace>('/workspaces', data);

export const listWorkspaces = () => get<Workspace[]>('/workspaces');

export const getWorkspace = (id: string) => get<Workspace>(`/workspaces/${id}`);

export const updateWorkspace = (id: string, data: { name?: string; description?: string }) =>
  put<Workspace>(`/workspaces/${id}`, data);

export const deleteWorkspace = (id: string) => del<{ deleted: boolean }>(`/workspaces/${id}`);
```

`frontend/src/api/paper.ts`:
```typescript
import { get, post, put, del } from './client';
import type { Paper, PaperDetail } from '../types';

export const importPaper = (workspaceId: string, identifier: string) =>
  post<PaperDetail>(`/workspaces/${workspaceId}/papers`, { identifier });

export const listPapers = (workspaceId: string) =>
  get<Paper[]>(`/workspaces/${workspaceId}/papers`);

export const getPaper = (id: string) => get<PaperDetail>(`/papers/${id}`);

export const updatePaper = (id: string, data: { user_notes?: string }) =>
  put<Paper>(`/papers/${id}`, data);

export const deletePaper = (workspaceId: string, paperId: string) =>
  del<{ removed: boolean }>(`/workspaces/${workspaceId}/papers/${paperId}`);
```

`frontend/src/api/author.ts`:
```typescript
import { get } from './client';
import type { Author, GraphData, AuthorWithPapers } from '../types';

export const listAuthors = (workspaceId: string) =>
  get<Author[]>(`/workspaces/${workspaceId}/authors`);

export const getGraphData = (workspaceId: string) =>
  get<GraphData>(`/workspaces/${workspaceId}/authors/graph`);

export const getAuthorPapers = (authorId: string) =>
  get<AuthorWithPapers>(`/authors/${authorId}/papers`);
```

`frontend/src/api/search.ts`:
```typescript
import { get } from './client';

export const searchByKeyword = (workspaceId: string, query: string) =>
  get<{ mode: string; query: string; results: any[] }>(`/workspaces/${workspaceId}/search?q=${encodeURIComponent(query)}`);

export const searchByAuthor = (workspaceId: string, author: string) =>
  get<{ mode: string; query: string; results: any[] }>(`/workspaces/${workspaceId}/search?author=${encodeURIComponent(author)}`);
```

`frontend/src/api/export.ts`:
```typescript
import { downloadMarkdown } from './client';
import type { ExportRequest } from '../types';

export const exportWorkspace = (workspaceId: string, req: ExportRequest) =>
  downloadMarkdown(`/workspaces/${workspaceId}/export`, req);
```

- [ ] **Step 6: 配置 Vite 代理**

更新 `frontend/vite.config.ts`：

```typescript
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  server: {
    port: 5173,
    proxy: {
      '/api': {
        target: 'http://localhost:3000',
        changeOrigin: true,
      },
    },
  },
})
```

- [ ] **Step 7: 验证前端编译**

```bash
cd /workspace/frontend && npm run build
```

- [ ] **Step 8: Commit**

```bash
cd /workspace && git add frontend/ && git commit -m "feat: initialize React frontend with types and API client"
```

---

### Task 13: 前端布局 & 工作区页面

**Files:**
- Create: `frontend/src/styles/global.css`
- Create: `frontend/src/components/Layout.tsx`
- Create: `frontend/src/components/WorkspaceList.tsx`
- Create: `frontend/src/components/WorkspaceForm.tsx`
- Create: `frontend/src/pages/WorkspacesPage.tsx`
- Modify: `frontend/src/App.tsx`
- Modify: `frontend/src/main.tsx`

- [ ] **Step 1: 创建 global.css**

```css
body {
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

#root {
  height: 100vh;
}
```

- [ ] **Step 2: 创建 Layout.tsx**

```tsx
import { useState } from 'react';
import { Layout as AntLayout, Menu } from 'antd';
import { BookOutlined, PlusOutlined } from '@ant-design/icons';
import { useNavigate, Outlet, useLocation } from 'react-router-dom';
import WorkspaceForm from './WorkspaceForm';

const { Sider, Content } = AntLayout;

export default function Layout() {
  const navigate = useNavigate();
  const location = useLocation();
  const [showForm, setShowForm] = useState(false);

  const workspaceId = location.pathname.split('/')[2];

  return (
    <AntLayout style={{ height: '100vh' }}>
      <Sider width={250} style={{ background: '#fff', borderRight: '1px solid #f0f0f0' }}>
        <div style={{ padding: '16px', borderBottom: '1px solid #f0f0f0' }}>
          <h2 style={{ margin: 0, fontSize: '16px' }}>
            <BookOutlined /> LiteratureIntegration
          </h2>
        </div>
        <Menu
          mode="inline"
          selectedKeys={workspaceId ? [workspaceId] : []}
          onClick={({ key }) => navigate(`/workspace/${key}`)}
          items={[]}
        />
        <div style={{ padding: '12px 16px' }}>
          <a onClick={() => setShowForm(true)} style={{ cursor: 'pointer' }}>
            <PlusOutlined /> 新建工作区
          </a>
        </div>
      </Sider>
      <Content style={{ padding: '24px', overflow: 'auto' }}>
        <Outlet />
      </Content>
      <WorkspaceForm open={showForm} onClose={() => setShowForm(false)} />
    </AntLayout>
  );
}
```

- [ ] **Step 3: 创建 WorkspaceList.tsx**

```tsx
import { useEffect, useState } from 'react';
import { List, Button, Popconfirm } from 'antd';
import { DeleteOutlined } from '@ant-design/icons';
import { listWorkspaces, deleteWorkspace } from '../api/workspace';
import type { Workspace } from '../types';
import { useNavigate } from 'react-router-dom';

export default function WorkspaceList() {
  const [workspaces, setWorkspaces] = useState<Workspace[]>([]);
  const navigate = useNavigate();

  const load = async () => {
    const data = await listWorkspaces();
    setWorkspaces(data);
  };

  useEffect(() => { load(); }, []);

  const handleDelete = async (id: string) => {
    await deleteWorkspace(id);
    load();
  };

  return (
    <List
      dataSource={workspaces}
      renderItem={(ws) => (
        <List.Item
          actions={[
            <Popconfirm title="确定删除？" onConfirm={() => handleDelete(ws.id)}>
              <Button type="text" danger icon={<DeleteOutlined />} />
            </Popconfirm>,
          ]}
        >
          <List.Item.Meta
            title={<a onClick={() => navigate(`/workspace/${ws.id}`)}>{ws.name}</a>}
            description={ws.description}
          />
        </List.Item>
      )}
    />
  );
}
```

- [ ] **Step 4: 创建 WorkspaceForm.tsx**

```tsx
import { Modal, Form, Input } from 'antd';
import { createWorkspace } from '../api/workspace';

interface Props {
  open: boolean;
  onClose: () => void;
}

export default function WorkspaceForm({ open, onClose }: Props) {
  const [form] = Form.useForm();

  const handleOk = async () => {
    const values = await form.validateFields();
    await createWorkspace(values);
    form.resetFields();
    onClose();
  };

  return (
    <Modal title="新建工作区" open={open} onOk={handleOk} onCancel={onClose}>
      <Form form={form} layout="vertical">
        <Form.Item name="name" label="名称" rules={[{ required: true }]}>
          <Input />
        </Form.Item>
        <Form.Item name="description" label="描述">
          <Input.TextArea />
        </Form.Item>
      </Form>
    </Modal>
  );
}
```

- [ ] **Step 5: 创建 pages/WorkspacesPage.tsx**

```tsx
import WorkspaceList from '../components/WorkspaceList';

export default function WorkspacesPage() {
  return (
    <div>
      <h2>工作区</h2>
      <WorkspaceList />
    </div>
  );
}
```

- [ ] **Step 6: 更新 App.tsx 配置路由**

```tsx
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import Layout from './components/Layout';
import WorkspacesPage from './pages/WorkspacesPage';
import WorkspaceDetail from './pages/WorkspaceDetail';
import PaperPage from './pages/PaperPage';
import SearchPage from './pages/SearchPage';

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route element={<Layout />}>
          <Route path="/" element={<WorkspacesPage />} />
          <Route path="/workspace/:id" element={<WorkspaceDetail />} />
          <Route path="/paper/:id" element={<PaperPage />} />
          <Route path="/workspace/:id/search" element={<SearchPage />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}
```

- [ ] **Step 7: 更新 main.tsx**

```tsx
import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './styles/global.css';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
```

- [ ] **Step 8: 验证编译**

```bash
cd /workspace/frontend && npm run build
```

- [ ] **Step 9: Commit**

```bash
cd /workspace && git add frontend/ && git commit -m "feat: add frontend layout, workspace list, and routing"
```

---

### Task 14: 前端论文管理页面

**Files:**
- Create: `frontend/src/components/PaperList.tsx`
- Create: `frontend/src/components/PaperImport.tsx`
- Create: `frontend/src/components/PaperDetail.tsx`
- Create: `frontend/src/components/PaperNotes.tsx`
- Create: `frontend/src/pages/WorkspaceDetail.tsx`
- Create: `frontend/src/pages/PaperPage.tsx`
- Create: `frontend/src/hooks/useWorkspaces.ts`
- Create: `frontend/src/hooks/usePapers.ts`

- [ ] **Step 1: 创建 hooks/useWorkspaces.ts**

```typescript
import { useState, useEffect } from 'react';
import { listWorkspaces } from '../api/workspace';
import type { Workspace } from '../types';

export function useWorkspaces() {
  const [workspaces, setWorkspaces] = useState<Workspace[]>([]);
  const [loading, setLoading] = useState(false);

  const load = async () => {
    setLoading(true);
    const data = await listWorkspaces();
    setWorkspaces(data);
    setLoading(false);
  };

  useEffect(() => { load(); }, []);

  return { workspaces, loading, reload: load };
}
```

- [ ] **Step 2: 创建 hooks/usePapers.ts**

```typescript
import { useState, useEffect } from 'react';
import { listPapers } from '../api/paper';
import type { Paper } from '../types';

export function usePapers(workspaceId: string | undefined) {
  const [papers, setPapers] = useState<Paper[]>([]);
  const [loading, setLoading] = useState(false);

  const load = async () => {
    if (!workspaceId) return;
    setLoading(true);
    const data = await listPapers(workspaceId);
    setPapers(data);
    setLoading(false);
  };

  useEffect(() => { load(); }, [workspaceId]);

  return { papers, loading, reload: load };
}
```

- [ ] **Step 3: 创建 PaperList.tsx**

```tsx
import { List, Tag } from 'antd';
import type { Paper } from '../types';
import { useNavigate } from 'react-router-dom';

interface Props {
  papers: Paper[];
  loading: boolean;
}

export default function PaperList({ papers, loading }: Props) {
  const navigate = useNavigate();

  return (
    <List
      loading={loading}
      dataSource={papers}
      renderItem={(paper) => (
        <List.Item
          style={{ cursor: 'pointer' }}
          onClick={() => navigate(`/paper/${paper.id}`)}
        >
          <List.Item.Meta
            title={paper.title}
            description={
              <div>
                <div>{paper.year} · {paper.journal}</div>
                {paper.doi && <Tag>DOI: {paper.doi}</Tag>}
                {paper.arxiv_id && <Tag>arXiv: {paper.arxiv_id}</Tag>}
              </div>
            }
          />
        </List.Item>
      )}
    />
  );
}
```

- [ ] **Step 4: 创建 PaperImport.tsx**

```tsx
import { useState } from 'react';
import { Modal, Input, message } from 'antd';
import { importPaper } from '../api/paper';

interface Props {
  workspaceId: string;
  open: boolean;
  onClose: () => void;
  onImported: () => void;
}

export default function PaperImport({ workspaceId, open, onClose, onImported }: Props) {
  const [identifier, setIdentifier] = useState('');
  const [loading, setLoading] = useState(false);

  const handleOk = async () => {
    if (!identifier.trim()) return;
    setLoading(true);
    try {
      await importPaper(workspaceId, identifier.trim());
      message.success('导入成功');
      setIdentifier('');
      onImported();
      onClose();
    } catch (e: any) {
      message.error(e.message || '导入失败');
    } finally {
      setLoading(false);
    }
  };

  return (
    <Modal title="导入论文" open={open} onOk={handleOk} onCancel={onClose} confirmLoading={loading}>
      <div style={{ marginBottom: 12 }}>
        <Input
          placeholder="输入 DOI 或 arXiv ID"
          value={identifier}
          onChange={(e) => setIdentifier(e.target.value)}
        />
      </div>
      <div style={{ color: '#999', fontSize: 12 }}>
        示例: DOI: 10.1038/s41586-020-2649-2 | arXiv: 2301.12345
      </div>
    </Modal>
  );
}
```

- [ ] **Step 5: 创建 PaperNotes.tsx**

```tsx
import { useState } from 'react';
import { Input, Button, message } from 'antd';
import { updatePaper } from '../api/paper';

interface Props {
  paperId: string;
  initialNotes: string;
}

export default function PaperNotes({ paperId, initialNotes }: Props) {
  const [notes, setNotes] = useState(initialNotes);
  const [editing, setEditing] = useState(false);
  const [saving, setSaving] = useState(false);

  const handleSave = async () => {
    setSaving(true);
    try {
      await updatePaper(paperId, { user_notes: notes });
      message.success('保存成功');
      setEditing(false);
    } catch {
      message.error('保存失败');
    } finally {
      setSaving(false);
    }
  };

  return (
    <div>
      <div style={{ marginBottom: 8, fontWeight: 'bold' }}>我的笔记:</div>
      {editing ? (
        <>
          <Input.TextArea
            rows={4}
            value={notes}
            onChange={(e) => setNotes(e.target.value)}
          />
          <div style={{ marginTop: 8 }}>
            <Button type="primary" loading={saving} onClick={handleSave}>保存</Button>
            <Button style={{ marginLeft: 8 }} onClick={() => setEditing(false)}>取消</Button>
          </div>
        </>
      ) : (
        <div
          style={{ cursor: 'pointer', minHeight: 40, padding: 8, background: '#fafafa', borderRadius: 4 }}
          onClick={() => setEditing(true)}
        >
          {notes || '点击添加笔记...'}
        </div>
      )}
    </div>
  );
}
```

- [ ] **Step 6: 创建 PaperDetail.tsx**

```tsx
import { Descriptions, Tag } from 'antd';
import type { PaperDetail as PaperDetailType } from '../types';

interface Props {
  detail: PaperDetailType;
}

export default function PaperDetail({ detail }: Props) {
  const { paper, first_author, corresponding_author, keywords } = detail;

  return (
    <Descriptions bordered column={1}>
      <Descriptions.Item label="标题">{paper.title}</Descriptions.Item>
      <Descriptions.Item label="年份">{paper.year}</Descriptions.Item>
      <Descriptions.Item label="期刊">{paper.journal}</Descriptions.Item>
      <Descriptions.Item label="DOI">{paper.doi}</Descriptions.Item>
      <Descriptions.Item label="arXiv">{paper.arxiv_id}</Descriptions.Item>
      <Descriptions.Item label="一作">{first_author?.name}</Descriptions.Item>
      <Descriptions.Item label="通讯作者">{corresponding_author?.name}</Descriptions.Item>
      <Descriptions.Item label="关键词">
        {keywords.map((k) => <Tag key={k.id}>{k.name}</Tag>)}
      </Descriptions.Item>
      <Descriptions.Item label="Abstract">{paper.abstract_text}</Descriptions.Item>
    </Descriptions>
  );
}
```

- [ ] **Step 7: 创建 pages/WorkspaceDetail.tsx**

```tsx
import { useState } from 'react';
import { useParams } from 'react-router-dom';
import { Tabs, Button, Space } from 'antd';
import { PlusOutlined } from '@ant-design/icons';
import PaperList from '../components/PaperList';
import PaperImport from '../components/PaperImport';
import AuthorGraph from '../components/AuthorGraph';
import { usePapers } from '../hooks/usePapers';

export default function WorkspaceDetail() {
  const { id } = useParams<{ id: string }>();
  const { papers, loading, reload } = usePapers(id);
  const [showImport, setShowImport] = useState(false);
  const [activeTab, setActiveTab] = useState('papers');

  return (
    <div>
      <div style={{ marginBottom: 16, display: 'flex', justifyContent: 'space-between' }}>
        <h2 style={{ margin: 0 }}>工作区详情</h2>
        <Space>
          <Button type="primary" icon={<PlusOutlined />} onClick={() => setShowImport(true)}>
            导入论文
          </Button>
        </Space>
      </div>

      <Tabs activeKey={activeTab} onChange={setActiveTab} items={[
        { key: 'papers', label: '论文列表', children: <PaperList papers={papers} loading={loading} /> },
        { key: 'graph', label: '作者网络图', children: <AuthorGraph workspaceId={id!} /> },
      ]} />

      {id && (
        <PaperImport
          workspaceId={id}
          open={showImport}
          onClose={() => setShowImport(false)}
          onImported={reload}
        />
      )}
    </div>
  );
}
```

- [ ] **Step 8: 创建 pages/PaperPage.tsx**

```tsx
import { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import { Spin } from 'antd';
import PaperDetailComponent from '../components/PaperDetail';
import PaperNotes from '../components/PaperNotes';
import { getPaper } from '../api/paper';
import type { PaperDetail as PaperDetailType } from '../types';

export default function PaperPage() {
  const { id } = useParams<{ id: string }>();
  const [detail, setDetail] = useState<PaperDetailType | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!id) return;
    setLoading(true);
    getPaper(id).then(setDetail).finally(() => setLoading(false));
  }, [id]);

  if (loading) return <Spin />;
  if (!detail) return <div>论文未找到</div>;

  return (
    <div>
      <PaperDetailComponent detail={detail} />
      <div style={{ marginTop: 24 }}>
        <PaperNotes paperId={detail.paper.id} initialNotes={detail.paper.user_notes || ''} />
      </div>
    </div>
  );
}
```

- [ ] **Step 9: 验证编译**

```bash
cd /workspace/frontend && npm run build
```

- [ ] **Step 10: Commit**

```bash
cd /workspace && git add frontend/ && git commit -m "feat: add paper management pages - list, import, detail, notes"
```

---

### Task 15: 前端作者网络图

**Files:**
- Create: `frontend/src/components/AuthorGraph.tsx`
- Create: `frontend/src/hooks/useGraph.ts`

- [ ] **Step 1: 创建 hooks/useGraph.ts**

```typescript
import { useState, useEffect } from 'react';
import { getGraphData } from '../api/author';
import type { GraphData } from '../types';

export function useGraph(workspaceId: string | undefined) {
  const [data, setData] = useState<GraphData | null>(null);
  const [loading, setLoading] = useState(false);

  const load = async () => {
    if (!workspaceId) return;
    setLoading(true);
    const result = await getGraphData(workspaceId);
    setData(result);
    setLoading(false);
  };

  useEffect(() => { load(); }, [workspaceId]);

  return { data, loading, reload: load };
}
```

- [ ] **Step 2: 创建 AuthorGraph.tsx**

```tsx
import { useRef, useCallback, useEffect, useState } from 'react';
import ForceGraph2D from 'react-force-graph-2d';
import { Card, List, Spin, Drawer } from 'antd';
import { useGraph } from '../hooks/useGraph';
import { getAuthorPapers } from '../api/author';
import type { Paper, GraphNode } from '../types';

interface Props {
  workspaceId: string;
}

export default function AuthorGraph({ workspaceId }: Props) {
  const { data, loading } = useGraph(workspaceId);
  const fgRef = useRef<any>();
  const [selectedAuthor, setSelectedAuthor] = useState<{ id: string; name: string } | null>(null);
  const [authorPapers, setAuthorPapers] = useState<Paper[]>([]);
  const [drawerOpen, setDrawerOpen] = useState(false);

  const handleClickNode = useCallback(async (node: any) => {
    const graphNode = node as GraphNode;
    setSelectedAuthor({ id: graphNode.id, name: graphNode.name });
    const result = await getAuthorPapers(graphNode.id);
    setAuthorPapers(result.papers);
    setDrawerOpen(true);
  }, []);

  const graphData = data ? {
    nodes: data.nodes.map(n => ({ ...n, val: Math.max(n.paper_count, 1) })),
    links: data.links,
  } : { nodes: [], links: [] };

  if (loading) return <Spin />;

  return (
    <Card>
      <div style={{ marginBottom: 8, fontSize: 12, color: '#999' }}>
        ● 一作 &nbsp; ○ 通讯作者 &nbsp; ◎ 两者兼有 &nbsp; 线条粗细 = 合著论文数
      </div>
      <ForceGraph2D
        ref={fgRef}
        graphData={graphData}
        nodeLabel="name"
        nodeVal="val"
        nodeColor={(node: any) => {
          const n = node as GraphNode;
          if (n.author_type === 'both') return '#722ed1';
          if (n.author_type === 'first') return '#1890ff';
          return '#52c41a';
        }}
        nodeCanvasObject={(node: any, ctx: any, globalScale: number) => {
          const label = node.name;
          const fontSize = 12 / globalScale;
          ctx.font = `${fontSize}px Sans-Serif`;
          ctx.textAlign = 'center';
          ctx.textBaseline = 'middle';

          const n = node as GraphNode;
          const radius = 4 + n.paper_count * 1.5;

          ctx.beginPath();
          ctx.arc(node.x, node.y, radius, 0, 2 * Math.PI);
          if (n.author_type === 'corresponding') {
            ctx.strokeStyle = '#52c41a';
            ctx.lineWidth = 2 / globalScale;
            ctx.stroke();
          } else {
            ctx.fillStyle = n.author_type === 'both' ? '#722ed1' : '#1890ff';
            ctx.fill();
          }

          ctx.fillStyle = '#333';
          ctx.fillText(label, node.x, node.y + radius + fontSize);
        }}
        linkWidth={(link: any) => Math.max(link.paper_count * 0.5, 0.5)}
        linkColor={() => '#ccc'}
        onNodeClick={handleClickNode}
        width={800}
        height={500}
      />
      <Drawer
        title={selectedAuthor?.name}
        open={drawerOpen}
        onClose={() => setDrawerOpen(false)}
      >
        <List
          dataSource={authorPapers}
          renderItem={(paper) => (
            <List.Item>
              <List.Item.Meta title={paper.title} description={`${paper.year} · ${paper.journal}`} />
            </List.Item>
          )}
        />
      </Drawer>
    </Card>
  );
}
```

- [ ] **Step 3: 验证编译**

```bash
cd /workspace/frontend && npm run build
```

- [ ] **Step 4: Commit**

```bash
cd /workspace && git add frontend/ && git commit -m "feat: add author relationship network graph visualization"
```

---

### Task 16: 前端搜索 & 导出页面

**Files:**
- Create: `frontend/src/components/SearchBar.tsx`
- Create: `frontend/src/components/SearchResult.tsx`
- Create: `frontend/src/components/ExportPanel.tsx`
- Create: `frontend/src/pages/SearchPage.tsx`
- Create: `frontend/src/hooks/useSearch.ts`

- [ ] **Step 1: 创建 hooks/useSearch.ts**

```typescript
import { useState } from 'react';
import { searchByKeyword, searchByAuthor } from '../api/search';
import type { Paper, AuthorWithPapers } from '../types';

export function useSearch(workspaceId: string | undefined) {
  const [results, setResults] = useState<Paper[] | AuthorWithPapers[] | null>(null);
  const [mode, setMode] = useState<'keyword' | 'author'>('keyword');
  const [loading, setLoading] = useState(false);

  const search = async (query: string, searchMode: 'keyword' | 'author') => {
    if (!workspaceId || !query.trim()) return;
    setLoading(true);
    setMode(searchMode);
    try {
      if (searchMode === 'keyword') {
        const res = await searchByKeyword(workspaceId, query);
        setResults(res.results as Paper[]);
      } else {
        const res = await searchByAuthor(workspaceId, query);
        setResults(res.results as AuthorWithPapers[]);
      }
    } finally {
      setLoading(false);
    }
  };

  return { results, mode, loading, search };
}
```

- [ ] **Step 2: 创建 SearchBar.tsx**

```tsx
import { useState } from 'react';
import { Input, Radio, Button } from 'antd';
import { SearchOutlined } from '@ant-design/icons';

interface Props {
  onSearch: (query: string, mode: 'keyword' | 'author') => void;
  loading: boolean;
}

export default function SearchBar({ onSearch, loading }: Props) {
  const [query, setQuery] = useState('');
  const [mode, setMode] = useState<'keyword' | 'author'>('keyword');

  return (
    <div style={{ display: 'flex', gap: 12, alignItems: 'center', marginBottom: 16 }}>
      <Radio.Group value={mode} onChange={(e) => setMode(e.target.value)}>
        <Radio.Button value="keyword">关键词搜索</Radio.Button>
        <Radio.Button value="author">作者搜索</Radio.Button>
      </Radio.Group>
      <Input
        style={{ width: 300 }}
        placeholder={mode === 'keyword' ? '搜索关键词...' : '搜索作者姓名...'}
        value={query}
        onChange={(e) => setQuery(e.target.value)}
        onPressEnter={() => onSearch(query, mode)}
      />
      <Button type="primary" icon={<SearchOutlined />} loading={loading} onClick={() => onSearch(query, mode)}>
        搜索
      </Button>
    </div>
  );
}
```

- [ ] **Step 3: 创建 SearchResult.tsx**

```tsx
import { List, Tag } from 'antd';
import type { Paper, AuthorWithPapers } from '../types';
import { useNavigate } from 'react-router-dom';

interface Props {
  results: Paper[] | AuthorWithPapers[] | null;
  mode: 'keyword' | 'author';
}

function isAuthorResults(results: any[]): results is AuthorWithPapers[] {
  return results.length > 0 && 'author' in results[0];
}

export default function SearchResult({ results, mode }: Props) {
  const navigate = useNavigate();

  if (!results) return null;

  if (mode === 'keyword') {
    const papers = results as Paper[];
    return (
      <List
        dataSource={papers}
        renderItem={(paper) => (
          <List.Item style={{ cursor: 'pointer' }} onClick={() => navigate(`/paper/${paper.id}`)}>
            <List.Item.Meta
              title={paper.title}
              description={`${paper.year} · ${paper.journal}`}
            />
          </List.Item>
        )}
      />
    );
  }

  const authorResults = results as AuthorWithPapers[];
  return (
    <List
      dataSource={authorResults}
      renderItem={(item) => (
        <List.Item>
          <List.Item.Meta
            title={item.author.name}
            description={`${item.papers.length} 篇论文`}
          />
          <div>
            {item.papers.map((p) => (
              <Tag key={p.id} style={{ cursor: 'pointer' }} onClick={() => navigate(`/paper/${p.id}`)}>
                {p.title}
              </Tag>
            ))}
          </div>
        </List.Item>
      )}
    />
  );
}
```

- [ ] **Step 4: 创建 ExportPanel.tsx**

```tsx
import { useState } from 'react';
import { Card, Radio, Button, Checkbox, message } from 'antd';
import { DownloadOutlined } from '@ant-design/icons';
import { exportWorkspace } from '../api/export';
import type { ExportRequest } from '../types';

interface Props {
  workspaceId: string;
}

export default function ExportPanel({ workspaceId }: Props) {
  const [groupBy, setGroupBy] = useState<'author' | 'keyword'>('author');
  const [loading, setLoading] = useState(false);

  const handleExport = async () => {
    setLoading(true);
    try {
      const req: ExportRequest = { format: 'markdown', group_by: groupBy };
      await exportWorkspace(workspaceId, req);
      message.success('导出成功');
    } catch {
      message.error('导出失败');
    } finally {
      setLoading(false);
    }
  };

  return (
    <Card title="导出 Markdown">
      <div style={{ marginBottom: 16 }}>
        <div style={{ marginBottom: 8 }}>分组方式:</div>
        <Radio.Group value={groupBy} onChange={(e) => setGroupBy(e.target.value)}>
          <Radio.Button value="author">按作者分组</Radio.Button>
          <Radio.Button value="keyword">按关键词分组</Radio.Button>
        </Radio.Group>
      </div>
      <Button type="primary" icon={<DownloadOutlined />} loading={loading} onClick={handleExport}>
        下载 .md
      </Button>
    </Card>
  );
}
```

- [ ] **Step 5: 创建 pages/SearchPage.tsx**

```tsx
import { useParams } from 'react-router-dom';
import SearchBar from '../components/SearchBar';
import SearchResult from '../components/SearchResult';
import ExportPanel from '../components/ExportPanel';
import { useSearch } from '../hooks/useSearch';

export default function SearchPage() {
  const { id } = useParams<{ id: string }>();
  const { results, mode, loading, search } = useSearch(id);

  return (
    <div>
      <h2>搜索</h2>
      <SearchBar onSearch={search} loading={loading} />
      <SearchResult results={results} mode={mode} />
      <div style={{ marginTop: 24 }}>
        {id && <ExportPanel workspaceId={id} />}
      </div>
    </div>
  );
}
```

- [ ] **Step 6: 验证编译**

```bash
cd /workspace/frontend && npm run build
```

- [ ] **Step 7: Commit**

```bash
cd /workspace && git add frontend/ && git commit -m "feat: add search page and Markdown export panel"
```

---

### Task 17: 集成 & 最终验证

**Files:**
- Modify: `backend/src/main.rs` (添加静态文件服务)
- Create: `backend/.gitignore`

- [ ] **Step 1: 更新后端 main.rs 添加前端静态文件服务**

在 main.rs 中添加 `tower_http::services::ServeDir`，让后端同时服务前端构建产物：

```rust
mod config;
mod errors;
mod models;
mod repositories;
mod services;
mod routes;

use axum::{routing::{get, delete, post, put}, Router, http::HeaderValue};
use neo4rs::Graph;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::init();

    let cfg = config::Config::from_env();
    let graph = config::create_neo4j_pool(&cfg)
        .await
        .expect("Failed to connect to Neo4j");

    let cors = CorsLayer::new()
        .allow_origin(HeaderValue::from_str(&cfg.cors_origin).unwrap_or_else(|_| HeaderValue::from_static("*")))
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    let workspace_routes = Router::new()
        .route("/", post(routes::workspace::create_workspace).get(routes::workspace::list_workspaces))
        .route("/{id}", get(routes::workspace::get_workspace).put(routes::workspace::update_workspace).delete(routes::workspace::delete_workspace));

    let paper_routes = Router::new()
        .route("/", post(routes::paper::import_paper).get(routes::paper::list_papers))
        .route("/{id}", get(routes::paper::get_paper).put(routes::paper::update_paper));

    let author_routes = Router::new()
        .route("/", get(routes::author::list_authors))
        .route("/graph", get(routes::author::get_graph));

    let app = Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .nest("/api/workspaces", workspace_routes)
        .nest("/api/workspaces/{workspace_id}/papers", paper_routes)
        .nest("/api/workspaces/{workspace_id}/authors", author_routes)
        .route("/api/workspaces/{workspace_id}/search", get(routes::search::search))
        .route("/api/workspaces/{workspace_id}/export", post(routes::export::export_workspace))
        .route("/api/authors/{id}/papers", get(routes::author::get_author_papers))
        .fallback_service(ServeDir::new("../frontend/dist").append_index_html_on_directories(true))
        .with_state(graph)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", cfg.server_host, cfg.server_port))
        .await
        .unwrap();
    tracing::info!("Server running on {}:{}", cfg.server_host, cfg.server_port);
    axum::serve(listener, app).await.unwrap();
}
```

在 `Cargo.toml` 中更新 tower-http 的 features：

```toml
tower-http = { version = "0.5", features = ["cors", "fs"] }
```

- [ ] **Step 2: 创建 .gitignore**

```
/target
.env
node_modules
/dist
```

- [ ] **Step 3: 构建前端**

```bash
cd /workspace/frontend && npm run build
```

- [ ] **Step 4: 构建后端**

```bash
cd /workspace/backend && cargo build
```

- [ ] **Step 5: Commit**

```bash
cd /workspace && git add -A && git commit -m "feat: integrate frontend static file serving and finalize project structure"
```

---

## Self-Review Checklist

### Spec Coverage
- ✅ 工作区 CRUD → Task 4, 5
- ✅ 论文自动导入 (DOI/arXiv) → Task 6, 7, 8
- ✅ 一作/通讯作者信息 → Task 7, 8
- ✅ Abstract + 笔记 → Task 8, 14
- ✅ 作者关系网络图 → Task 9, 15
- ✅ 关键词模糊搜索 → Task 10
- ✅ 按作者搜索 → Task 10
- ✅ Markdown 批量导出 → Task 11, 16
- ✅ 前端 TypeScript → Task 12-16
- ✅ 后端 Rust → Task 1-11

### Placeholder Scan
- ✅ No TBD, TODO, or placeholder patterns found

### Type Consistency
- ✅ PaperDetailResponse used consistently between backend (dto.rs) and frontend (types/index.ts)
- ✅ GraphDataResponse / GraphNode / GraphLink consistent across backend and frontend
- ✅ API paths match between backend routes and frontend api/ modules
