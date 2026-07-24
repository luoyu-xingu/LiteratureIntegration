use std::time::{Duration, Instant};

use neo4rs::Graph;
use literature_integration::repositories::neo4j_repo::Neo4jRepo;

async fn setup_test_data(repo: &Neo4jRepo) -> (String, String) {
    let ws_id = uuid::Uuid::new_v4().to_string();
    let paper_id = uuid::Uuid::new_v4().to_string();
    
    repo.create_workspace(&ws_id, "PerfTest", "", "2025-01-01T00:00:00Z").await.unwrap();
    
    repo.create_paper_if_not_exists(
        &paper_id,
        "Performance Test Paper",
        Some("10.1234/perf"),
        None,
        Some("Abstract for performance testing"),
        Some(2024),
        Some("Performance Journal"),
        "2025-01-01T00:00:00Z",
    ).await.unwrap();
    
    repo.add_paper_to_workspace(&ws_id, &paper_id, "2025-01-01T00:00:00Z").await.unwrap();
    
    let author1_id = uuid::Uuid::new_v4().to_string();
    let author2_id = uuid::Uuid::new_v4().to_string();
    
    repo.create_author_if_not_exists(&author1_id, "Author One", None).await.unwrap();
    repo.create_author_if_not_exists(&author2_id, "Author Two", None).await.unwrap();
    
    repo.link_first_author(&author1_id, &paper_id).await.unwrap();
    repo.link_corresponding_author(&author2_id, &paper_id).await.unwrap();
    repo.link_co_authors(&author1_id, &author2_id, &ws_id).await.unwrap();
    
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "performance", &paper_id).await.unwrap();
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "testing", &paper_id).await.unwrap();
    
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
        .max_connections(4)
        .fetch_size(1000)
        .build()
        .unwrap();

    Graph::connect(config).await.unwrap()
}

#[derive(Debug, Clone)]
struct PerformanceResult {
    test_name: String,
    iterations: usize,
    avg_duration: Duration,
    threshold: Duration,
    passed: bool,
}

impl PerformanceResult {
    fn new(test_name: &str, iterations: usize, avg_duration: Duration, threshold: Duration) -> Self {
        Self {
            test_name: test_name.to_string(),
            iterations,
            avg_duration,
            threshold,
            passed: avg_duration < threshold,
        }
    }
}

#[tokio::test]
async fn comprehensive_performance_test() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (_ws_id, paper_id) = setup_test_data(&repo).await;
    
    let mut results = Vec::with_capacity(6);
    
    results.push(test_get_paper_detail(&repo, &paper_id).await);
    results.push(test_get_graph_data(&repo, &_ws_id).await);
    results.push(test_create_authors_batch(&repo, &_ws_id, &paper_id).await);
    results.push(test_list_papers(&repo, &_ws_id).await);
    results.push(test_search_by_keyword(&repo, &_ws_id).await);
    results.push(test_search_by_author(&repo, &_ws_id).await);
    
    println!("\n=== Comprehensive Performance Test Results ===");
    let mut all_passed = true;
    for result in &results {
        let status = if result.passed { "PASS" } else { "FAIL" };
        println!("{}: {} | {} iterations | avg {:?} | threshold {:?}",
            status, result.test_name, result.iterations, result.avg_duration, result.threshold);
        if !result.passed {
            all_passed = false;
        }
    }
    
    if !all_passed {
        println!("\n⚠️  Performance thresholds not met! Optimization required.");
        std::process::exit(1);
    } else {
        println!("\n✅  All performance tests passed!");
    }
}

async fn test_get_paper_detail(repo: &Neo4jRepo, paper_id: &str) -> PerformanceResult {
    let mut total_duration = Duration::new(0, 0);
    let iterations = 100;
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.get_paper_detail(paper_id).await;
        total_duration += start.elapsed();
        assert!(result.is_ok());
        let detail = result.unwrap();
        assert!(detail.is_some());
    }
    
    let avg_duration = total_duration / iterations as u32;
    PerformanceResult::new("get_paper_detail", iterations, avg_duration, Duration::from_millis(50))
}

async fn test_get_graph_data(repo: &Neo4jRepo, workspace_id: &str) -> PerformanceResult {
    let mut total_duration = Duration::new(0, 0);
    let iterations = 50;
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.get_graph_data(workspace_id).await;
        total_duration += start.elapsed();
        assert!(result.is_ok());
        let (nodes, links) = result.unwrap();
        assert!(!nodes.is_empty());
    }
    
    let avg_duration = total_duration / iterations as u32;
    PerformanceResult::new("get_graph_data", iterations, avg_duration, Duration::from_millis(100))
}

async fn test_create_authors_batch(repo: &Neo4jRepo, workspace_id: &str, paper_id: &str) -> PerformanceResult {
    let authors = vec![
        (uuid::Uuid::new_v4().to_string(), "Batch Author 1".to_string(), None, true, false),
        (uuid::Uuid::new_v4().to_string(), "Batch Author 2".to_string(), None, false, true),
        (uuid::Uuid::new_v4().to_string(), "Batch Author 3".to_string(), None, false, false),
        (uuid::Uuid::new_v4().to_string(), "Batch Author 4".to_string(), None, false, false),
        (uuid::Uuid::new_v4().to_string(), "Batch Author 5".to_string(), None, false, false),
    ];
    
    let mut total_duration = Duration::new(0, 0);
    let iterations = 50;
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.create_authors_batch(&authors, paper_id, workspace_id).await;
        total_duration += start.elapsed();
        assert!(result.is_ok());
    }
    
    let avg_duration = total_duration / iterations as u32;
    PerformanceResult::new("create_authors_batch", iterations, avg_duration, Duration::from_millis(80))
}

async fn test_list_papers(repo: &Neo4jRepo, workspace_id: &str) -> PerformanceResult {
    let mut total_duration = Duration::new(0, 0);
    let iterations = 100;
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.list_papers_in_workspace(workspace_id).await;
        total_duration += start.elapsed();
        assert!(result.is_ok());
    }
    
    let avg_duration = total_duration / iterations as u32;
    PerformanceResult::new("list_papers_in_workspace", iterations, avg_duration, Duration::from_millis(30))
}

async fn test_search_by_keyword(repo: &Neo4jRepo, workspace_id: &str) -> PerformanceResult {
    let mut total_duration = Duration::new(0, 0);
    let iterations = 50;
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.search_by_keyword(workspace_id, "performance").await;
        total_duration += start.elapsed();
        assert!(result.is_ok());
    }
    
    let avg_duration = total_duration / iterations as u32;
    PerformanceResult::new("search_by_keyword", iterations, avg_duration, Duration::from_millis(50))
}

async fn test_search_by_author(repo: &Neo4jRepo, workspace_id: &str) -> PerformanceResult {
    let mut total_duration = Duration::new(0, 0);
    let iterations = 50;
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.search_by_author(workspace_id, "Author").await;
        total_duration += start.elapsed();
        assert!(result.is_ok());
    }
    
    let avg_duration = total_duration / iterations as u32;
    PerformanceResult::new("search_by_author", iterations, avg_duration, Duration::from_millis(60))
}