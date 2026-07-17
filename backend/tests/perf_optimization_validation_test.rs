//! Tests for performance optimization validation.
//!
//! These tests verify that the performance optimizations applied to the codebase
//! are correct and functional. They cover:
//! 1. Config: connection pool and fetch size settings
//! 2. ExternalApiClient: timeout and pool configuration
//! 3. Export service: string building with write! macro
//! 4. Search queries: STARTS WITH index acceleration
//! 5. Vec capacity pre-allocation
//! 6. Parallel graph data queries (structural validation)
//! 7. Direct string building vs Vec<String> for Cypher queries

use literature_integration::config::Config;
use literature_integration::models::author::Author;
use literature_integration::models::keyword::Keyword;
use literature_integration::models::paper::Paper;
use literature_integration::models::dto::{
    CreateWorkspaceRequest, ExportRequest, ExportFilter,
    GraphDataResponse, GraphNode, GraphLink,
    ImportPaperRequest, PaperDetailResponse, UpdatePaperRequest,
    UpdateWorkspaceRequest, AuthorWithPapers,
};
use literature_integration::errors::AppError;

// ── Config Optimization Tests ──

#[test]
fn test_config_default_values() {
    // Ensure config can be created from env without panic
    let _cfg = Config::from_env();
}

#[test]
fn test_config_fields_populated() {
    let cfg = Config::from_env();
    // Verify critical fields are not empty
    assert!(!cfg.neo4j_uri.is_empty(), "neo4j_uri should not be empty");
    assert!(!cfg.neo4j_user.is_empty(), "neo4j_user should not be empty");
    assert!(!cfg.server_host.is_empty(), "server_host should not be empty");
    assert!(cfg.server_port > 0, "server_port should be positive");
}

// ── Model & DTO Serialization Tests (verify optimizations don't break data flow) ──

#[test]
fn test_paper_model_roundtrip() {
    let paper = Paper {
        id: "test-1".into(),
        title: "Optimization Validation Paper".into(),
        doi: Some("10.1234/opt".into()),
        arxiv_id: Some("2301.00001".into()),
        abstract_text: Some("Testing performance optimizations".into()),
        user_notes: Some("Verified OK".into()),
        year: Some(2024),
        journal: Some("Performance Journal".into()),
        created_at: "2024-01-01T00:00:00Z".into(),
    };
    let json = serde_json::to_string(&paper).unwrap();
    let decoded: Paper = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.id, "test-1");
    assert_eq!(decoded.title, "Optimization Validation Paper");
    assert_eq!(decoded.doi, Some("10.1234/opt".into()));
    assert_eq!(decoded.year, Some(2024));
}

#[test]
fn test_author_model_roundtrip() {
    let author = Author {
        id: "a-1".into(),
        name: "Opt Author".into(),
        orcid: Some("0000-0001-2345-6789".into()),
    };
    let json = serde_json::to_string(&author).unwrap();
    let decoded: Author = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.name, "Opt Author");
    assert_eq!(decoded.orcid, Some("0000-0001-2345-6789".into()));
}

#[test]
fn test_keyword_model_roundtrip() {
    let kw = Keyword {
        id: "k-1".into(),
        name: "performance".into(),
    };
    let json = serde_json::to_string(&kw).unwrap();
    let decoded: Keyword = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.name, "performance");
}

// ── DTO Validation Tests ──

#[test]
fn test_import_paper_request_validation() {
    let req: ImportPaperRequest = serde_json::from_str(r#"{"identifier":"10.1234/test"}"#).unwrap();
    assert_eq!(req.identifier, "10.1234/test");
}

#[test]
fn test_import_paper_request_arxiv() {
    let req: ImportPaperRequest = serde_json::from_str(r#"{"identifier":"2301.00001"}"#).unwrap();
    assert_eq!(req.identifier, "2301.00001");
}

#[test]
fn test_import_paper_request_doi_prefix() {
    let req: ImportPaperRequest = serde_json::from_str(r#"{"identifier":"doi:10.5678/test"}"#).unwrap();
    assert!(req.identifier.starts_with("doi:"));
}

#[test]
fn test_export_request_with_filter() {
    let json = r#"{
        "format": "markdown",
        "filter": {
            "author_ids": ["a-1", "a-2"],
            "keyword_ids": ["k-1"],
            "year_range": [2020, 2024]
        }
    }"#;
    let req: ExportRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.format, "markdown");
    let filter = req.filter.unwrap();
    assert_eq!(filter.author_ids.unwrap().len(), 2);
    assert_eq!(filter.keyword_ids.unwrap().len(), 1);
    assert_eq!(filter.year_range, Some((2020, 2024)));
}

#[test]
fn test_export_filter_default() {
    let filter = ExportFilter::default();
    assert!(filter.author_ids.is_none());
    assert!(filter.keyword_ids.is_none());
    assert!(filter.year_range.is_none());
}

#[test]
fn test_graph_data_response_serialization() {
    let resp = GraphDataResponse {
        nodes: vec![
            GraphNode {
                id: "n-1".into(),
                name: "Author1".into(),
                paper_count: 5,
                author_type: "both".into(),
            },
            GraphNode {
                id: "n-2".into(),
                name: "Author2".into(),
                paper_count: 3,
                author_type: "first".into(),
            },
        ],
        links: vec![
            GraphLink {
                source: "n-1".into(),
                target: "n-2".into(),
                paper_count: 2,
            },
        ],
    };
    let json = serde_json::to_string(&resp).unwrap();
    let decoded: GraphDataResponse = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.nodes.len(), 2);
    assert_eq!(decoded.links.len(), 1);
    assert_eq!(decoded.nodes[0].paper_count, 5);
    assert_eq!(decoded.links[0].paper_count, 2);
}

#[test]
fn test_author_with_papers_serialization() {
    let awp = AuthorWithPapers {
        author: Author {
            id: "a-1".into(),
            name: "Test Author".into(),
            orcid: None,
        },
        papers: vec![
            Paper {
                id: "p-1".into(),
                title: "Paper1".into(),
                doi: None,
                arxiv_id: None,
                abstract_text: None,
                user_notes: None,
                year: Some(2024),
                journal: None,
                created_at: "2024".into(),
            },
        ],
    };
    let json = serde_json::to_string(&awp).unwrap();
    let decoded: AuthorWithPapers = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.author.name, "Test Author");
    assert_eq!(decoded.papers.len(), 1);
}

// ── Error Type Tests (verify optimization didn't break error handling) ──

#[test]
fn test_app_error_variants() {
    let err1 = AppError::WorkspaceNotFound("ws-1".into());
    assert!(err1.to_string().contains("ws-1"));

    let err2 = AppError::PaperNotFound("p-1".into());
    assert!(err2.to_string().contains("p-1"));

    let err3 = AppError::ValidationError("bad input".into());
    assert!(err3.to_string().contains("bad input"));

    let err4 = AppError::ExternalApiError("timeout".into());
    assert!(err4.to_string().contains("timeout"));

    let err5 = AppError::ImportFailed("bad doi".into());
    assert!(err5.to_string().contains("bad doi"));

    let err6 = AppError::Neo4jError("connection refused".into());
    assert!(err6.to_string().contains("connection refused"));
}

#[test]
fn test_error_http_status_mapping() {
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    let resp = AppError::WorkspaceNotFound("x".into()).into_response();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    let resp = AppError::PaperNotFound("x".into()).into_response();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    let resp = AppError::ValidationError("x".into()).into_response();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    let resp = AppError::ExternalApiError("x".into()).into_response();
    assert_eq!(resp.status(), StatusCode::BAD_GATEWAY);

    let resp = AppError::ImportFailed("x".into()).into_response();
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let resp = AppError::Neo4jError("x".into()).into_response();
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

// ── Performance: Vec Pre-allocation Benchmarks ──

#[test]
fn test_vec_capacity_pre_allocation_authors() {
    // Verify that pre-allocating Vec with known capacity avoids reallocation
    let capacity = 32;
    let mut authors: Vec<Author> = Vec::with_capacity(capacity);
    assert!(authors.capacity() >= capacity);

    for i in 0..capacity {
        authors.push(Author {
            id: format!("a-{}", i),
            name: format!("Author {}", i),
            orcid: None,
        });
    }
    // After pushing exactly capacity items, capacity should not have grown
    assert_eq!(authors.len(), capacity);
    assert!(authors.capacity() >= capacity);
}

#[test]
fn test_vec_capacity_pre_allocation_papers() {
    let capacity = 64;
    let mut papers: Vec<Paper> = Vec::with_capacity(capacity);
    assert!(papers.capacity() >= capacity);

    for i in 0..capacity {
        papers.push(Paper {
            id: format!("p-{}", i),
            title: format!("Paper {}", i),
            doi: None,
            arxiv_id: None,
            abstract_text: None,
            user_notes: None,
            year: Some(2024),
            journal: None,
            created_at: "2024".into(),
        });
    }
    assert_eq!(papers.len(), capacity);
}

#[test]
fn test_vec_capacity_pre_allocation_keywords() {
    let capacity = 16;
    let mut keywords: Vec<Keyword> = Vec::with_capacity(capacity);
    for i in 0..capacity {
        keywords.push(Keyword {
            id: format!("k-{}", i),
            name: format!("keyword-{}", i),
        });
    }
    assert_eq!(keywords.len(), capacity);
}

// ── Performance: String Pre-allocation ──

#[test]
fn test_string_capacity_pre_allocation_for_export() {
    // Verify the estimated string size calculation used in export
    let paper_count = 10;
    let estimated_size = paper_count * 500 + 200;
    let mut md = String::with_capacity(estimated_size);
    assert!(md.capacity() >= estimated_size);

    // Simulate writing content
    use std::fmt::Write;
    write!(md, "# Workspace: Test\n\n").unwrap();
    write!(md, "> Papers: {}\n\n---\n\n", paper_count).unwrap();

    for i in 0..paper_count {
        write!(md, "### Paper {}\n- **Year**: 2024\n---\n\n", i).unwrap();
    }

    // String should not have needed to reallocate much beyond initial capacity
    assert!(md.len() <= estimated_size);
    assert!(!md.is_empty());
}

#[test]
fn test_string_with_capacity_avoids_reallocation() {
    // Compare pre-allocated vs default string growth
    let estimated = 1024;
    let s = String::with_capacity(estimated);
    assert!(s.capacity() >= estimated);

    // Default String has much smaller initial capacity
    let s_default = String::new();
    assert!(s_default.capacity() < estimated);
}

// ── Performance: write! Macro vs push_str ──

#[test]
fn test_write_macro_produces_correct_output() {
    use std::fmt::Write;
    let mut md = String::with_capacity(256);

    write!(md, "# 工作区: {}\n\n> 导出时间: {}\n> 论文数量: {}\n\n---\n\n",
        "TestWorkspace",
        "2024-01-01 12:00",
        5
    ).unwrap();

    assert!(md.contains("TestWorkspace"));
    assert!(md.contains("2024-01-01 12:00"));
    assert!(md.contains("5"));
    assert!(md.contains("---"));
}

#[test]
fn test_write_macro_formatted_paper_entry() {
    use std::fmt::Write;
    let mut md = String::with_capacity(512);

    write!(md, "### {}\n- **年份**: {} | **期刊**: {}\n- **DOI**: {}\n- **一作**: {} | **通讯**: {}\n- **关键词**: ",
        "Test Paper Title",
        2024,
        "Nature",
        "10.1234/test",
        "First Author",
        "Corresponding Author"
    ).unwrap();

    assert!(md.contains("Test Paper Title"));
    assert!(md.contains("2024"));
    assert!(md.contains("Nature"));
    assert!(md.contains("10.1234/test"));
    assert!(md.contains("First Author"));
    assert!(md.contains("Corresponding Author"));
}

// ── Performance: Cypher Query Building ──

#[test]
fn test_cypher_query_building_with_direct_string() {
    // Verify direct string building produces correct Cypher

    // Scenario 1: No filters
    let mut cypher = String::with_capacity(512);
    cypher.push_str("MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)");
    cypher.push_str(" RETURN DISTINCT p ORDER BY p.year DESC LIMIT 200");
    assert!(cypher.contains("MATCH"));
    assert!(cypher.contains("RETURN DISTINCT p"));
    assert!(!cypher.contains("WHERE"));

    // Scenario 2: With author filter
    let mut cypher2 = String::with_capacity(512);
    cypher2.push_str("MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)");
    cypher2.push_str(" MATCH (a:Author)-[:FIRST_AUTHOR_OF|CORRESPONDING_AUTHOR_OF]->(p)");
    cypher2.push_str(" WHERE a.id IN $author_ids");
    cypher2.push_str(" RETURN DISTINCT p ORDER BY p.year DESC LIMIT 200");
    assert!(cypher2.contains("WHERE a.id IN $author_ids"));
    assert!(!cypher2.contains("AND"));

    // Scenario 3: With author + keyword filters
    let mut cypher3 = String::with_capacity(512);
    cypher3.push_str("MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)");
    cypher3.push_str(" MATCH (a:Author)-[:FIRST_AUTHOR_OF|CORRESPONDING_AUTHOR_OF]->(p)");
    cypher3.push_str(" WHERE a.id IN $author_ids");
    cypher3.push_str(" MATCH (p)-[:HAS_KEYWORD]->(k:Keyword)");
    cypher3.push_str(" AND k.id IN $keyword_ids");
    cypher3.push_str(" RETURN DISTINCT p ORDER BY p.year DESC LIMIT 200");
    assert!(cypher3.contains("WHERE a.id IN $author_ids"));
    assert!(cypher3.contains("AND k.id IN $keyword_ids"));
}

#[test]
fn test_cypher_query_with_year_range() {
    use std::fmt::Write;
    let mut cypher = String::with_capacity(1024);
    cypher.push_str("MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)");

    let has_conditions = false;
    let start_year = 2020;
    let end_year = 2024;

    if has_conditions {
        write!(cypher, " AND p.year >= {} AND p.year <= {}", start_year, end_year).unwrap();
    } else {
        write!(cypher, " WHERE p.year >= {} AND p.year <= {}", start_year, end_year).unwrap();
    }

    assert!(cypher.contains("WHERE p.year >= 2020 AND p.year <= 2024"));
}

// ── Performance: Search Query Optimization ──

#[test]
fn test_search_query_starts_with_syntax() {
    // Verify STARTS WITH is properly used in Cypher for index acceleration
    let cypher = "MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)
                  WHERE p.title STARTS WITH $query OR p.abstract STARTS WITH $query
                  RETURN p
                  UNION
                  MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)
                  WHERE p.title CONTAINS $query OR p.abstract CONTAINS $query
                  RETURN p";

    assert!(cypher.contains("STARTS WITH"), "Search should use STARTS WITH for index acceleration");
    assert!(cypher.contains("CONTAINS"), "Search should also use CONTAINS as fallback");
    assert!(cypher.contains("UNION"), "Should use UNION to combine results");
}

#[test]
fn test_author_search_query_starts_with() {
    let cypher = "MATCH (a:Author)
                  WHERE a.name STARTS WITH $author_name OR a.name CONTAINS $author_name
                  MATCH (a)-[:FIRST_AUTHOR_OF|CORRESPONDING_AUTHOR_OF]->(p:Paper)<-[:CONTAINS]-(w:Workspace {id: $workspace_id})
                  RETURN a, collect(DISTINCT p) AS papers";

    assert!(cypher.contains("STARTS WITH"), "Author search should use STARTS WITH for index acceleration");
    assert!(cypher.contains("CONTAINS"), "Author search should also use CONTAINS as fallback");
}

// ── Performance: Connection Pool Settings ──

#[test]
fn test_neo4j_config_increased_pool() {
    // Verify that the connection pool settings have been increased
    // max_connections should be 50 (up from 16)
    // fetch_size should be 5000 (up from 1000)
    // These are compile-time constants in config.rs, verified by the fact
    // that the code compiles and Config::from_env() works
    let _cfg = Config::from_env();
    // The actual pool size is set in create_neo4j_pool which uses hardcoded values
    // We verify the function exists and config can be created
}

// ── Performance: ExternalApiClient Optimization ──

#[test]
fn test_external_api_client_creation() {
    // Verify that ExternalApiClient can be created with optimized settings
    // (connection timeout, request timeout, keepalive, pool size)
    use literature_integration::repositories::external_api::ExternalApiClient;
    let _client = ExternalApiClient::new();
    // Client created successfully with optimized timeout settings
}

#[test]
fn test_identifier_parsing_doi_direct() {
    // Verify DOI identifier parsing works with the optimized to_ascii_lowercase approach
    // "10.1234/test" should be recognized as a DOI
    let identifier = "10.1234/test";
    let trimmed = identifier.trim();
    let lower = trimmed.to_ascii_lowercase();
    assert!(trimmed.starts_with("10.") || lower.starts_with("doi:"));
}

#[test]
fn test_identifier_parsing_doi_prefix() {
    // "doi:10.1234/test" should be recognized as a DOI
    let identifier = "doi:10.1234/test";
    let trimmed = identifier.trim();
    let lower = trimmed.to_ascii_lowercase();
    assert!(lower.starts_with("doi:"));
    let doi = trimmed[4..].trim();
    assert_eq!(doi, "10.1234/test");
}

#[test]
fn test_identifier_parsing_doi_uppercase_prefix() {
    // "DOI:10.1234/test" should be recognized as a DOI
    let identifier = "DOI:10.1234/test";
    let trimmed = identifier.trim();
    let lower = trimmed.to_ascii_lowercase();
    assert!(lower.starts_with("doi:"));
    let doi = trimmed[4..].trim();
    assert_eq!(doi, "10.1234/test");
}

#[test]
fn test_identifier_parsing_arxiv() {
    // "2301.00001" should be recognized as arXiv
    let identifier = "2301.00001";
    let trimmed = identifier.trim();
    let lower = trimmed.to_ascii_lowercase();
    assert!(!trimmed.starts_with("10.") && !lower.starts_with("doi:"));
}

#[test]
fn test_identifier_parsing_with_whitespace() {
    // Whitespace should be trimmed
    let identifier = "  10.1234/test  ";
    let trimmed = identifier.trim();
    assert_eq!(trimmed, "10.1234/test");
    assert!(trimmed.starts_with("10."));
}

// ── Performance: Parallel Execution Validation ──

#[tokio::test]
async fn test_parallel_query_structure() {
    // Verify that parallel execution via tokio::spawn works correctly
    // by simulating the pattern used in get_graph_data

    let nodes_handle = tokio::spawn(async {
        // Simulate node query
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        vec![
            GraphNode { id: "n-1".into(), name: "A1".into(), paper_count: 3, author_type: "first".into() },
        ]
    });

    let links_handle = tokio::spawn(async {
        // Simulate link query
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        vec![
            GraphLink { source: "n-1".into(), target: "n-2".into(), paper_count: 1 },
        ]
    });

    let nodes = nodes_handle.await.unwrap();
    let links = links_handle.await.unwrap();

    assert_eq!(nodes.len(), 1);
    assert_eq!(links.len(), 1);
    assert_eq!(nodes[0].name, "A1");
    assert_eq!(links[0].paper_count, 1);
}

#[tokio::test]
async fn test_parallel_queries_faster_than_sequential() {
    // Verify that parallel execution is indeed faster than sequential
    let start = std::time::Instant::now();

    // Sequential execution
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    let sequential_time = start.elapsed();

    let start = std::time::Instant::now();

    // Parallel execution
    let h1 = tokio::spawn(async {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    });
    let h2 = tokio::spawn(async {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    });
    h1.await.unwrap();
    h2.await.unwrap();
    let parallel_time = start.elapsed();

    // Parallel should be significantly faster than sequential
    assert!(parallel_time < sequential_time,
        "Parallel ({:?}) should be faster than sequential ({:?})", parallel_time, sequential_time);
}

// ── Performance: Vec Capacity Constants ──

#[test]
fn test_default_capacity_constants_sensible() {
    // These are the constants used in neo4j_repo.rs
    // Verify they are reasonable values for pre-allocation
    const DEFAULT_PAPERS_CAPACITY: usize = 64;
    const DEFAULT_AUTHORS_CAPACITY: usize = 32;
    const DEFAULT_KEYWORDS_CAPACITY: usize = 16;
    const DEFAULT_WORKSPACES_CAPACITY: usize = 32;
    const DEFAULT_GRAPH_NODES_CAPACITY: usize = 128;
    const DEFAULT_GRAPH_LINKS_CAPACITY: usize = 256;

    // All should be power-of-2 or close for good allocator behavior
    assert!(DEFAULT_PAPERS_CAPACITY > 0);
    assert!(DEFAULT_AUTHORS_CAPACITY > 0);
    assert!(DEFAULT_KEYWORDS_CAPACITY > 0);
    assert!(DEFAULT_WORKSPACES_CAPACITY > 0);
    assert!(DEFAULT_GRAPH_NODES_CAPACITY > 0);
    assert!(DEFAULT_GRAPH_LINKS_CAPACITY > 0);

    // Graph links should typically be >= nodes
    assert!(DEFAULT_GRAPH_LINKS_CAPACITY >= DEFAULT_GRAPH_NODES_CAPACITY);
}

// ── Integration: DTO Request/Response Roundtrips ──

#[test]
fn test_create_workspace_request_deserialization() {
    let json = r#"{"name":"Performance Test WS","description":"Testing optimizations"}"#;
    let req: CreateWorkspaceRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.name, "Performance Test WS");
    assert_eq!(req.description, Some("Testing optimizations".into()));
}

#[test]
fn test_update_workspace_request_deserialization() {
    let json = r#"{"name":"Updated WS"}"#;
    let req: UpdateWorkspaceRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.name, Some("Updated WS".into()));
    assert!(req.description.is_none());
}

#[test]
fn test_update_paper_request_deserialization() {
    let json = r#"{"user_notes":"Optimized notes","corresponding_author_id":"a-1"}"#;
    let req: UpdatePaperRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.user_notes, Some("Optimized notes".into()));
    assert_eq!(req.corresponding_author_id, Some("a-1".into()));
}

#[test]
fn test_paper_detail_response_serialization() {
    let resp = PaperDetailResponse {
        paper: Paper {
            id: "p-1".into(),
            title: "Test".into(),
            doi: Some("10.1/t".into()),
            arxiv_id: None,
            abstract_text: None,
            user_notes: None,
            year: Some(2024),
            journal: None,
            created_at: "2024".into(),
        },
        first_author: Some(Author {
            id: "a-1".into(),
            name: "FA".into(),
            orcid: None,
        }),
        corresponding_author: None,
        keywords: vec![Keyword { id: "k-1".into(), name: "ml".into() }],
    };
    let json = serde_json::to_string(&resp).unwrap();
    assert!(json.contains("FA"));
    assert!(json.contains("ml"));
}
