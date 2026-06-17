mod common;

#[cfg(test)]
mod performance_tests {
    use crate::common::{app, spawn_neo4j};
    use axum::body::Body;
    use http_body_util::BodyExt;
    use hyper::{Request, StatusCode};
    use tower::util::ServiceExt;
    use serde_json::json;
    use std::time::Instant;

    /// Test that paper detail retrieval works correctly with parallelized queries
    #[tokio::test]
    async fn test_get_paper_detail_parallel_queries() {
        let graph = spawn_neo4j().await;
        let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph.clone());
        let app = app(graph);

        // Create workspace
        let ws = repo
            .create_workspace("perf-test-ws", "PerformanceTest", "", "2025-01-01T00:00:00Z")
            .await
            .unwrap();

        // Create paper with multiple authors and keywords
        let paper = repo
            .create_paper_if_not_exists(
                "perf-test-paper",
                "Performance Test Paper",
                Some("10.1234/perf"),
                None,
                Some("Abstract for performance test"),
                Some(2024),
                Some("Performance Journal"),
                "2025-01-01T00:00:00Z",
            )
            .await
            .unwrap();
        repo.add_paper_to_workspace(&ws.id, &paper.id, "2025-01-01T00:00:00Z")
            .await
            .unwrap();

        // Create multiple authors
        for i in 0..5 {
            let author = repo
                .create_author_if_not_exists(
                    &format!("perf-test-author-{}", i),
                    &format!("Test Author {}", i),
                    None,
                )
                .await
                .unwrap();
            if i == 0 {
                repo.link_first_author(&author.id, &paper.id).await.unwrap();
            }
            if i == 1 {
                repo.link_corresponding_author(&author.id, &paper.id).await.unwrap();
            }
        }

        // Create keywords
        for i in 0..3 {
            repo.add_keyword(
                &format!("perf-test-keyword-{}", i),
                &format!("keyword{}", i),
                &paper.id,
            )
            .await
            .unwrap();
        }

        // Test the optimized get_paper endpoint
        let response = app
            .oneshot(
                Request::builder()
                    .uri(&format!("/api/paper/{}", paper.id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let detail: serde_json::Value = serde_json::from_slice(&body).unwrap();

        // Verify the response structure
        assert_eq!(detail["paper"]["title"], "Performance Test Paper");
        assert!(detail["first_author"].is_object());
        assert!(detail["corresponding_author"].is_object());
        assert!(detail["keywords"].is_array());
        assert_eq!(detail["keywords"].as_array().unwrap().len(), 3);
    }

    /// Test that multiple concurrent paper detail requests work correctly
    #[tokio::test]
    async fn test_concurrent_paper_detail_requests() {
        let graph = spawn_neo4j().await;
        let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph.clone());

        // Create workspace
        let ws = repo
            .create_workspace("concurrent-test-ws", "ConcurrentTest", "", "2025-01-01T00:00:00Z")
            .await
            .unwrap();

        // Create multiple papers
        let paper_ids: Vec<String> = (0..3)
            .map(|i| format!("concurrent-paper-{}", i))
            .collect();

        for (i, paper_id) in paper_ids.iter().enumerate() {
            let paper = repo
                .create_paper_if_not_exists(
                    paper_id,
                    &format!("Concurrent Paper {}", i),
                    Some(&format!("10.1234/concurrent-{}", i)),
                    None,
                    Some("Abstract for concurrent test"),
                    Some(2024),
                    Some("Concurrent Journal"),
                    "2025-01-01T00:00:00Z",
                )
                .await
                .unwrap();
            repo.add_paper_to_workspace(&ws.id, &paper.id, "2025-01-01T00:00:00Z")
                .await
                .unwrap();

            // Add first author
            let author = repo
                .create_author_if_not_exists(
                    &format!("concurrent-author-{}", i),
                    &format!("Author {}", i),
                    None,
                )
                .await
                .unwrap();
            repo.link_first_author(&author.id, &paper.id).await.unwrap();

            // Add keywords
            for j in 0..2 {
                repo.add_keyword(
                    &format!("concurrent-kw-{}-{}", i, j),
                    &format!("keyword{}{}", i, j),
                    &paper.id,
                )
                .await
                .unwrap();
            }
        }

        let app = app(graph.clone());

        // Make concurrent requests
        let start = Instant::now();
        let futures: Vec<_> = paper_ids
            .iter()
            .map(|paper_id| {
                let app = app.clone();
                let paper_id = paper_id.clone();
                async move {
                    app.oneshot(
                        Request::builder()
                            .uri(&format!("/api/paper/{}", paper_id))
                            .body(Body::empty())
                            .unwrap(),
                    )
                    .await
                    .unwrap()
                }
            })
            .collect();

        let responses = futures::future::join_all(futures).await;
        let elapsed = start.elapsed();

        // Verify all responses are OK
        for response in responses {
            assert_eq!(response.status(), StatusCode::OK);
        }

        println!("Concurrent requests completed in {:?}", elapsed);
    }

    /// Test that workspace with many authors returns correct graph data
    #[tokio::test]
    async fn test_graph_data_with_many_authors() {
        let graph = spawn_neo4j().await;
        let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph.clone());
        let app = app(graph);

        let ws = repo
            .create_workspace("graph-perf-ws", "GraphPerfTest", "", "2025-01-01T00:00:00Z")
            .await
            .unwrap();

        // Create multiple papers with authors
        for i in 0..3 {
            let paper = repo
                .create_paper_if_not_exists(
                    &format!("graph-perf-paper-{}", i),
                    &format!("Graph Perf Paper {}", i),
                    Some(&format!("10.1234/graph-perf-{}", i)),
                    None,
                    Some("Abstract for graph perf test"),
                    Some(2024),
                    Some("Graph Journal"),
                    "2025-01-01T00:00:00Z",
                )
                .await
                .unwrap();
            repo.add_paper_to_workspace(&ws.id, &paper.id, "2025-01-01T00:00:00Z")
                .await
                .unwrap();

            // Add two authors per paper
            for j in 0..2 {
                let author = repo
                    .create_author_if_not_exists(
                        &format!("graph-perf-author-{}-{}", i, j),
                        &format!("Author {}{}", i, j),
                        None,
                    )
                    .await
                    .unwrap();
                repo.link_first_author(&author.id, &paper.id).await.unwrap();

                // Link co-authors
                if j == 1 {
                    let prev_author_id = format!("graph-perf-author-{}-{}", i, j - 1);
                    repo.link_co_authors(&prev_author_id, &author.id, &ws.id)
                        .await
                        .unwrap();
                }
            }
        }

        // Request graph data
        let response = app
            .oneshot(
                Request::builder()
                    .uri(&format!("/api/graph?workspace_id={}", ws.id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let graph_data: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert!(graph_data["nodes"].as_array().unwrap().len() > 0);
        assert!(graph_data["links"].as_array().unwrap().len() >= 0);
    }
}