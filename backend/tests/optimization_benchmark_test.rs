use std::time::{Duration, Instant};

use literature_integration::repositories::external_api::{extract_xml_tag, extract_xml_tags};

const SAMPLE_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <entry>
    <id>http://arxiv.org/abs/1234.56789v1</id>
    <title>Sample Paper Title</title>
    <summary>This is a sample abstract text.</summary>
    <published>2024-01-15T00:00:00Z</published>
    <author>
      <name>John Doe</name>
    </author>
    <author>
      <name>Jane Smith</name>
    </author>
    <author>
      <name>Bob Johnson</name>
    </author>
    <author>
      <name>Alice Williams</name>
    </author>
    <author>
      <name>Charlie Brown</name>
    </author>
    <author>
      <name>Eve Davis</name>
    </author>
  </entry>
</feed>"#;

#[test]
fn test_extract_xml_tag_performance() {
    let iterations = 10000;
    let mut total_duration = Duration::new(0, 0);
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = extract_xml_tag(SAMPLE_XML, "title");
        total_duration += start.elapsed();
        assert_eq!(result, Some("Sample Paper Title".to_string()));
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("extract_xml_tag: {} iterations, avg {:?}", iterations, avg_duration);
    
    assert!(avg_duration < Duration::from_micros(50), 
        "extract_xml_tag avg duration {:?} exceeds 50us", avg_duration);
}

#[test]
fn test_extract_xml_tags_performance() {
    let iterations = 10000;
    let mut total_duration = Duration::new(0, 0);
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = extract_xml_tags(SAMPLE_XML, "name");
        total_duration += start.elapsed();
        assert_eq!(result.len(), 6);
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("extract_xml_tags: {} iterations, avg {:?}", iterations, avg_duration);
    
    assert!(avg_duration < Duration::from_micros(100), 
        "extract_xml_tags avg duration {:?} exceeds 100us", avg_duration);
}

#[test]
fn test_string_allocation_optimization() {
    let iterations = 10000;
    let mut total_duration = Duration::new(0, 0);
    
    for _ in 0..iterations {
        let start = Instant::now();
        let _ = format!("arXiv:{}", "1234.56789");
        let _ = format!("https://api.crossref.org/works/{}", "10.1234/test");
        total_duration += start.elapsed();
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("string_format: {} iterations, avg {:?}", iterations, avg_duration);
    
    assert!(avg_duration < Duration::from_micros(30), 
        "string format avg duration {:?} exceeds 30us", avg_duration);
}

async fn get_graph() -> Result<neo4rs::Graph, String> {
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
        .map_err(|e| format!("Failed to build config: {}", e))?;

    neo4rs::Graph::connect(config).await.map_err(|e| format!("Failed to connect: {}", e))
}

async fn setup_neo4j_test(repo: &literature_integration::repositories::neo4j_repo::Neo4jRepo) -> Result<(String, String), String> {
    let ws_id = uuid::Uuid::new_v4().to_string();
    let paper_id = uuid::Uuid::new_v4().to_string();
    
    repo.create_workspace(&ws_id, "BenchmarkTest", "", "2025-01-01T00:00:00Z").await.map_err(|e| format!("Failed to create workspace: {}", e))?;
    
    repo.create_paper_if_not_exists(
        &paper_id,
        "Benchmark Test Paper",
        Some("10.1234/bench"),
        None,
        Some("Abstract for benchmark testing"),
        Some(2024),
        Some("Benchmark Journal"),
        "2025-01-01T00:00:00Z",
    ).await.map_err(|e| format!("Failed to create paper: {}", e))?;
    
    repo.add_paper_to_workspace(&ws_id, &paper_id, "2025-01-01T00:00:00Z").await.map_err(|e| format!("Failed to add paper to workspace: {}", e))?;
    
    let author1_id = uuid::Uuid::new_v4().to_string();
    let author2_id = uuid::Uuid::new_v4().to_string();
    
    repo.create_author_if_not_exists(&author1_id, "Bench Author One", None).await.map_err(|e| format!("Failed to create author: {}", e))?;
    repo.create_author_if_not_exists(&author2_id, "Bench Author Two", None).await.map_err(|e| format!("Failed to create author: {}", e))?;
    
    repo.link_first_author(&author1_id, &paper_id).await.map_err(|e| format!("Failed to link first author: {}", e))?;
    repo.link_corresponding_author(&author2_id, &paper_id).await.map_err(|e| format!("Failed to link corresponding author: {}", e))?;
    
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "benchmark", &paper_id).await.map_err(|e| format!("Failed to add keyword: {}", e))?;
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "testing", &paper_id).await.map_err(|e| format!("Failed to add keyword: {}", e))?;
    
    Ok((ws_id, paper_id))
}

#[tokio::test]
async fn test_neo4j_query_performance() {
    let graph_result = get_graph().await;
    if graph_result.is_err() {
        println!("get_graph: SKIP (no Neo4j connection)");
        return;
    }
    
    let graph = graph_result.unwrap();
    let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph);
    
    let setup_result = setup_neo4j_test(&repo).await;
    if setup_result.is_err() {
        println!("setup_neo4j_test: SKIP (setup failed)");
        return;
    }
    
    let (ws_id, paper_id) = setup_result.unwrap();
    
    let mut all_passed = true;
    
    let result = test_get_paper_detail(&repo, &paper_id).await;
    println!("get_paper_detail: {}", if result { "PASS" } else { "FAIL" });
    all_passed &= result;
    
    let result = test_search_by_keyword(&repo, &ws_id).await;
    println!("search_by_keyword: {}", if result { "PASS" } else { "FAIL" });
    all_passed &= result;
    
    let result = test_get_graph_data(&repo, &ws_id).await;
    println!("get_graph_data: {}", if result { "PASS" } else { "FAIL" });
    all_passed &= result;
    
    assert!(all_passed, "Some Neo4j query performance tests failed");
}

async fn test_get_paper_detail(repo: &literature_integration::repositories::neo4j_repo::Neo4jRepo, paper_id: &str) -> bool {
    let iterations = 50;
    let mut total_duration = Duration::new(0, 0);
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.get_paper_detail(paper_id).await;
        total_duration += start.elapsed();
        if result.is_err() || result.unwrap().is_none() {
            return false;
        }
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("  get_paper_detail: {} iterations, avg {:?}", iterations, avg_duration);
    
    avg_duration < Duration::from_millis(100)
}

async fn test_search_by_keyword(repo: &literature_integration::repositories::neo4j_repo::Neo4jRepo, ws_id: &str) -> bool {
    let iterations = 50;
    let mut total_duration = Duration::new(0, 0);
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.search_by_keyword(ws_id, "benchmark").await;
        total_duration += start.elapsed();
        if result.is_err() {
            return false;
        }
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("  search_by_keyword: {} iterations, avg {:?}", iterations, avg_duration);
    
    avg_duration < Duration::from_millis(100)
}

async fn test_get_graph_data(repo: &literature_integration::repositories::neo4j_repo::Neo4jRepo, ws_id: &str) -> bool {
    let iterations = 50;
    let mut total_duration = Duration::new(0, 0);
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = repo.get_graph_data(ws_id).await;
        total_duration += start.elapsed();
        if result.is_err() {
            return false;
        }
    }
    
    let avg_duration = total_duration / iterations as u32;
    println!("  get_graph_data: {} iterations, avg {:?}", iterations, avg_duration);
    
    avg_duration < Duration::from_millis(150)
}