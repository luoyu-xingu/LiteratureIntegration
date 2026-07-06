use std::time::{Duration, Instant};
use literature_integration::repositories::neo4j_repo::Neo4jRepo;
use neo4rs::{ConfigBuilder, Graph, query};

async fn setup_test_data(repo: &Neo4jRepo) -> (String, String) {
    let ws_id = uuid::Uuid::new_v4().to_string();
    let paper_id = uuid::Uuid::new_v4().to_string();
    
    repo.create_workspace(&ws_id, "PerfTestWorkspace", "Test workspace for performance", "2025-01-01T00:00:00Z").await.unwrap();
    
    repo.create_paper_if_not_exists(
        &paper_id,
        "High Performance Computing in Data Science",
        Some("10.1234/hpc-data-science"),
        None,
        Some("This paper explores high performance computing techniques for data science applications"),
        Some(2024),
        Some("Journal of Data Science"),
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
    
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "performance", &paper_id).await.unwrap();
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "data science", &paper_id).await.unwrap();
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "computing", &paper_id).await.unwrap();
    
    for i in 0..10 {
        let additional_paper_id = uuid::Uuid::new_v4().to_string();
        repo.create_paper_if_not_exists(
            &additional_paper_id,
            &format!("Additional Paper {}", i),
            None,
            None,
            Some(&format!("Abstract for paper {}", i)),
            Some(2024 - (i % 5) as i32),
            Some("Test Journal"),
            "2025-01-01T00:00:00Z",
        ).await.unwrap();
        repo.add_paper_to_workspace(&ws_id, &additional_paper_id, "2025-01-01T00:00:00Z").await.unwrap();
        repo.link_first_author(&author1_id, &additional_paper_id).await.unwrap();
    }
    
    (ws_id, paper_id)
}

async fn measure_time_async<F, T>(f: F) -> (T, Duration)
where
    F: std::future::Future<Output = T>,
{
    let start = Instant::now();
    let result = f.await;
    let duration = start.elapsed();
    (result, duration)
}

async fn try_create_repo() -> Option<Neo4jRepo> {
    let config = match ConfigBuilder::default()
        .uri("bolt://localhost:7687")
        .user("neo4j")
        .password("password")
        .max_connections(4)
        .build()
    {
        Ok(c) => c,
        Err(_) => return None,
    };
    let graph = match Graph::connect(config).await {
        Ok(g) => g,
        Err(_) => return None,
    };
    let test_query = query("RETURN 1");
    match graph.execute(test_query).await {
        Ok(mut result) => {
            let _ = result.next().await;
            Some(Neo4jRepo::new(graph))
        }
        Err(_) => None,
    }
}

#[tokio::test]
async fn test_performance_optimizations() {
    let repo = match try_create_repo().await {
        Some(r) => r,
        None => {
            println!("SKIP: Neo4j not available, skipping performance tests");
            return;
        }
    };
    
    let (ws_id, paper_id) = setup_test_data(&repo).await;
    
    let (result, duration) = measure_time_async(repo.get_paper_detail(&paper_id)).await;
    assert!(result.is_ok());
    let detail = result.unwrap().unwrap();
    assert!(!detail.0.title.is_empty());
    assert!(detail.1.is_some());
    assert!(detail.2.is_some());
    assert!(detail.3.len() >= 3);
    println!("get_paper_detail took: {:?}", duration);
    assert!(duration < Duration::from_millis(500), "get_paper_detail should complete in under 500ms, took {:?}", duration);
    
    let (result, duration) = measure_time_async(repo.search_by_keyword(&ws_id, "performance")).await;
    assert!(result.is_ok());
    let papers = result.unwrap();
    assert!(!papers.is_empty());
    println!("search_by_keyword took: {:?}", duration);
    assert!(duration < Duration::from_millis(500), "search_by_keyword should complete in under 500ms, took {:?}", duration);
    
    let (result, duration) = measure_time_async(repo.search_by_author(&ws_id, "John")).await;
    assert!(result.is_ok());
    let authors = result.unwrap();
    assert!(!authors.is_empty());
    println!("search_by_author took: {:?}", duration);
    assert!(duration < Duration::from_millis(500), "search_by_author should complete in under 500ms, took {:?}", duration);
    
    let (result, duration) = measure_time_async(repo.get_graph_data(&ws_id)).await;
    assert!(result.is_ok());
    let (nodes, _links) = result.unwrap();
    assert!(!nodes.is_empty());
    println!("get_graph_data took: {:?}", duration);
    assert!(duration < Duration::from_millis(500), "get_graph_data should complete in under 500ms, took {:?}", duration);
    
    let (result, duration) = measure_time_async(repo.list_papers_in_workspace(&ws_id)).await;
    assert!(result.is_ok());
    let papers = result.unwrap();
    assert!(papers.len() >= 10);
    println!("list_papers_in_workspace took: {:?}", duration);
    assert!(duration < Duration::from_millis(500), "list_papers_in_workspace should complete in under 500ms, took {:?}", duration);
}