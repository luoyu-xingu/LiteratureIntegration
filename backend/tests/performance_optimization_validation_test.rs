use std::time::{Duration, Instant};

use neo4rs::Graph;
use literature_integration::repositories::neo4j_repo::Neo4jRepo;

const TARGET_GET_PAPER_DETAIL: Duration = Duration::from_millis(30);
const TARGET_GET_GRAPH_DATA: Duration = Duration::from_millis(60);
const TARGET_CREATE_AUTHORS_BATCH: Duration = Duration::from_millis(50);
const TARGET_LIST_PAPERS: Duration = Duration::from_millis(20);
const TARGET_SEARCH_BY_KEYWORD: Duration = Duration::from_millis(30);
const TARGET_SEARCH_BY_AUTHOR: Duration = Duration::from_millis(40);
const TARGET_GET_PAPERS_DETAIL_BATCH: Duration = Duration::from_millis(80);

async fn setup_test_data(repo: &Neo4jRepo) -> (String, String) {
    let ws_id = uuid::Uuid::new_v4().to_string();
    let paper_id = uuid::Uuid::new_v4().to_string();
    
    repo.create_workspace(&ws_id, "PerfValidationTest", "", "2025-01-01T00:00:00Z").await.unwrap();
    
    repo.create_paper_if_not_exists(
        &paper_id,
        "Performance Validation Test Paper",
        Some("10.1234/validation"),
        None,
        Some("Abstract for performance validation testing with comprehensive data"),
        Some(2024),
        Some("Validation Journal"),
        "2025-01-01T00:00:00Z",
    ).await.unwrap();
    
    repo.add_paper_to_workspace(&ws_id, &paper_id, "2025-01-01T00:00:00Z").await.unwrap();
    
    let author1_id = uuid::Uuid::new_v4().to_string();
    let author2_id = uuid::Uuid::new_v4().to_string();
    
    repo.create_author_if_not_exists(&author1_id, "Validation Author One", None).await.unwrap();
    repo.create_author_if_not_exists(&author2_id, "Validation Author Two", None).await.unwrap();
    
    repo.link_first_author(&author1_id, &paper_id).await.unwrap();
    repo.link_corresponding_author(&author2_id, &paper_id).await.unwrap();
    repo.link_co_authors(&author1_id, &author2_id, &ws_id).await.unwrap();
    
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "validation", &paper_id).await.unwrap();
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
        .fetch_size(1000)
        .build()
        .unwrap();

    Graph::connect(config).await.unwrap()
}

async fn measure_performance<F, Fut>(repo: &Neo4jRepo, func: F, iterations: usize, target: Duration, name: &str) -> bool
where
    F: Fn(&Neo4jRepo) -> Fut,
    Fut: std::future::Future<Output = bool>,
{
    let mut total_duration = Duration::new(0, 0);
    let mut success_count = 0;
    
    for _ in 0..iterations {
        let start = Instant::now();
        let success = func(repo).await;
        let elapsed = start.elapsed();
        total_duration += elapsed;
        if success {
            success_count += 1;
        }
    }
    
    let avg_duration = total_duration / iterations as u32;
    let success_rate = success_count as f64 / iterations as f64;
    
    println!("{}:", name);
    println!("  Iterations: {}", iterations);
    println!("  Success rate: {:.1}%", success_rate * 100.0);
    println!("  Average duration: {:?}", avg_duration);
    println!("  Target: {:?}", target);
    println!("  Result: {}", if avg_duration < target && success_rate >= 0.95 { "PASS" } else { "FAIL" });
    
    avg_duration < target && success_rate >= 0.95
}

#[tokio::test]
async fn performance_optimization_validation() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (ws_id, paper_id) = setup_test_data(&repo).await;
    
    let mut all_passed = true;
    
    let paper_id_clone = paper_id.clone();
    let result = measure_performance(
        &repo,
        |r| async move {
            let result = r.get_paper_detail(&paper_id_clone).await;
            result.is_ok() && result.unwrap().is_some()
        },
        100,
        TARGET_GET_PAPER_DETAIL,
        "get_paper_detail",
    ).await;
    all_passed &= result;
    
    let ws_id_clone1 = ws_id.clone();
    let result = measure_performance(
        &repo,
        |r| async move {
            let result = r.get_graph_data(&ws_id_clone1).await;
            result.is_ok() && !result.unwrap().0.is_empty()
        },
        50,
        TARGET_GET_GRAPH_DATA,
        "get_graph_data",
    ).await;
    all_passed &= result;
    
    let ws_id_clone2 = ws_id.clone();
    let paper_id_clone2 = paper_id.clone();
    let result = measure_performance(
        &repo,
        |r| async move {
            let authors = vec![
                (uuid::Uuid::new_v4().to_string(), "Test Author A".to_string(), None, true, false),
                (uuid::Uuid::new_v4().to_string(), "Test Author B".to_string(), None, false, true),
                (uuid::Uuid::new_v4().to_string(), "Test Author C".to_string(), None, false, false),
            ];
            let result = r.create_authors_batch(&authors, &paper_id_clone2, &ws_id_clone2).await;
            result.is_ok()
        },
        50,
        TARGET_CREATE_AUTHORS_BATCH,
        "create_authors_batch",
    ).await;
    all_passed &= result;
    
    let ws_id_clone3 = ws_id.clone();
    let result = measure_performance(
        &repo,
        |r| async move {
            let result = r.list_papers_in_workspace(&ws_id_clone3).await;
            result.is_ok()
        },
        100,
        TARGET_LIST_PAPERS,
        "list_papers_in_workspace",
    ).await;
    all_passed &= result;
    
    let ws_id_clone4 = ws_id.clone();
    let result = measure_performance(
        &repo,
        |r| async move {
            let result = r.search_by_keyword(&ws_id_clone4, "performance").await;
            result.is_ok()
        },
        50,
        TARGET_SEARCH_BY_KEYWORD,
        "search_by_keyword",
    ).await;
    all_passed &= result;
    
    let ws_id_clone5 = ws_id.clone();
    let result = measure_performance(
        &repo,
        |r| async move {
            let result = r.search_by_author(&ws_id_clone5, "Author").await;
            result.is_ok()
        },
        50,
        TARGET_SEARCH_BY_AUTHOR,
        "search_by_author",
    ).await;
    all_passed &= result;
    
    let ws_id_clone6 = ws_id.clone();
    let result = measure_performance(
        &repo,
        |r| async move {
            let result = r.get_papers_detail_batch(&ws_id_clone6, None, None, None).await;
            result.is_ok()
        },
        30,
        TARGET_GET_PAPERS_DETAIL_BATCH,
        "get_papers_detail_batch",
    ).await;
    all_passed &= result;
    
    if !all_passed {
        eprintln!("PERFORMANCE VALIDATION FAILED - Optimization required");
    }
    
    assert!(all_passed, "Performance optimization validation failed - some operations exceed target thresholds");
}