use std::time::{Instant, Duration};
use literature_integration::repositories::neo4j_repo::Neo4jRepo;
use crate::common::{spawn_neo4j, app};
use axum::body::Body;
use tower::util::ServiceExt;
use http_body_util::BodyExt;
use hyper::{Request, StatusCode};
use serde_json::json;

mod common;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_graph_data_performance() {
        let graph = spawn_neo4j().await;
        let repo = Neo4jRepo::new(graph.clone());

        let ws = repo
            .create_workspace("perf-test-ws", "PerfTestWorkspace", "Performance test", "2025-01-01T00:00:00Z")
            .await
            .unwrap();

        for i in 0..10 {
            let paper = repo
                .create_paper_if_not_exists(
                    &format!("perf-paper-{}", i),
                    &format!("Performance Test Paper {}", i),
                    Some(&format!("10.1234/perf{}", i)),
                    None,
                    Some(&format!("Abstract for paper {}", i)),
                    Some(2024),
                    Some("Performance Journal"),
                    "2025-01-01T00:00:00Z",
                )
                .await
                .unwrap();
            repo.add_paper_to_workspace(&ws.id, &paper.id, "2025-01-01T00:00:00Z")
                .await
                .unwrap();
            
            let author = repo
                .create_author_if_not_exists(&format!("perf-author-{}", i), &format!("Author {}", i), None)
                .await
                .unwrap();
            repo.link_first_author(&author.id, &paper.id)
                .await
                .unwrap();
        }

        let start = Instant::now();
        let result = repo.get_graph_data(&ws.id).await.unwrap();
        let elapsed = start.elapsed();

        assert!(!result.0.is_empty());
        assert!(elapsed < Duration::from_secs(5), "Graph data query took too long: {:?}", elapsed);
    }

    #[tokio::test]
    async fn test_search_performance() {
        let graph = spawn_neo4j().await;
        let repo = Neo4jRepo::new(graph.clone());

        let ws = repo
            .create_workspace("search-perf-ws", "SearchPerfWorkspace", "Search performance test", "2025-01-01T00:00:00Z")
            .await
            .unwrap();

        for i in 0..50 {
            let paper = repo
                .create_paper_if_not_exists(
                    &format!("search-paper-{}", i),
                    &format!("Search Test Paper {}", i),
                    Some(&format!("10.1234/search{}", i)),
                    None,
                    Some("This paper contains important research findings about search optimization"),
                    Some(2024),
                    Some("Search Journal"),
                    "2025-01-01T00:00:00Z",
                )
                .await
                .unwrap();
            repo.add_paper_to_workspace(&ws.id, &paper.id, "2025-01-01T00:00:00Z")
                .await
                .unwrap();
        }

        let start = Instant::now();
        let results = repo.search_by_keyword(&ws.id, "search").await.unwrap();
        let elapsed = start.elapsed();

        assert!(!results.is_empty());
        assert!(elapsed < Duration::from_secs(3), "Search query took too long: {:?}", elapsed);
    }

    #[tokio::test]
    async fn test_list_papers_performance() {
        let graph = spawn_neo4j().await;
        let repo = Neo4jRepo::new(graph.clone());

        let ws = repo
            .create_workspace("list-perf-ws", "ListPerfWorkspace", "List performance test", "2025-01-01T00:00:00Z")
            .await
            .unwrap();

        for i in 0..100 {
            let paper = repo
                .create_paper_if_not_exists(
                    &format!("list-paper-{}", i),
                    &format!("List Test Paper {}", i),
                    Some(&format!("10.1234/list{}", i)),
                    None,
                    Some(&format!("Abstract for list paper {}", i)),
                    Some(2024),
                    Some("List Journal"),
                    "2025-01-01T00:00:00Z",
                )
                .await
                .unwrap();
            repo.add_paper_to_workspace(&ws.id, &paper.id, "2025-01-01T00:00:00Z")
                .await
                .unwrap();
        }

        let start = Instant::now();
        let papers = repo.list_papers_in_workspace(&ws.id).await.unwrap();
        let elapsed = start.elapsed();

        assert_eq!(papers.len(), 100);
        assert!(elapsed < Duration::from_secs(2), "List papers query took too long: {:?}", elapsed);
    }

    #[tokio::test]
    async fn test_api_response_time() {
        let graph = spawn_neo4j().await;
        let app = app(graph.clone());
        let repo = Neo4jRepo::new(graph);

        let create_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/workspaces")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_string(&json!({
                            "name": "API Perf Test",
                            "description": "API performance test"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(create_resp.status(), StatusCode::OK);
        let create_body = create_resp.into_body().collect().await.unwrap().to_bytes();
        let created: serde_json::Value = serde_json::from_slice(&create_body).unwrap();
        let ws_id = created["id"].as_str().unwrap();

        let start = Instant::now();
        let response = app
            .oneshot(
                Request::builder()
                    .uri(&format!("/api/workspace/{}", ws_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let elapsed = start.elapsed();

        assert_eq!(response.status(), StatusCode::OK);
        assert!(elapsed < Duration::from_secs(1), "API response took too long: {:?}", elapsed);
    }
}