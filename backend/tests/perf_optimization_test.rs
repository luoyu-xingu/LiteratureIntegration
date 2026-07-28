use literature_integration::repositories::neo4j_repo::Neo4jRepo;
use literature_integration::models::dto::{ExportRequest, GraphNode, GraphLink};
use std::time::Instant;

async fn setup_test_data(repo: &Neo4jRepo) -> (String, Vec<String>) {
    let ws_id = uuid::Uuid::new_v4().to_string();
    repo.create_workspace(&ws_id, "PerfTestWS", "Performance test workspace", "2025-01-01T00:00:00Z").await.unwrap();

    let mut paper_ids = Vec::new();
    for i in 0..20 {
        let paper_id = uuid::Uuid::new_v4().to_string();
        repo.create_paper_if_not_exists(
            &paper_id,
            &format!("Performance Test Paper {}", i),
            Some(&format!("10.1234/perf.{}", i)),
            None,
            Some(&format!("Abstract for performance testing paper {}. This is a longer abstract to simulate real data.", i)),
            Some(2020 + (i % 5)),
            Some("Performance Journal"),
            "2025-01-01T00:00:00Z",
        ).await.unwrap();
        repo.add_paper_to_workspace(&ws_id, &paper_id, "2025-01-01T00:00:00Z").await.unwrap();

        let author_id = uuid::Uuid::new_v4().to_string();
        let author2_id = uuid::Uuid::new_v4().to_string();
        repo.create_author_if_not_exists(&author_id, &format!("Author First {}", i), None).await.unwrap();
        repo.create_author_if_not_exists(&author2_id, &format!("Author Corr {}", i), None).await.unwrap();
        repo.link_first_author(&author_id, &paper_id).await.unwrap();
        repo.link_corresponding_author(&author2_id, &paper_id).await.unwrap();
        repo.link_co_authors(&author_id, &author2_id, &ws_id).await.unwrap();

        repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "performance", &paper_id).await.unwrap();
        repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "testing", &paper_id).await.unwrap();
        repo.add_keyword(&uuid::Uuid::new_v4().to_string(), &format!("topic{}", i), &paper_id).await.unwrap();

        paper_ids.push(paper_id);
    }

    (ws_id, paper_ids)
}

#[tokio::test]
async fn test_search_by_keyword_performance() {
    let graph = literature_integration::config::create_neo4j_pool(&literature_integration::config::Config::from_env())
        .await
        .unwrap();
    let repo = Neo4jRepo::new(graph);

    let (ws_id, _) = setup_test_data(&repo).await;

    let start = Instant::now();
    let result = repo.search_by_keyword(&ws_id, "performance").await.unwrap();
    let elapsed = start.elapsed();

    assert!(!result.is_empty(), "Search should return results");
    assert!(elapsed.as_millis() < 5000, "Search by keyword took too long: {:?}", elapsed);
    println!("search_by_keyword: {:?}, results: {}", elapsed, result.len());
}

#[tokio::test]
async fn test_search_by_author_performance() {
    let graph = literature_integration::config::create_neo4j_pool(&literature_integration::config::Config::from_env())
        .await
        .unwrap();
    let repo = Neo4jRepo::new(graph);

    let (ws_id, _) = setup_test_data(&repo).await;

    let start = Instant::now();
    let result = repo.search_by_author(&ws_id, "Author").await.unwrap();
    let elapsed = start.elapsed();

    assert!(!result.is_empty(), "Search should return results");
    assert!(elapsed.as_millis() < 5000, "Search by author took too long: {:?}", elapsed);
    println!("search_by_author: {:?}, results: {}", elapsed, result.len());
}

#[tokio::test]
async fn test_get_graph_data_performance() {
    let graph = literature_integration::config::create_neo4j_pool(&literature_integration::config::Config::from_env())
        .await
        .unwrap();
    let repo = Neo4jRepo::new(graph);

    let (ws_id, _) = setup_test_data(&repo).await;

    let start = Instant::now();
    let (nodes, links) = repo.get_graph_data(&ws_id).await.unwrap();
    let elapsed = start.elapsed();

    assert!(!nodes.is_empty(), "Graph should have nodes");
    assert!(elapsed.as_millis() < 5000, "get_graph_data took too long: {:?}", elapsed);
    println!("get_graph_data: {:?}, nodes: {}, links: {}", elapsed, nodes.len(), links.len());
}

#[tokio::test]
async fn test_get_papers_detail_batch_performance() {
    let graph = literature_integration::config::create_neo4j_pool(&literature_integration::config::Config::from_env())
        .await
        .unwrap();
    let repo = Neo4jRepo::new(graph);

    let (ws_id, _) = setup_test_data(&repo).await;

    let start = Instant::now();
    let result = repo.get_papers_detail_batch(&ws_id, None, None, None).await.unwrap();
    let elapsed = start.elapsed();

    assert!(!result.is_empty(), "Should return paper details");
    assert!(elapsed.as_millis() < 5000, "get_papers_detail_batch took too long: {:?}", elapsed);
    println!("get_papers_detail_batch: {:?}, results: {}", elapsed, result.len());
}

#[tokio::test]
async fn test_get_paper_detail_performance() {
    let graph = literature_integration::config::create_neo4j_pool(&literature_integration::config::Config::from_env())
        .await
        .unwrap();
    let repo = Neo4jRepo::new(graph);

    let (ws_id, paper_ids) = setup_test_data(&repo).await;

    let start = Instant::now();
    for paper_id in &paper_ids {
        let _ = repo.get_paper_detail(paper_id).await.unwrap();
    }
    let elapsed = start.elapsed();

    assert!(elapsed.as_millis() < 10000, "get_paper_detail for 20 papers took too long: {:?}", elapsed);
    println!("get_paper_detail (20 papers): {:?}", elapsed);
}

#[tokio::test]
async fn test_list_papers_in_workspace_performance() {
    let graph = literature_integration::config::create_neo4j_pool(&literature_integration::config::Config::from_env())
        .await
        .unwrap();
    let repo = Neo4jRepo::new(graph);

    let (ws_id, _) = setup_test_data(&repo).await;

    let start = Instant::now();
    let result = repo.list_papers_in_workspace(&ws_id).await.unwrap();
    let elapsed = start.elapsed();

    assert_eq!(result.len(), 20, "Should return all 20 papers");
    assert!(elapsed.as_millis() < 5000, "list_papers_in_workspace took too long: {:?}", elapsed);
    println!("list_papers_in_workspace: {:?}, results: {}", elapsed, result.len());
}

#[tokio::test]
async fn test_create_authors_batch_performance() {
    let graph = literature_integration::config::create_neo4j_pool(&literature_integration::config::Config::from_env())
        .await
        .unwrap();
    let repo = Neo4jRepo::new(graph);

    let ws_id = uuid::Uuid::new_v4().to_string();
    let paper_id = uuid::Uuid::new_v4().to_string();
    repo.create_workspace(&ws_id, "BatchTest", "", "2025-01-01T00:00:00Z").await.unwrap();
    repo.create_paper_if_not_exists(
        &paper_id, "Batch Test Paper", None, None, None, Some(2024), None, "2025-01-01T00:00:00Z",
    ).await.unwrap();
    repo.add_paper_to_workspace(&ws_id, &paper_id, "2025-01-01T00:00:00Z").await.unwrap();

    let authors: Vec<(String, String, Option<String>, bool, bool)> = (0..10)
        .map(|i| (
            uuid::Uuid::new_v4().to_string(),
            format!("Batch Author {}", i),
            None,
            i == 0,
            i == 9,
        ))
        .collect();

    let start = Instant::now();
    let result = repo.create_authors_batch(&authors, &paper_id, &ws_id).await.unwrap();
    let elapsed = start.elapsed();

    assert!(result.0.is_some(), "Should have first author");
    assert!(elapsed.as_millis() < 5000, "create_authors_batch took too long: {:?}", elapsed);
    println!("create_authors_batch: {:?}", elapsed);
}

#[tokio::test]
async fn test_export_workspace_performance() {
    let graph = literature_integration::config::create_neo4j_pool(&literature_integration::config::Config::from_env())
        .await
        .unwrap();
    let repo = Neo4jRepo::new(graph);

    let (ws_id, _) = setup_test_data(&repo).await;

    let req = ExportRequest {
        format: "markdown".to_string(),
        group_by: None,
        filter: None,
    };

    let start = Instant::now();
    let result = literature_integration::services::export::ExportService::export_markdown(&repo, &ws_id, req).await.unwrap();
    let elapsed = start.elapsed();

    assert!(!result.is_empty(), "Export should return content");
    assert!(elapsed.as_millis() < 5000, "export_workspace took too long: {:?}", elapsed);
    println!("export_workspace: {:?}, content length: {}", elapsed, result.len());
}

#[tokio::test]
async fn test_get_papers_for_export_performance() {
    let graph = literature_integration::config::create_neo4j_pool(&literature_integration::config::Config::from_env())
        .await
        .unwrap();
    let repo = Neo4jRepo::new(graph);

    let (ws_id, _) = setup_test_data(&repo).await;

    let start = Instant::now();
    let result = repo.get_papers_for_export(&ws_id, None, None, None).await.unwrap();
    let elapsed = start.elapsed();

    assert!(!result.is_empty(), "Should return papers");
    assert!(elapsed.as_millis() < 5000, "get_papers_for_export took too long: {:?}", elapsed);
    println!("get_papers_for_export: {:?}, results: {}", elapsed, result.len());
}

#[tokio::test]
async fn test_list_authors_performance() {
    let graph = literature_integration::config::create_neo4j_pool(&literature_integration::config::Config::from_env())
        .await
        .unwrap();
    let repo = Neo4jRepo::new(graph);

    let (ws_id, _) = setup_test_data(&repo).await;

    let start = Instant::now();
    let result = repo.list_authors_in_workspace(&ws_id).await.unwrap();
    let elapsed = start.elapsed();

    assert!(!result.is_empty(), "Should return authors");
    assert!(elapsed.as_millis() < 5000, "list_authors took too long: {:?}", elapsed);
    println!("list_authors: {:?}, results: {}", elapsed, result.len());
}