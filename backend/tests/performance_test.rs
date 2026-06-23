mod common;

#[cfg(test)]
mod tests {
    use crate::common::{app, spawn_neo4j};
    use axum::body::Body;
    use axum::Router;
    use http_body_util::BodyExt;
    use hyper::{Request, StatusCode};
    use std::time::{Instant};
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_paper_detail_performance() {
        let graph = spawn_neo4j().await;
        let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph.clone());
        let app = app(graph);

        let ws = repo
            .create_workspace("perf-ws", "PerfTest", "", "2025-01-01T00:00:00Z")
            .await
            .unwrap();
        let paper = repo
            .create_paper_if_not_exists(
                "perf-paper",
                "Performance Test Paper",
                Some("10.1234/perf"),
                None,
                Some("Abstract for performance testing"),
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
            .create_author_if_not_exists("perf-author", "Performance Author", None)
            .await
            .unwrap();
        repo.link_first_author(&author.id, &paper.id)
            .await
            .unwrap();
        repo.link_corresponding_author(&author.id, &paper.id)
            .await
            .unwrap();
        repo.add_keyword("perf-keyword", "performance", &paper.id)
            .await
            .unwrap();

        let iterations = 10;
        let mut total_duration = std::time::Duration::new(0, 0);

        for _ in 0..iterations {
            let start = Instant::now();
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri(&format!("/api/paper/{}", paper.id))
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            let duration = start.elapsed();
            total_duration += duration;

            assert_eq!(response.status(), StatusCode::OK);
            let body = response.into_body().collect().await.unwrap().to_bytes();
            let _detail: serde_json::Value = serde_json::from_slice(&body).unwrap();
        }

        let avg_duration = total_duration / iterations as u32;
        println!("Average paper detail request time: {:?}", avg_duration);
        
        assert!(avg_duration < std::time::Duration::from_millis(50), 
            "Average request time should be under 50ms, got {:?}", avg_duration);
    }

    #[tokio::test]
    async fn test_search_performance() {
        let graph = spawn_neo4j().await;
        let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph.clone());
        let app = app(graph);

        let ws = repo
            .create_workspace("search-perf-ws", "SearchPerfTest", "", "2025-01-01T00:00:00Z")
            .await
            .unwrap();

        for i in 0..5 {
            let paper = repo
                .create_paper_if_not_exists(
                    &format!("search-paper-{}", i),
                    &format!("Search Test Paper {}", i),
                    Some(&format!("10.1234/search{}", i)),
                    None,
                    Some(&format!("Abstract containing test keyword {}", i)),
                    Some(2024),
                    Some("Search Journal"),
                    "2025-01-01T00:00:00Z",
                )
                .await
                .unwrap();
            repo.add_paper_to_workspace(&ws.id, &paper.id, "2025-01-01T00:00:00Z")
                .await
                .unwrap();
            repo.add_keyword(&format!("search-kw-{}", i), "test", &paper.id)
                .await
                .unwrap();
        }

        let iterations = 10;
        let mut total_duration = std::time::Duration::new(0, 0);

        for _ in 0..iterations {
            let start = Instant::now();
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri(&format!("/api/search?workspace_id={}&q=test", ws.id))
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            let duration = start.elapsed();
            total_duration += duration;

            assert_eq!(response.status(), StatusCode::OK);
        }

        let avg_duration = total_duration / iterations as u32;
        println!("Average search request time: {:?}", avg_duration);
        
        assert!(avg_duration < std::time::Duration::from_millis(100), 
            "Average search time should be under 100ms, got {:?}", avg_duration);
    }

    #[tokio::test]
    async fn test_graph_data_performance() {
        let graph = spawn_neo4j().await;
        let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph.clone());
        let app = app(graph);

        let ws = repo
            .create_workspace("graph-perf-ws", "GraphPerfTest", "", "2025-01-01T00:00:00Z")
            .await
            .unwrap();

        for i in 0..3 {
            let paper = repo
                .create_paper_if_not_exists(
                    &format!("graph-paper-{}", i),
                    &format!("Graph Paper {}", i),
                    Some(&format!("10.1234/graph{}", i)),
                    None,
                    None,
                    Some(2024),
                    None,
                    "2025-01-01T00:00:00Z",
                )
                .await
                .unwrap();
            repo.add_paper_to_workspace(&ws.id, &paper.id, "2025-01-01T00:00:00Z")
                .await
                .unwrap();
            let author = repo
                .create_author_if_not_exists(&format!("graph-author-{}", i), &format!("Author {}", i), None)
                .await
                .unwrap();
            repo.link_first_author(&author.id, &paper.id)
                .await
                .unwrap();
        }

        let iterations = 5;
        let mut total_duration = std::time::Duration::new(0, 0);

        for _ in 0..iterations {
            let start = Instant::now();
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri(&format!("/api/graph?workspace_id={}", ws.id))
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            let duration = start.elapsed();
            total_duration += duration;

            assert_eq!(response.status(), StatusCode::OK);
        }

        let avg_duration = total_duration / iterations as u32;
        println!("Average graph data request time: {:?}", avg_duration);
        
        assert!(avg_duration < std::time::Duration::from_millis(100), 
            "Average graph data time should be under 100ms, got {:?}", avg_duration);
    }
}