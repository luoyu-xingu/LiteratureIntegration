use axum::{
    routing::{delete, get, post, put},
    Router,
    extract::{Path, Query, State},
    Json,
    http::{StatusCode, header},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Workspace {
    id: String,
    name: String,
    description: String,
    created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Paper {
    id: String,
    title: String,
    doi: Option<String>,
    arxiv_id: Option<String>,
    abstract_text: Option<String>,
    user_notes: Option<String>,
    year: Option<i32>,
    journal: Option<String>,
    created_at: String,
    workspace_id: String,
    first_author_name: Option<String>,
    corresponding_author_name: Option<String>,
    keyword_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Author {
    id: String,
    name: String,
    orcid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Keyword {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct CreateWorkspaceRequest {
    name: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UpdateWorkspaceRequest {
    name: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ImportPaperRequest {
    identifier: String,
}

#[derive(Debug, Deserialize)]
struct UpdatePaperRequest {
    user_notes: Option<String>,
    corresponding_author_id: Option<String>,
}

#[derive(Debug, Serialize)]
struct PaperDetailResponse {
    paper: PaperView,
    first_author: Option<Author>,
    corresponding_author: Option<Author>,
    keywords: Vec<Keyword>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PaperView {
    id: String,
    title: String,
    doi: Option<String>,
    arxiv_id: Option<String>,
    abstract_text: Option<String>,
    user_notes: Option<String>,
    year: Option<i32>,
    journal: Option<String>,
    created_at: String,
}

#[derive(Debug, Serialize)]
struct GraphDataResponse {
    nodes: Vec<GraphNode>,
    links: Vec<GraphLink>,
}

#[derive(Debug, Serialize)]
struct GraphNode {
    id: String,
    name: String,
    paper_count: i32,
    author_type: String,
}

#[derive(Debug, Serialize)]
struct GraphLink {
    source: String,
    target: String,
    paper_count: i32,
}

#[derive(Debug, Serialize)]
struct AuthorWithPapers {
    author: Author,
    papers: Vec<PaperView>,
}

#[derive(Debug, Deserialize)]
struct ExportRequest {
    format: String,
    group_by: Option<String>,
}

#[derive(Debug, Deserialize)]
struct WorkspaceQuery {
    workspace_id: String,
}

#[derive(Debug, Deserialize)]
struct DeletePaperQuery {
    workspace_id: String,
    paper_id: String,
}

#[derive(Debug, Deserialize)]
struct SearchParams {
    workspace_id: String,
    q: Option<String>,
    author: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ExportQuery {
    workspace_id: String,
}

#[derive(Debug, Clone)]
struct AppError(String);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let body = serde_json::json!({ "error": { "code": "ERROR", "message": self.0 } });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
    }
}

use axum::response::IntoResponse;

struct AppState {
    workspaces: Mutex<HashMap<String, Workspace>>,
    papers: Mutex<HashMap<String, Paper>>,
    authors: Mutex<HashMap<String, Author>>,
    keywords: Mutex<HashMap<String, Keyword>>,
    workspace_papers: Mutex<HashMap<String, Vec<String>>>,
}

impl AppState {
    fn new() -> Self {
        let mut workspaces = HashMap::new();
        let mut papers = HashMap::new();
        let mut authors = HashMap::new();
        let mut keywords = HashMap::new();
        let mut workspace_papers = HashMap::new();

        let ws1_id = uuid::Uuid::new_v4().to_string();
        let ws2_id = uuid::Uuid::new_v4().to_string();

        workspaces.insert(ws1_id.clone(), Workspace {
            id: ws1_id.clone(),
            name: "深度学习论文集".to_string(),
            description: "深度学习相关论文收集".to_string(),
            created_at: "2025-01-15T08:00:00Z".to_string(),
        });
        workspaces.insert(ws2_id.clone(), Workspace {
            id: ws2_id.clone(),
            name: "自然语言处理".to_string(),
            description: "NLP 相关论文收集".to_string(),
            created_at: "2025-02-20T10:00:00Z".to_string(),
        });

        let a1_id = uuid::Uuid::new_v4().to_string();
        let a2_id = uuid::Uuid::new_v4().to_string();
        let a3_id = uuid::Uuid::new_v4().to_string();

        authors.insert(a1_id.clone(), Author { id: a1_id.clone(), name: "Ashish Vaswani".to_string(), orcid: None });
        authors.insert(a2_id.clone(), Author { id: a2_id.clone(), name: "Noam Shazeer".to_string(), orcid: None });
        authors.insert(a3_id.clone(), Author { id: a3_id.clone(), name: "Jacob Devlin".to_string(), orcid: None });

        let k1_id = uuid::Uuid::new_v4().to_string();
        let k2_id = uuid::Uuid::new_v4().to_string();
        let k3_id = uuid::Uuid::new_v4().to_string();
        let k4_id = uuid::Uuid::new_v4().to_string();

        keywords.insert(k1_id.clone(), Keyword { id: k1_id.clone(), name: "transformer".to_string() });
        keywords.insert(k2_id.clone(), Keyword { id: k2_id.clone(), name: "attention".to_string() });
        keywords.insert(k3_id.clone(), Keyword { id: k3_id.clone(), name: "deep-learning".to_string() });
        keywords.insert(k4_id.clone(), Keyword { id: k4_id.clone(), name: "BERT".to_string() });

        let p1_id = uuid::Uuid::new_v4().to_string();
        let p2_id = uuid::Uuid::new_v4().to_string();
        let p3_id = uuid::Uuid::new_v4().to_string();

        papers.insert(p1_id.clone(), Paper {
            id: p1_id.clone(),
            title: "Attention Is All You Need".to_string(),
            doi: Some("10.5555/3295222.3295349".to_string()),
            arxiv_id: Some("1706.03762".to_string()),
            abstract_text: Some("We propose a new simple network architecture, the Transformer, based solely on attention mechanisms, dispensing with recurrence and convolutions entirely.".to_string()),
            user_notes: Some("核心贡献：提出了 Self-Attention 机制，彻底改变了 NLP 领域".to_string()),
            year: Some(2017),
            journal: Some("NeurIPS".to_string()),
            created_at: "2025-01-15T09:00:00Z".to_string(),
            workspace_id: ws1_id.clone(),
            first_author_name: Some("Ashish Vaswani".to_string()),
            corresponding_author_name: Some("Noam Shazeer".to_string()),
            keyword_names: vec!["transformer".to_string(), "attention".to_string(), "deep-learning".to_string()],
        });

        papers.insert(p2_id.clone(), Paper {
            id: p2_id.clone(),
            title: "BERT: Pre-training of Deep Bidirectional Transformers for Language Understanding".to_string(),
            doi: Some("10.18653/v1/N19-1423".to_string()),
            arxiv_id: Some("1810.04805".to_string()),
            abstract_text: Some("We introduce a new language representation model called BERT, which stands for Bidirectional Encoder Representations from Transformers.".to_string()),
            user_notes: None,
            year: Some(2019),
            journal: Some("NAACL".to_string()),
            created_at: "2025-01-16T10:00:00Z".to_string(),
            workspace_id: ws1_id.clone(),
            first_author_name: Some("Jacob Devlin".to_string()),
            corresponding_author_name: Some("Jacob Devlin".to_string()),
            keyword_names: vec!["BERT".to_string(), "deep-learning".to_string()],
        });

        papers.insert(p3_id.clone(), Paper {
            id: p3_id.clone(),
            title: "Deep Residual Learning for Image Recognition".to_string(),
            doi: Some("10.1109/CVPR.2016.90".to_string()),
            arxiv_id: Some("1512.03385".to_string()),
            abstract_text: Some("We present a residual learning framework to ease the training of networks that are substantially deeper than those used previously.".to_string()),
            user_notes: Some("ResNet 的开创性工作，残差连接是深度学习的重要技巧".to_string()),
            year: Some(2016),
            journal: Some("CVPR".to_string()),
            created_at: "2025-02-20T11:00:00Z".to_string(),
            workspace_id: ws2_id.clone(),
            first_author_name: Some("Kaiming He".to_string()),
            corresponding_author_name: Some("Jian Sun".to_string()),
            keyword_names: vec!["deep-learning".to_string(), "residual-learning".to_string()],
        });

        workspace_papers.insert(ws1_id.clone(), vec![p1_id.clone(), p2_id.clone()]);
        workspace_papers.insert(ws2_id.clone(), vec![p3_id.clone()]);

        Self {
            workspaces: Mutex::new(workspaces),
            papers: Mutex::new(papers),
            authors: Mutex::new(authors),
            keywords: Mutex::new(keywords),
            workspace_papers: Mutex::new(workspace_papers),
        }
    }
}

fn paper_to_view(p: &Paper) -> PaperView {
    PaperView {
        id: p.id.clone(),
        title: p.title.clone(),
        doi: p.doi.clone(),
        arxiv_id: p.arxiv_id.clone(),
        abstract_text: p.abstract_text.clone(),
        user_notes: p.user_notes.clone(),
        year: p.year,
        journal: p.journal.clone(),
        created_at: p.created_at.clone(),
    }
}

async fn list_workspaces(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Workspace>>, AppError> {
    let ws = state.workspaces.lock().await;
    let mut list: Vec<Workspace> = ws.values().cloned().collect();
    list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(Json(list))
}

async fn create_workspace(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateWorkspaceRequest>,
) -> Result<Json<Workspace>, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let ws = Workspace {
        id: id.clone(),
        name: req.name,
        description: req.description.unwrap_or_default(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    state.workspaces.lock().await.insert(id.clone(), ws.clone());
    state.workspace_papers.lock().await.insert(id, vec![]);
    Ok(Json(ws))
}

async fn get_workspace(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Workspace>, AppError> {
    let ws = state.workspaces.lock().await;
    ws.get(&id).cloned().map(Json).ok_or_else(|| AppError(format!("Workspace not found: {}", id)))
}

async fn update_workspace(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<UpdateWorkspaceRequest>,
) -> Result<Json<Workspace>, AppError> {
    let mut ws_map = state.workspaces.lock().await;
    let ws = ws_map.get_mut(&id).ok_or_else(|| AppError(format!("Workspace not found: {}", id)))?;
    if let Some(name) = req.name { ws.name = name; }
    if let Some(desc) = req.description { ws.description = desc; }
    Ok(Json(ws.clone()))
}

async fn delete_workspace(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    state.workspaces.lock().await.remove(&id);
    state.workspace_papers.lock().await.remove(&id);
    Ok(Json(serde_json::json!({"deleted": true})))
}

async fn list_papers(
    State(state): State<Arc<AppState>>,
    Query(params): Query<WorkspaceQuery>,
) -> Result<Json<Vec<PaperView>>, AppError> {
    let wp = state.workspace_papers.lock().await;
    let paper_ids = wp.get(&params.workspace_id).cloned().unwrap_or_default();
    let papers = state.papers.lock().await;
    let mut list: Vec<PaperView> = paper_ids.iter()
        .filter_map(|id| papers.get(id).map(paper_to_view))
        .collect();
    list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(Json(list))
}

async fn import_paper(
    State(state): State<Arc<AppState>>,
    Query(params): Query<WorkspaceQuery>,
    Json(req): Json<ImportPaperRequest>,
) -> Result<Json<PaperDetailResponse>, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let paper = Paper {
        id: id.clone(),
        title: format!("Imported: {}", req.identifier),
        doi: if req.identifier.contains("10.") { Some(req.identifier.clone()) } else { None },
        arxiv_id: if !req.identifier.contains("10.") { Some(req.identifier.clone()) } else { None },
        abstract_text: Some("This is a mock imported paper. In production, this would fetch from Crossref or arXiv API.".to_string()),
        user_notes: None,
        year: Some(2024),
        journal: Some("Mock Journal".to_string()),
        created_at: chrono::Utc::now().to_rfc3339(),
        workspace_id: params.workspace_id.clone(),
        first_author_name: Some("Mock Author".to_string()),
        corresponding_author_name: Some("Mock Corresponding".to_string()),
        keyword_names: vec!["mock".to_string()],
    };

    let view = paper_to_view(&paper);
    state.papers.lock().await.insert(id.clone(), paper);
    let mut wp = state.workspace_papers.lock().await;
    wp.entry(params.workspace_id).or_default().push(id);

    Ok(Json(PaperDetailResponse {
        paper: view,
        first_author: Some(Author { id: uuid::Uuid::new_v4().to_string(), name: "Mock Author".to_string(), orcid: None }),
        corresponding_author: Some(Author { id: uuid::Uuid::new_v4().to_string(), name: "Mock Corresponding".to_string(), orcid: None }),
        keywords: vec![Keyword { id: uuid::Uuid::new_v4().to_string(), name: "mock".to_string() }],
    }))
}

async fn get_paper(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<PaperDetailResponse>, AppError> {
    let papers = state.papers.lock().await;
    let paper = papers.get(&id).ok_or_else(|| AppError(format!("Paper not found: {}", id)))?;
    let view = paper_to_view(paper);
    let authors = state.authors.lock().await;

    let first_author = paper.first_author_name.as_ref().and_then(|name| {
        authors.values().find(|a| a.name == *name).cloned()
    });
    let corresponding_author = paper.corresponding_author_name.as_ref().and_then(|name| {
        authors.values().find(|a| a.name == *name).cloned()
    });

    let kw_map = state.keywords.lock().await;
    let keyword_list: Vec<Keyword> = paper.keyword_names.iter().filter_map(|name| {
        kw_map.values().find(|k| k.name == *name).cloned()
    }).collect();

    Ok(Json(PaperDetailResponse {
        paper: view,
        first_author,
        corresponding_author,
        keywords: keyword_list,
    }))
}

async fn update_paper(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<UpdatePaperRequest>,
) -> Result<Json<PaperView>, AppError> {
    let mut papers = state.papers.lock().await;
    let paper = papers.get_mut(&id).ok_or_else(|| AppError(format!("Paper not found: {}", id)))?;
    if let Some(notes) = req.user_notes { paper.user_notes = Some(notes); }
    Ok(Json(paper_to_view(paper)))
}

async fn delete_paper(
    State(state): State<Arc<AppState>>,
    Query(params): Query<DeletePaperQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    state.papers.lock().await.remove(&params.paper_id);
    let mut wp = state.workspace_papers.lock().await;
    if let Some(ids) = wp.get_mut(&params.workspace_id) {
        ids.retain(|id| id != &params.paper_id);
    }
    Ok(Json(serde_json::json!({"removed": true})))
}

async fn list_authors(
    State(state): State<Arc<AppState>>,
    Query(params): Query<WorkspaceQuery>,
) -> Result<Json<Vec<Author>>, AppError> {
    let wp = state.workspace_papers.lock().await;
    let paper_ids = wp.get(&params.workspace_id).cloned().unwrap_or_default();
    let papers = state.papers.lock().await;
    let authors = state.authors.lock().await;

    let mut result: Vec<Author> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for pid in &paper_ids {
        if let Some(p) = papers.get(pid) {
            let names: Vec<&String> = [&p.first_author_name, &p.corresponding_author_name]
                .into_iter()
                .flatten()
                .collect();
            for name in names {
                if seen.insert(name.clone()) {
                    if let Some(a) = authors.values().find(|a| a.name == *name) {
                        result.push(a.clone());
                    } else {
                        result.push(Author { id: uuid::Uuid::new_v4().to_string(), name: name.clone(), orcid: None });
                    }
                }
            }
        }
    }
    Ok(Json(result))
}

async fn get_graph(
    State(state): State<Arc<AppState>>,
    Query(params): Query<WorkspaceQuery>,
) -> Result<Json<GraphDataResponse>, AppError> {
    let wp = state.workspace_papers.lock().await;
    let paper_ids = wp.get(&params.workspace_id).cloned().unwrap_or_default();
    let papers = state.papers.lock().await;

    let mut author_papers: HashMap<String, (i32, String)> = HashMap::new();
    let mut coauthor_pairs: HashMap<(String, String), i32> = HashMap::new();

    for pid in &paper_ids {
        if let Some(p) = papers.get(pid) {
            let mut paper_authors: Vec<String> = vec![];
            if let Some(ref name) = p.first_author_name {
                paper_authors.push(name.clone());
                let entry = author_papers.entry(name.clone()).or_insert((0, "first".to_string()));
                entry.0 += 1;
            }
            if let Some(ref name) = p.corresponding_author_name {
                if !paper_authors.contains(name) {
                    paper_authors.push(name.clone());
                }
                let entry = author_papers.entry(name.clone()).or_insert((0, "corresponding".to_string()));
                entry.0 += 1;
                if entry.1 == "first" {
                    entry.1 = "both".to_string();
                }
            }
            for i in 0..paper_authors.len() {
                for j in (i + 1)..paper_authors.len() {
                    let mut a = paper_authors[i].clone();
                    let mut b = paper_authors[j].clone();
                    if a > b { std::mem::swap(&mut a, &mut b); }
                    *coauthor_pairs.entry((a, b)).or_insert(0) += 1;
                }
            }
        }
    }

    let nodes: Vec<GraphNode> = author_papers.iter()
        .map(|(name, (count, atype))| GraphNode {
            id: name.clone(),
            name: name.clone(),
            paper_count: *count,
            author_type: atype.clone(),
        })
        .collect();

    let links: Vec<GraphLink> = coauthor_pairs.iter()
        .map(|((a, b), count)| GraphLink {
            source: a.clone(),
            target: b.clone(),
            paper_count: *count,
        })
        .collect();

    Ok(Json(GraphDataResponse { nodes, links }))
}

async fn get_author_papers(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<AuthorWithPapers>, AppError> {
    let authors = state.authors.lock().await;
    let author = match authors.get(&id).cloned() {
        Some(a) => a,
        None => Author { id: id.clone(), name: id.clone(), orcid: None },
    };
    drop(authors);

    let papers = state.papers.lock().await;
    let paper_views: Vec<PaperView> = papers.values()
        .filter(|p| p.first_author_name.as_deref() == Some(&author.name) || p.corresponding_author_name.as_deref() == Some(&author.name))
        .map(|p| paper_to_view(p))
        .collect();

    Ok(Json(AuthorWithPapers { author, papers: paper_views }))
}

async fn search(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SearchParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let wp = state.workspace_papers.lock().await;
    let paper_ids = wp.get(&params.workspace_id).cloned().unwrap_or_default();
    let papers = state.papers.lock().await;

    let workspace_papers: Vec<&Paper> = paper_ids.iter()
        .filter_map(|id| papers.get(id))
        .collect();

    if let Some(query) = params.q {
        let q = query.to_lowercase();
        let results: Vec<PaperView> = workspace_papers.iter()
            .filter(|p| {
                p.title.to_lowercase().contains(&q)
                    || p.abstract_text.as_ref().map_or(false, |a| a.to_lowercase().contains(&q))
                    || p.keyword_names.iter().any(|k| k.to_lowercase().contains(&q))
            })
            .map(|p| paper_to_view(p))
            .collect();
        Ok(Json(serde_json::json!({ "mode": "keyword", "query": query, "results": results })))
    } else if let Some(author) = params.author {
        let a = author.to_lowercase();
        let results: Vec<PaperView> = workspace_papers.iter()
            .filter(|p| {
                p.first_author_name.as_ref().map_or(false, |n| n.to_lowercase().contains(&a))
                    || p.corresponding_author_name.as_ref().map_or(false, |n| n.to_lowercase().contains(&a))
            })
            .map(|p| paper_to_view(p))
            .collect();
        Ok(Json(serde_json::json!({ "mode": "author", "query": author, "results": results })))
    } else {
        Ok(Json(serde_json::json!({ "mode": "none", "results": [] })))
    }
}

async fn export_workspace(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ExportQuery>,
    Json(_req): Json<ExportRequest>,
) -> Result<(StatusCode, [(header::HeaderName, &'static str); 1], String), AppError> {
    let ws_map = state.workspaces.lock().await;
    let ws = ws_map.get(&params.workspace_id).ok_or_else(|| AppError("Workspace not found".to_string()))?;

    let wp = state.workspace_papers.lock().await;
    let paper_ids = wp.get(&params.workspace_id).cloned().unwrap_or_default();
    let papers = state.papers.lock().await;

    let mut md = format!("# 工作区: {}\n\n", ws.name);
    md.push_str(&format!("> 导出时间: {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M")));
    md.push_str(&format!("> 论文数量: {}\n\n---\n\n", paper_ids.len()));

    for pid in &paper_ids {
        if let Some(p) = papers.get(pid) {
            md.push_str(&format!("### {}\n", p.title));
            if let Some(y) = p.year { md.push_str(&format!("- **年份**: {}\n", y)); }
            if let Some(ref j) = p.journal { md.push_str(&format!("- **期刊**: {}\n", j)); }
            if let Some(ref d) = p.doi { md.push_str(&format!("- **DOI**: {}\n", d)); }
            if let Some(ref a) = p.first_author_name { md.push_str(&format!("- **一作**: {}\n", a)); }
            if let Some(ref a) = p.corresponding_author_name { md.push_str(&format!("- **通讯**: {}\n", a)); }
            if !p.keyword_names.is_empty() { md.push_str(&format!("- **关键词**: {}\n", p.keyword_names.join(", "))); }
            md.push_str("\n");
            if let Some(ref abs) = p.abstract_text { md.push_str(&format!("**Abstract:**\n{}\n\n", abs)); }
            if let Some(ref notes) = p.user_notes { if !notes.is_empty() { md.push_str(&format!("**笔记:**\n{}\n\n", notes)); } }
            md.push_str("---\n\n");
        }
    }

    Ok((StatusCode::OK, [(header::CONTENT_TYPE, "text/markdown; charset=utf-8")], md))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .route("/api/workspaces", get(list_workspaces).post(create_workspace))
        .route("/api/workspace/:id", get(get_workspace).put(update_workspace).delete(delete_workspace))
        .route("/api/papers", get(list_papers).post(import_paper))
        .route("/api/paper/:id", get(get_paper).put(update_paper))
        .route("/api/paper-rm", delete(delete_paper))
        .route("/api/authors", get(list_authors))
        .route("/api/graph", get(get_graph))
        .route("/api/author-papers/:id", get(get_author_papers))
        .route("/api/search", get(search))
        .route("/api/export", post(export_workspace))
        .fallback_service(ServeDir::new("../frontend/dist").append_index_html_on_directories(true))
        .with_state(state)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Mock server running on 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
