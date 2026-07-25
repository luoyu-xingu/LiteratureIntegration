mod common;

#[cfg(test)]
mod tests {
    use crate::common::{app, spawn_neo4j};
    use axum::body::Body;
    use http_body_util::BodyExt;
    use hyper::{Request, StatusCode};
    use serde_json::json;
    use std::time::Instant;

    async fn setup_test_data(graph: &neo4rs::Graph, count: usize) -> (String, Vec<String>) {
        let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph.clone());
        
        let ws_id = uuid::Uuid::new_v4().to_string();
        repo.create_workspace(&ws_id, "PerformanceTest", "", "2025-01-01T00:00:00Z")
            .await
            .unwrap();
        
        let mut paper_ids = Vec::with_capacity(count);
        for i in 0..count {
            let paper_id = uuid::Uuid::new_v4().to_string();
            let title = format!("Performance Test Paper {}", i);
            let abstract_text = format!("This is abstract number {} for performance testing", i);
            
            repo.create_paper_if_not_exists(
                &paper_id,
                &title,
                Some(&format!("10.1234/perf{}", i)),
                None,
                Some(&abstract_text),
                Some(2024),
                Some("Performance Journal"),
                "2025-01-01T00:00:00Z",
            )
            .await
            .unwrap();
            
            repo.add_paper_to_workspace(&ws_id, &paper_id, "2025-01-01T00:00:00Z")
                .await
                .unwrap();
            
            let author_id = uuid::Uuid::new_v4().to_string();
            repo.create_author_if_not_exists(&author_id, &format!("Author {}", i), None)
                .await
                .unwrap();
            repo.link_first_author(&author_id, &paper_id).await.unwrap();
            
            paper_ids.push(paper_id);
        }
        
        (ws_id, paper_ids)
    }

    #[tokio::test]
    async fn test_search_performance() {
        let graph = spawn_neo4j().await;
        let app = app(graph.clone());
        
        let (ws_id, _) = setup_test_data(&graph, 50).await;
        
        let start = Instant::now();
        for _ in 0..10 {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri(&format!("/api/search?workspace_id={}&q=Performance", ws_id))
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            
            assert_eq!(response.status(), StatusCode::OK);
            let body = response.into_body().collect().await.unwrap().to_bytes();
            let result: serde_json::Value = serde_json::from_slice(&body).unwrap();
            assert!(result["results"].as_array().unwrap().len() > 0);
        }
        let duration = start.elapsed();
        
        println!("Search performance: {} ms for 10 requests", duration.as_millis());
        
        assert!(duration.as_millis() < 5000, 
            "Search took too long: {} ms", duration.as_millis());
    }

    #[tokio::test]
    async fn test_get_paper_detail_performance() {
        let graph = spawn_neo4j().await;
        let app = app(graph.clone());
        
        let (_ws_id, paper_ids) = setup_test_data(&graph, 20).await;
        
        let start = Instant::now();
        for paper_id in &paper_ids {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri(&format!("/api/paper/{}", paper_id))
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            
            assert_eq!(response.status(), StatusCode::OK);
        }
        let duration = start.elapsed();
        
        println!("Get paper detail performance: {} ms for {} requests", 
            duration.as_millis(), paper_ids.len());
        
        assert!(duration.as_millis() < 3000, 
            "Get paper detail took too long: {} ms", duration.as_millis());
    }

    #[tokio::test]
    async fn test_list_papers_performance() {
        let graph = spawn_neo4j().await;
        let app = app(graph.clone());
        
        let (ws_id, _) = setup_test_data(&graph, 100).await;
        
        let start = Instant::now();
        for _ in 0..5 {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri(&format!("/api/papers?workspace_id={}", ws_id))
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            
            assert_eq!(response.status(), StatusCode::OK);
            let body = response.into_body().collect().await.unwrap().to_bytes();
            let papers: serde_json::Value = serde_json::from_slice(&body).unwrap();
            assert_eq!(papers.as_array().unwrap().len(), 100);
        }
        let duration = start.elapsed();
        
        println!("List papers performance: {} ms for 5 requests", duration.as_millis());
        
        assert!(duration.as_millis() < 2000, 
            "List papers took too long: {} ms", duration.as_millis());
    }

    #[tokio::test]
    async fn test_get_graph_data_performance() {
        let graph = spawn_neo4j().await;
        let app = app(graph.clone());
        
        let (ws_id, _) = setup_test_data(&graph, 50).await;
        
        let start = Instant::now();
        for _ in 0..5 {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri(&format!("/api/graph?workspace_id={}", ws_id))
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            
            assert_eq!(response.status(), StatusCode::OK);
        }
        let duration = start.elapsed();
        
        println!("Get graph data performance: {} ms for 5 requests", duration.as_millis());
        
        assert!(duration.as_millis() < 3000, 
            "Get graph data took too long: {} ms", duration.as_millis());
    }

    #[tokio::test]
    async fn test_export_performance() {
        let graph = spawn_neo4j().await;
        let app = app(graph.clone());
        
        let (ws_id, _) = setup_test_data(&graph, 30).await;
        
        let start = Instant::now();
        for _ in 0..3 {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri(&format!("/api/export?workspace_id={}", ws_id))
                        .header("content-type", "application/json")
                        .body(Body::from(
                            serde_json::to_string(&json!({ "format": "markdown" })).unwrap(),
                        ))
                        .unwrap(),
                )
                .await
                .unwrap();
            
            assert_eq!(response.status(), StatusCode::OK);
        }
        let duration = start.elapsed();
        
        println!("Export performance: {} ms for 3 requests", duration.as_millis());
        
        assert!(duration.as_millis() < 3000, 
            "Export took too long: {} ms", duration.as_millis());
    }
}