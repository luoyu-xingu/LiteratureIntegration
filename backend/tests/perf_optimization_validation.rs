use std::time::{Duration, Instant};

use neo4rs::Graph;
use literature_integration::repositories::neo4j_repo::Neo4jRepo;
use literature_integration::repositories::external_api::{ExternalApiClient, extract_xml_tags};

async fn setup_test_data(repo: &Neo4jRepo) -> (String, String) {
    let ws_id = uuid::Uuid::new_v4().to_string();
    let paper_id = uuid::Uuid::new_v4().to_string();
    
    repo.create_workspace(&ws_id, "PerfTest", "", "2025-01-01T00:00:00Z").await.unwrap();
    
    repo.create_paper_if_not_exists(
        &paper_id,
        "Performance Test Paper with Optimization Keywords",
        Some("10.1234/perf-opt"),
        None,
        Some("Abstract for comprehensive performance testing with optimization focus"),
        Some(2024),
        Some("Performance Optimization Journal"),
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
    
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "optimization", &paper_id).await.unwrap();
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "performance", &paper_id).await.unwrap();
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "benchmark", &paper_id).await.unwrap();
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
        .max_connections(8)
        .fetch_size(1000)
        .build()
        .unwrap();

    Graph::connect(config).await.unwrap()
}

#[tokio::test]
async fn validate_search_by_keyword_optimization() {
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
        let papers = result.unwrap();
        assert!(!papers.is_empty());
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("validate_search_by_keyword_optimization: {} iterations, avg {:?}", iterations, avg_duration);
    
    assert!(avg_duration < Duration::from_millis(40), 
        "search_by_keyword optimization failed: avg duration {:?} exceeds 40ms", avg_duration);
}

#[tokio::test]
async fn validate_get_graph_data_optimization() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (ws_id, _paper_id) = setup_test_data(&repo).await;
    
    let mut total_duration = Duration::new(0, 0);
    let iterations = 30;
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.get_graph_data(&ws_id).await;
        total_duration += start.elapsed();
        assert!(result.is_ok());
        let (nodes, links) = result.unwrap();
        assert!(!nodes.is_empty());
        assert!(!links.is_empty());
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("validate_get_graph_data_optimization: {} iterations, avg {:?}", iterations, avg_duration);
    
    assert!(avg_duration < Duration::from_millis(80), 
        "get_graph_data optimization failed: avg duration {:?} exceeds 80ms", avg_duration);
}

#[tokio::test]
async fn validate_get_paper_detail_optimization() {
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
    println!("validate_get_paper_detail_optimization: {} iterations, avg {:?}", iterations, avg_duration);
    
    assert!(avg_duration < Duration::from_millis(40), 
        "get_paper_detail optimization failed: avg duration {:?} exceeds 40ms", avg_duration);
}

#[tokio::test]
async fn validate_create_authors_batch_optimization() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (ws_id, paper_id) = setup_test_data(&repo).await;
    
    let authors = vec![
        (uuid::Uuid::new_v4().to_string(), "Optimization Batch A".to_string(), None, true, false),
        (uuid::Uuid::new_v4().to_string(), "Optimization Batch B".to_string(), None, false, true),
        (uuid::Uuid::new_v4().to_string(), "Optimization Batch C".to_string(), None, false, false),
        (uuid::Uuid::new_v4().to_string(), "Optimization Batch D".to_string(), None, false, false),
        (uuid::Uuid::new_v4().to_string(), "Optimization Batch E".to_string(), None, false, false),
    ];
    
    let mut total_duration = Duration::new(0, 0);
    let iterations = 30;
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.create_authors_batch(&authors, &paper_id, &ws_id).await;
        total_duration += start.elapsed();
        assert!(result.is_ok());
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("validate_create_authors_batch_optimization: {} iterations, avg {:?}", iterations, avg_duration);
    
    assert!(avg_duration < Duration::from_millis(60), 
        "create_authors_batch optimization failed: avg duration {:?} exceeds 60ms", avg_duration);
}

#[test]
fn validate_extract_xml_tags_optimization() {
    let xml = r#"<feed>
        <entry><name>Author One</name></entry>
        <entry><name>Author Two</name></entry>
        <entry><name>Author Three</name></entry>
        <entry><name>Author Four</name></entry>
        <entry><name>Author Five</name></entry>
    </feed>"#;
    
    let mut total_duration = Duration::new(0, 0);
    let iterations = 1000;
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = extract_xml_tags(xml, "name");
        total_duration += start.elapsed();
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], "Author One");
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("validate_extract_xml_tags_optimization: {} iterations, avg {:?}", iterations, avg_duration);
    
    assert!(avg_duration < Duration::from_micros(50), 
        "extract_xml_tags optimization failed: avg duration {:?} exceeds 50us", avg_duration);
}

#[tokio::test]
async fn validate_list_papers_optimization() {
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
    println!("validate_list_papers_optimization: {} iterations, avg {:?}", iterations, avg_duration);
    
    assert!(avg_duration < Duration::from_millis(25), 
        "list_papers_in_workspace optimization failed: avg duration {:?} exceeds 25ms", avg_duration);
}

#[tokio::test]
async fn validate_search_by_author_optimization() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (ws_id, _paper_id) = setup_test_data(&repo).await;
    
    let mut total_duration = Duration::new(0, 0);
    let iterations = 50;
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.search_by_author(&ws_id, "Optimization").await;
        total_duration += start.elapsed();
        assert!(result.is_ok());
        let authors = result.unwrap();
        assert!(!authors.is_empty());
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("validate_search_by_author_optimization: {} iterations, avg {:?}", iterations, avg_duration);
    
    assert!(avg_duration < Duration::from_millis(50), 
        "search_by_author optimization failed: avg duration {:?} exceeds 50ms", avg_duration);
}

#[test]
fn validate_external_api_client_reuse() {
    let client1 = ExternalApiClient::shared();
    let client2 = ExternalApiClient::shared();
    
    assert!(std::ptr::eq(client1, client2), "ExternalApiClient should be a singleton");
}
