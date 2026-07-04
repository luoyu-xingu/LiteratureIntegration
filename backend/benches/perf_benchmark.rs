use criterion::{criterion_group, criterion_main, Criterion};
use literature_integration::repositories::neo4j_repo::Neo4jRepo;
use neo4rs::Graph;

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

fn bench_get_paper_detail(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    rt.block_on(async {
        let graph = Graph::new("neo4j://localhost:7687").await.unwrap();
        let repo = Neo4jRepo::new(graph);
        
        let (_ws_id, paper_id) = setup_test_data(&repo).await;
        
        c.bench_function("get_paper_detail_optimized", |b| {
            b.to_async(&rt).iter(|| async {
                let result = repo.get_paper_detail(&paper_id).await;
                assert!(result.is_ok());
                let detail = result.unwrap();
                assert!(detail.is_some());
            });
        });
    });
}

fn bench_create_authors_batch(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    rt.block_on(async {
        let graph = Graph::new("neo4j://localhost:7687").await.unwrap();
        let repo = Neo4jRepo::new(graph);
        
        let (ws_id, paper_id) = setup_test_data(&repo).await;
        
        let authors = vec![
            (uuid::Uuid::new_v4().to_string(), "Batch Author 1".to_string(), None, true, false),
            (uuid::Uuid::new_v4().to_string(), "Batch Author 2".to_string(), None, false, true),
            (uuid::Uuid::new_v4().to_string(), "Batch Author 3".to_string(), None, false, false),
            (uuid::Uuid::new_v4().to_string(), "Batch Author 4".to_string(), None, false, false),
            (uuid::Uuid::new_v4().to_string(), "Batch Author 5".to_string(), None, false, false),
        ];
        
        c.bench_function("create_authors_batch", |b| {
            b.to_async(&rt).iter(|| async {
                let result = repo.create_authors_batch(&authors, &paper_id, &ws_id).await;
                assert!(result.is_ok());
            });
        });
    });
}

fn bench_get_graph_data(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    rt.block_on(async {
        let graph = Graph::new("neo4j://localhost:7687").await.unwrap();
        let repo = Neo4jRepo::new(graph);
        
        let (ws_id, _paper_id) = setup_test_data(&repo).await;
        
        c.bench_function("get_graph_data_optimized", |b| {
            b.to_async(&rt).iter(|| async {
                let result = repo.get_graph_data(&ws_id).await;
                assert!(result.is_ok());
                let (nodes, links) = result.unwrap();
                assert!(!nodes.is_empty());
            });
        });
    });
}

criterion_group!(benches, bench_get_paper_detail, bench_create_authors_batch, bench_get_graph_data);
criterion_main!(benches);