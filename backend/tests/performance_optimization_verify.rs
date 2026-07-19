use std::time::{Duration, Instant};
use std::sync::Arc;

use neo4rs::Graph;
use literature_integration::repositories::neo4j_repo::Neo4jRepo;

async fn setup_test_data(repo: &Neo4jRepo) -> (String, String) {
    let ws_id = uuid::Uuid::new_v4().to_string();
    let paper_id = uuid::Uuid::new_v4().to_string();
    
    repo.create_workspace(&ws_id, "OptimizationTest", "", "2025-01-01T00:00:00Z").await.unwrap();
    
    repo.create_paper_if_not_exists(
        &paper_id,
        "High Performance Computing Optimization Techniques",
        Some("10.1234/optimization"),
        None,
        Some("This paper explores various optimization techniques for high performance computing systems, including parallel processing and distributed algorithms."),
        Some(2024),
        Some("Journal of Performance Engineering"),
        "2025-01-01T00:00:00Z",
    ).await.unwrap();
    
    repo.add_paper_to_workspace(&ws_id, &paper_id, "2025-01-01T00:00:00Z").await.unwrap();
    
    let author1_id = uuid::Uuid::new_v4().to_string();
    let author2_id = uuid::Uuid::new_v4().to_string();
    
    repo.create_author_if_not_exists(&author1_id, "John Smith", None).await.unwrap();
    repo.create_author_if_not_exists(&author2_id, "Jane Doe", None).await.unwrap();
    
    repo.link_first_author(&author1_id, &paper_id).await.unwrap();
    repo.link_corresponding_author(&author2_id, &paper_id).await.unwrap();
    repo.link_co_authors(&author1_id, &author2_id, &ws_id).await.unwrap();
    
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "optimization", &paper_id).await.unwrap();
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "performance", &paper_id).await.unwrap();
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "computing", &paper_id).await.unwrap();
    
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
async fn verify_optimization_performance() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (ws_id, paper_id) = setup_test_data(&repo).await;
    
    let mut all_passed = true;
    
    let result = test_search_performance(&repo, &ws_id).await;
    println!("search_by_keyword: {}", if result { "PASS" } else { "FAIL" });
    all_passed &= result;
    
    let result = test_paper_detail_performance(&repo, &paper_id).await;
    println!("get_paper_detail: {}", if result { "PASS" } else { "FAIL" });
    all_passed &= result;
    
    let result = test_graph_data_performance(&repo, &ws_id).await;
    println!("get_graph_data: {}", if result { "PASS" } else { "FAIL" });
    all_passed &= result;
    
    let result = test_concurrent_queries_performance(&repo, &ws_id).await;
    println!("concurrent_queries: {}", if result { "PASS" } else { "FAIL" });
    all_passed &= result;
    
    assert!(all_passed, "Performance optimization verification failed");
}

async fn test_search_performance(repo: &Neo4jRepo, ws_id: &str) -> bool {
    let search_terms = vec!["optimization", "performance", "computing", "Smith"];
    
    let mut total_duration = Duration::new(0, 0);
    let iterations = 50;
    
    for _ in 0..iterations {
        for term in &search_terms {
            let start = Instant::now();
            let result = repo.search_by_keyword(ws_id, term).await;
            total_duration += start.elapsed();
            if result.is_err() {
                return false;
            }
        }
    }
    
    let avg_duration = total_duration / (iterations * search_terms.len()) as u32;
    println!("  search_by_keyword: {} iterations, avg {:?}", iterations * search_terms.len(), avg_duration);
    
    avg_duration < Duration::from_millis(30)
}

async fn test_paper_detail_performance(repo: &Neo4jRepo, paper_id: &str) -> bool {
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
    println!("  get_paper_detail: {} iterations, avg {:?}", iterations, avg_duration);
    
    avg_duration < Duration::from_millis(30)
}

async fn test_graph_data_performance(repo: &Neo4jRepo, ws_id: &str) -> bool {
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
    println!("  get_graph_data: {} iterations, avg {:?}", iterations, avg_duration);
    
    avg_duration < Duration::from_millis(60)
}

async fn test_concurrent_queries_performance(repo: &Neo4jRepo, ws_id: &str) -> bool {
    let repo_clone = repo.clone();
    let ws_id_clone = ws_id.to_string();
    
    let tasks: Vec<_> = (0..10).map(|_| {
        let repo = repo_clone.clone();
        let ws_id = ws_id_clone.clone();
        tokio::spawn(async move {
            let start = Instant::now();
            for _ in 0..10 {
                let _ = repo.list_papers_in_workspace(&ws_id).await;
            }
            start.elapsed()
        })
    }).collect();
    
    let mut max_duration = Duration::new(0, 0);
    for task in tasks {
        let duration = task.await.unwrap();
        if duration > max_duration {
            max_duration = duration;
        }
    }
    
    println!("  concurrent_queries: 10 concurrent tasks, max {:?}", max_duration);
    
    max_duration < Duration::from_millis(200)
}