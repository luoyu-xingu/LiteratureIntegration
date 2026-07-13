use std::time::{Duration, Instant};

use neo4rs::Graph;
use literature_integration::repositories::neo4j_repo::Neo4jRepo;

async fn setup_test_data(repo: &Neo4jRepo) -> (String, String) {
    let ws_id = uuid::Uuid::new_v4().to_string();
    let paper_id = uuid::Uuid::new_v4().to_string();
    
    repo.create_workspace(&ws_id, "PerformanceOptimizationTest", "", "2025-01-01T00:00:00Z").await.unwrap();
    
    repo.create_paper_if_not_exists(
        &paper_id,
        "Performance Optimization Test Paper with Detailed Abstract",
        Some("10.1234/optimization"),
        None,
        Some("This is a comprehensive abstract for performance optimization testing with many relevant keywords and detailed content"),
        Some(2024),
        Some("Journal of Performance Engineering"),
        "2025-01-01T00:00:00Z",
    ).await.unwrap();
    
    repo.add_paper_to_workspace(&ws_id, &paper_id, "2025-01-01T00:00:00Z").await.unwrap();
    
    let author1_id = uuid::Uuid::new_v4().to_string();
    let author2_id = uuid::Uuid::new_v4().to_string();
    let author3_id = uuid::Uuid::new_v4().to_string();
    
    repo.create_author_if_not_exists(&author1_id, "Optimization Author One", None).await.unwrap();
    repo.create_author_if_not_exists(&author2_id, "Optimization Author Two", None).await.unwrap();
    repo.create_author_if_not_exists(&author3_id, "Optimization Author Three", None).await.unwrap();
    
    repo.link_first_author(&author1_id, &paper_id).await.unwrap();
    repo.link_corresponding_author(&author2_id, &paper_id).await.unwrap();
    repo.link_co_authors(&author1_id, &author2_id, &ws_id).await.unwrap();
    repo.link_co_authors(&author1_id, &author3_id, &ws_id).await.unwrap();
    repo.link_co_authors(&author2_id, &author3_id, &ws_id).await.unwrap();
    
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "performance", &paper_id).await.unwrap();
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "optimization", &paper_id).await.unwrap();
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "benchmark", &paper_id).await.unwrap();
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "testing", &paper_id).await.unwrap();
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "efficiency", &paper_id).await.unwrap();
    
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
        .max_connections(16)
        .fetch_size(2000)
        .build()
        .unwrap();

    Graph::connect(config).await.unwrap()
}

#[tokio::test]
async fn test_performance_optimizations() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (ws_id, paper_id) = setup_test_data(&repo).await;
    
    let mut all_passed = true;
    let mut results = Vec::new();
    
    let (passed, duration) = test_get_paper_detail_optimization(&repo, &paper_id).await;
    results.push(("get_paper_detail", passed, duration));
    all_passed &= passed;
    
    let (passed, duration) = test_get_graph_data_optimization(&repo, &ws_id).await;
    results.push(("get_graph_data", passed, duration));
    all_passed &= passed;
    
    let (passed, duration) = test_create_authors_batch_optimization(&repo, &ws_id, &paper_id).await;
    results.push(("create_authors_batch", passed, duration));
    all_passed &= passed;
    
    let (passed, duration) = test_list_papers_optimization(&repo, &ws_id).await;
    results.push(("list_papers_in_workspace", passed, duration));
    all_passed &= passed;
    
    let (passed, duration) = test_search_by_keyword_optimization(&repo, &ws_id).await;
    results.push(("search_by_keyword", passed, duration));
    all_passed &= passed;
    
    let (passed, duration) = test_search_by_author_optimization(&repo, &ws_id).await;
    results.push(("search_by_author", passed, duration));
    all_passed &= passed;
    
    let (passed, duration) = test_get_papers_detail_batch_optimization(&repo, &ws_id).await;
    results.push(("get_papers_detail_batch", passed, duration));
    all_passed &= passed;
    
    println!("\n=== Performance Optimization Test Results ===");
    for (name, passed, duration) in results {
        println!("{}: {} (avg: {:?})", name, if passed { "PASS" } else { "FAIL" }, duration);
    }
    println!("=============================================\n");
    
    assert!(all_passed, "Some performance optimization tests failed");
}

async fn test_get_paper_detail_optimization(repo: &Neo4jRepo, paper_id: &str) -> (bool, Duration) {
    let iterations = 100;
    let mut total_duration = Duration::new(0, 0);
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.get_paper_detail(paper_id).await;
        total_duration += start.elapsed();
        if result.is_err() || result.unwrap().is_none() {
            return (false, Duration::new(0, 0));
        }
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("  get_paper_detail: {} iterations, avg {:?}", iterations, avg_duration);
    
    (avg_duration < Duration::from_millis(40), avg_duration)
}

async fn test_get_graph_data_optimization(repo: &Neo4jRepo, ws_id: &str) -> (bool, Duration) {
    let iterations = 50;
    let mut total_duration = Duration::new(0, 0);
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.get_graph_data(ws_id).await;
        total_duration += start.elapsed();
        if result.is_err() {
            return (false, Duration::new(0, 0));
        }
        let (nodes, _) = result.unwrap();
        if nodes.is_empty() {
            return (false, Duration::new(0, 0));
        }
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("  get_graph_data: {} iterations, avg {:?}", iterations, avg_duration);
    
    (avg_duration < Duration::from_millis(80), avg_duration)
}

async fn test_create_authors_batch_optimization(repo: &Neo4jRepo, ws_id: &str, paper_id: &str) -> (bool, Duration) {
    let authors = vec![
        (uuid::Uuid::new_v4().to_string(), "Batch Author Alpha".to_string(), None, true, false),
        (uuid::Uuid::new_v4().to_string(), "Batch Author Beta".to_string(), None, false, true),
        (uuid::Uuid::new_v4().to_string(), "Batch Author Gamma".to_string(), None, false, false),
        (uuid::Uuid::new_v4().to_string(), "Batch Author Delta".to_string(), None, false, false),
        (uuid::Uuid::new_v4().to_string(), "Batch Author Epsilon".to_string(), None, false, false),
        (uuid::Uuid::new_v4().to_string(), "Batch Author Zeta".to_string(), None, false, false),
        (uuid::Uuid::new_v4().to_string(), "Batch Author Eta".to_string(), None, false, false),
        (uuid::Uuid::new_v4().to_string(), "Batch Author Theta".to_string(), None, false, false),
    ];
    
    let iterations = 50;
    let mut total_duration = Duration::new(0, 0);
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.create_authors_batch(&authors, paper_id, ws_id).await;
        total_duration += start.elapsed();
        if result.is_err() {
            return (false, Duration::new(0, 0));
        }
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("  create_authors_batch: {} iterations, avg {:?}", iterations, avg_duration);
    
    (avg_duration < Duration::from_millis(60), avg_duration)
}

async fn test_list_papers_optimization(repo: &Neo4jRepo, ws_id: &str) -> (bool, Duration) {
    let iterations = 100;
    let mut total_duration = Duration::new(0, 0);
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.list_papers_in_workspace(ws_id).await;
        total_duration += start.elapsed();
        if result.is_err() {
            return (false, Duration::new(0, 0));
        }
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("  list_papers_in_workspace: {} iterations, avg {:?}", iterations, avg_duration);
    
    (avg_duration < Duration::from_millis(25), avg_duration)
}

async fn test_search_by_keyword_optimization(repo: &Neo4jRepo, ws_id: &str) -> (bool, Duration) {
    let iterations = 50;
    let mut total_duration = Duration::new(0, 0);
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.search_by_keyword(ws_id, "performance").await;
        total_duration += start.elapsed();
        if result.is_err() {
            return (false, Duration::new(0, 0));
        }
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("  search_by_keyword: {} iterations, avg {:?}", iterations, avg_duration);
    
    (avg_duration < Duration::from_millis(40), avg_duration)
}

async fn test_search_by_author_optimization(repo: &Neo4jRepo, ws_id: &str) -> (bool, Duration) {
    let iterations = 50;
    let mut total_duration = Duration::new(0, 0);
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.search_by_author(ws_id, "Author").await;
        total_duration += start.elapsed();
        if result.is_err() {
            return (false, Duration::new(0, 0));
        }
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("  search_by_author: {} iterations, avg {:?}", iterations, avg_duration);
    
    (avg_duration < Duration::from_millis(60), avg_duration)
}

async fn test_get_papers_detail_batch_optimization(repo: &Neo4jRepo, ws_id: &str) -> (bool, Duration) {
    let iterations = 30;
    let mut total_duration = Duration::new(0, 0);
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.get_papers_detail_batch(ws_id, None, None, None).await;
        total_duration += start.elapsed();
        if result.is_err() {
            return (false, Duration::new(0, 0));
        }
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("  get_papers_detail_batch: {} iterations, avg {:?}", iterations, avg_duration);
    
    (avg_duration < Duration::from_millis(80), avg_duration)
}