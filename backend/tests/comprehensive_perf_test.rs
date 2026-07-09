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
        Some("Abstract for performance testing with many keywords"),
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

#[tokio::test]
async fn comprehensive_performance_test() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (ws_id, paper_id) = setup_test_data(&repo).await;
    
    let mut all_passed = true;
    
    let result = test_get_paper_detail_performance(&repo, &paper_id).await;
    println!("get_paper_detail: {}", if result { "PASS" } else { "FAIL" });
    all_passed &= result;
    
    let result = test_get_graph_data_performance(&repo, &ws_id).await;
    println!("get_graph_data: {}", if result { "PASS" } else { "FAIL" });
    all_passed &= result;
    
    let result = test_create_authors_batch_performance(&repo, &ws_id, &paper_id).await;
    println!("create_authors_batch: {}", if result { "PASS" } else { "FAIL" });
    all_passed &= result;
    
    let result = test_list_papers_performance(&repo, &ws_id).await;
    println!("list_papers_in_workspace: {}", if result { "PASS" } else { "FAIL" });
    all_passed &= result;
    
    let result = test_search_by_keyword_performance(&repo, &ws_id).await;
    println!("search_by_keyword: {}", if result { "PASS" } else { "FAIL" });
    all_passed &= result;
    
    assert!(all_passed, "Some performance tests failed");
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
    println!("  {} iterations, avg {:?}", iterations, avg_duration);
    
    avg_duration < Duration::from_millis(50)
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
    println!("  {} iterations, avg {:?}", iterations, avg_duration);
    
    avg_duration < Duration::from_millis(100)
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
    println!("  {} iterations, avg {:?}", iterations, avg_duration);
    
    avg_duration < Duration::from_millis(80)
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
    println!("  {} iterations, avg {:?}", iterations, avg_duration);
    
    avg_duration < Duration::from_millis(30)
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
    println!("  {} iterations, avg {:?}", iterations, avg_duration);
    
    avg_duration < Duration::from_millis(50)
}