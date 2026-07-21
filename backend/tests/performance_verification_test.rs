use std::time::{Duration, Instant};

use neo4rs::Graph;
use literature_integration::repositories::neo4j_repo::Neo4jRepo;

async fn setup_test_data(repo: &Neo4jRepo) -> (String, String) {
    let ws_id = uuid::Uuid::new_v4().to_string();
    let paper_id = uuid::Uuid::new_v4().to_string();
    
    repo.create_workspace(&ws_id, "PerfVerifyTest", "", "2025-01-01T00:00:00Z").await.unwrap();
    
    repo.create_paper_if_not_exists(
        &paper_id,
        "Performance Verification Test Paper",
        Some("10.1234/verify"),
        None,
        Some("Abstract for performance verification testing"),
        Some(2024),
        Some("Performance Journal"),
        "2025-01-01T00:00:00Z",
    ).await.unwrap();
    
    repo.add_paper_to_workspace(&ws_id, &paper_id, "2025-01-01T00:00:00Z").await.unwrap();
    
    let author1_id = uuid::Uuid::new_v4().to_string();
    let author2_id = uuid::Uuid::new_v4().to_string();
    let author3_id = uuid::Uuid::new_v4().to_string();
    
    repo.create_author_if_not_exists(&author1_id, "Author One", None).await.unwrap();
    repo.create_author_if_not_exists(&author2_id, "Author Two", None).await.unwrap();
    repo.create_author_if_not_exists(&author3_id, "Author Three", None).await.unwrap();
    
    repo.link_first_author(&author1_id, &paper_id).await.unwrap();
    repo.link_corresponding_author(&author2_id, &paper_id).await.unwrap();
    repo.link_co_authors(&author1_id, &author2_id, &ws_id).await.unwrap();
    repo.link_co_authors(&author1_id, &author3_id, &ws_id).await.unwrap();
    repo.link_co_authors(&author2_id, &author3_id, &ws_id).await.unwrap();
    
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "performance", &paper_id).await.unwrap();
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "testing", &paper_id).await.unwrap();
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "optimization", &paper_id).await.unwrap();
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "benchmark", &paper_id).await.unwrap();
    
    (ws_id, paper_id)
}

async fn get_graph() -> Graph {
    dotenvy::dotenv().ok();
    let uri = std::env::var("NEO4J_URI").unwrap_or_else(|_| "bolt://localhost:7687".into());
    let user = std::env::var("NEO4J_USER").unwrap_or_else(|_| "neo4j".into());
    let password = std::env::var("NEO4J_PASSWORD").unwrap_or_else(|_| "password".into());

    let config = neo4rs::ConfigBuilder::default()
        .uri(&uri)
        .user(&user)
        .password(&password)
        .max_connections(8)
        .fetch_size(1000)
        .build()
        .unwrap();

    Graph::connect(config).await.unwrap()
}

const GET_PAPER_DETAIL_THRESHOLD: Duration = Duration::from_millis(50);
const GET_GRAPH_DATA_THRESHOLD: Duration = Duration::from_millis(100);
const CREATE_AUTHORS_BATCH_THRESHOLD: Duration = Duration::from_millis(80);
const LIST_PAPERS_THRESHOLD: Duration = Duration::from_millis(30);
const SEARCH_BY_KEYWORD_THRESHOLD: Duration = Duration::from_millis(50);

#[tokio::test]
async fn verify_performance_requirements() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (ws_id, paper_id) = setup_test_data(&repo).await;
    
    let mut results = Vec::new();
    
    let result = test_get_paper_detail_performance(&repo, &paper_id).await;
    results.push(("get_paper_detail", result));
    
    let result = test_get_graph_data_performance(&repo, &ws_id).await;
    results.push(("get_graph_data", result));
    
    let result = test_create_authors_batch_performance(&repo, &ws_id, &paper_id).await;
    results.push(("create_authors_batch", result));
    
    let result = test_list_papers_performance(&repo, &ws_id).await;
    results.push(("list_papers_in_workspace", result));
    
    let result = test_search_by_keyword_performance(&repo, &ws_id).await;
    results.push(("search_by_keyword", result));
    
    let failed_tests: Vec<_> = results.iter().filter(|(_, passed)| !passed).map(|(name, _)| name).collect();
    
    if failed_tests.is_empty() {
        println!("✅ All performance tests passed!");
    } else {
        println!("❌ Performance tests failed for: {:?}", failed_tests);
        println!("Performance requirements not met - requires optimization");
    }
    
    let all_passed = failed_tests.is_empty();
    assert!(all_passed, "Performance requirements not met for: {:?}", failed_tests);
}

async fn test_get_paper_detail_performance(repo: &Neo4jRepo, paper_id: &str) -> bool {
    let mut total_duration = Duration::new(0, 0);
    let iterations = 100;
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.get_paper_detail(paper_id).await;
        total_duration += start.elapsed();
        if result.is_err() {
            return false;
        }
        let detail = result.unwrap();
        if detail.is_none() {
            return false;
        }
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("get_paper_detail: {} iterations, avg {:?}, threshold {:?}", iterations, avg_duration, GET_PAPER_DETAIL_THRESHOLD);
    
    avg_duration < GET_PAPER_DETAIL_THRESHOLD
}

async fn test_get_graph_data_performance(repo: &Neo4jRepo, ws_id: &str) -> bool {
    let mut total_duration = Duration::new(0, 0);
    let iterations = 50;
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.get_graph_data(ws_id).await;
        total_duration += start.elapsed();
        if result.is_err() {
            return false;
        }
        let (nodes, links) = result.unwrap();
        if nodes.is_empty() {
            return false;
        }
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("get_graph_data: {} iterations, avg {:?}, threshold {:?}", iterations, avg_duration, GET_GRAPH_DATA_THRESHOLD);
    
    avg_duration < GET_GRAPH_DATA_THRESHOLD
}

async fn test_create_authors_batch_performance(repo: &Neo4jRepo, ws_id: &str, paper_id: &str) -> bool {
    let authors = vec![
        (uuid::Uuid::new_v4().to_string(), "Batch Author A".to_string(), None, true, false),
        (uuid::Uuid::new_v4().to_string(), "Batch Author B".to_string(), None, false, true),
        (uuid::Uuid::new_v4().to_string(), "Batch Author C".to_string(), None, false, false),
        (uuid::Uuid::new_v4().to_string(), "Batch Author D".to_string(), None, false, false),
        (uuid::Uuid::new_v4().to_string(), "Batch Author E".to_string(), None, false, false),
        (uuid::Uuid::new_v4().to_string(), "Batch Author F".to_string(), None, false, false),
        (uuid::Uuid::new_v4().to_string(), "Batch Author G".to_string(), None, false, false),
        (uuid::Uuid::new_v4().to_string(), "Batch Author H".to_string(), None, false, false),
    ];
    
    let mut total_duration = Duration::new(0, 0);
    let iterations = 50;
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.create_authors_batch(&authors, paper_id, ws_id).await;
        total_duration += start.elapsed();
        if result.is_err() {
            return false;
        }
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("create_authors_batch: {} iterations, avg {:?}, threshold {:?}", iterations, avg_duration, CREATE_AUTHORS_BATCH_THRESHOLD);
    
    avg_duration < CREATE_AUTHORS_BATCH_THRESHOLD
}

async fn test_list_papers_performance(repo: &Neo4jRepo, ws_id: &str) -> bool {
    let mut total_duration = Duration::new(0, 0);
    let iterations = 100;
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.list_papers_in_workspace(ws_id).await;
        total_duration += start.elapsed();
        if result.is_err() {
            return false;
        }
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("list_papers_in_workspace: {} iterations, avg {:?}, threshold {:?}", iterations, avg_duration, LIST_PAPERS_THRESHOLD);
    
    avg_duration < LIST_PAPERS_THRESHOLD
}

async fn test_search_by_keyword_performance(repo: &Neo4jRepo, ws_id: &str) -> bool {
    let mut total_duration = Duration::new(0, 0);
    let iterations = 50;
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.search_by_keyword(ws_id, "performance").await;
        total_duration += start.elapsed();
        if result.is_err() {
            return false;
        }
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("search_by_keyword: {} iterations, avg {:?}, threshold {:?}", iterations, avg_duration, SEARCH_BY_KEYWORD_THRESHOLD);
    
    avg_duration < SEARCH_BY_KEYWORD_THRESHOLD
}