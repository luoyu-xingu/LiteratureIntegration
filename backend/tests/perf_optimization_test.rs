use std::time::{Duration, Instant};

use neo4rs::Graph;
use literature_integration::repositories::neo4j_repo::Neo4jRepo;

async fn setup_test_data(repo: &Neo4jRepo) -> (String, String) {
    let ws_id = uuid::Uuid::new_v4().to_string();
    let paper_id = uuid::Uuid::new_v4().to_string();
    
    repo.create_workspace(&ws_id, "PerfOptTest", "", "2025-01-01T00:00:00Z").await.unwrap();
    
    repo.create_paper_if_not_exists(
        &paper_id,
        "Performance Optimization Test Paper",
        Some("10.1234/perf-opt"),
        None,
        Some("Abstract for performance optimization testing"),
        Some(2024),
        Some("Performance Optimization Journal"),
        "2025-01-01T00:00:00Z",
    ).await.unwrap();
    
    repo.add_paper_to_workspace(&ws_id, &paper_id, "2025-01-01T00:00:00Z").await.unwrap();
    
    let author1_id = uuid::Uuid::new_v4().to_string();
    let author2_id = uuid::Uuid::new_v4().to_string();
    
    repo.create_author_if_not_exists(&author1_id, "Opt Author One", None).await.unwrap();
    repo.create_author_if_not_exists(&author2_id, "Opt Author Two", None).await.unwrap();
    
    repo.link_first_author(&author1_id, &paper_id).await.unwrap();
    repo.link_corresponding_author(&author2_id, &paper_id).await.unwrap();
    repo.link_co_authors(&author1_id, &author2_id, &ws_id).await.unwrap();
    
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "optimization", &paper_id).await.unwrap();
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "performance", &paper_id).await.unwrap();
    
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
        .fetch_size(2000)
        .build()
        .unwrap();

    Graph::connect(config).await.unwrap()
}

#[tokio::test]
async fn test_optimized_get_paper_detail_performance() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (_ws_id, paper_id) = setup_test_data(&repo).await;
    
    let mut total_duration = Duration::new(0, 0);
    let iterations = 100;
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.get_paper_detail(&paper_id).await;
        total_duration += start.elapsed();
        assert!(result.is_ok());
        let detail = result.unwrap();
        assert!(detail.is_some());
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("optimized get_paper_detail: {} iterations, avg {:?}", iterations, avg_duration);
    
    assert!(avg_duration < Duration::from_millis(45), 
        "optimized get_paper_detail average duration {:?} exceeds 45ms (improvement expected)", avg_duration);
}

#[tokio::test]
async fn test_optimized_get_graph_data_performance() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (ws_id, _paper_id) = setup_test_data(&repo).await;
    
    let mut total_duration = Duration::new(0, 0);
    let iterations = 50;
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.get_graph_data(&ws_id).await;
        total_duration += start.elapsed();
        assert!(result.is_ok());
        let (nodes, links) = result.unwrap();
        assert!(!nodes.is_empty());
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("optimized get_graph_data: {} iterations, avg {:?}", iterations, avg_duration);
    
    assert!(avg_duration < Duration::from_millis(80), 
        "optimized get_graph_data average duration {:?} exceeds 80ms (improvement expected)", avg_duration);
}

#[tokio::test]
async fn test_optimized_create_authors_batch_performance() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (ws_id, paper_id) = setup_test_data(&repo).await;
    
    let authors = vec![
        (uuid::Uuid::new_v4().to_string(), "Opt Batch Author 1".to_string(), None, true, false),
        (uuid::Uuid::new_v4().to_string(), "Opt Batch Author 2".to_string(), None, false, true),
        (uuid::Uuid::new_v4().to_string(), "Opt Batch Author 3".to_string(), None, false, false),
        (uuid::Uuid::new_v4().to_string(), "Opt Batch Author 4".to_string(), None, false, false),
        (uuid::Uuid::new_v4().to_string(), "Opt Batch Author 5".to_string(), None, false, false),
    ];
    
    let mut total_duration = Duration::new(0, 0);
    let iterations = 50;
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.create_authors_batch(&authors, &paper_id, &ws_id).await;
        total_duration += start.elapsed();
        assert!(result.is_ok());
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("optimized create_authors_batch: {} iterations, avg {:?}", iterations, avg_duration);
    
    assert!(avg_duration < Duration::from_millis(60), 
        "optimized create_authors_batch average duration {:?} exceeds 60ms (improvement expected)", avg_duration);
}

#[tokio::test]
async fn test_optimized_list_papers_performance() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (ws_id, _paper_id) = setup_test_data(&repo).await;
    
    let mut total_duration = Duration::new(0, 0);
    let iterations = 100;
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.list_papers_in_workspace(&ws_id).await;
        total_duration += start.elapsed();
        assert!(result.is_ok());
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("optimized list_papers_in_workspace: {} iterations, avg {:?}", iterations, avg_duration);
    
    assert!(avg_duration < Duration::from_millis(25), 
        "optimized list_papers_in_workspace average duration {:?} exceeds 25ms (improvement expected)", avg_duration);
}

#[tokio::test]
async fn test_optimized_search_by_keyword_performance() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (ws_id, _paper_id) = setup_test_data(&repo).await;
    
    let mut total_duration = Duration::new(0, 0);
    let iterations = 50;
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.search_by_keyword(&ws_id, "optimization").await;
        total_duration += start.elapsed();
        assert!(result.is_ok());
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("optimized search_by_keyword: {} iterations, avg {:?}", iterations, avg_duration);
    
    assert!(avg_duration < Duration::from_millis(40), 
        "optimized search_by_keyword average duration {:?} exceeds 40ms (improvement expected)", avg_duration);
}

#[tokio::test]
async fn test_optimized_list_workspaces_performance() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let _ws_id = setup_test_data(&repo).await.0;
    
    let mut total_duration = Duration::new(0, 0);
    let iterations = 100;
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.list_workspaces().await;
        total_duration += start.elapsed();
        assert!(result.is_ok());
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("optimized list_workspaces: {} iterations, avg {:?}", iterations, avg_duration);
    
    assert!(avg_duration < Duration::from_millis(25), 
        "optimized list_workspaces average duration {:?} exceeds 25ms (improvement expected)", avg_duration);
}