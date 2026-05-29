mod common;

#[cfg(test)]
mod tests {
    use crate::common::{app, spawn_neo4j};
    use axum::body::Body;
    use tower::util::ServiceExt;
    use http_body_util::BodyExt;
    use hyper::{Request, StatusCode};
    use serde_json::json;

    #[tokio::test]
    async fn test_health_check() {
        let graph = spawn_neo4j().await;
        let app = app(graph);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let text = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(text, "ok");
    }

    #[tokio::test]
    async fn test_create_workspace() {
        let graph = spawn_neo4j().await;
        let app = app(graph);

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/workspaces")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_string(&json!({
                            "name": "Test Workspace",
                            "description": "A test workspace"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let workspace: serde_json::Value =
            serde_json::from_slice(&body).unwrap();
        assert_eq!(workspace["name"], "Test Workspace");
        assert_eq!(workspace["description"], "A test workspace");
        assert!(!workspace["id"].as_str().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_list_workspaces() {
        let graph = spawn_neo4j().await;
        let app = app(graph);

        let create_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/workspaces")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_string(&json!({
                            "name": "WS1",
                            "description": "First"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(create_resp.status(), StatusCode::OK);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/workspaces")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let workspaces: serde_json::Value =
            serde_json::from_slice(&body).unwrap();
        assert!(workspaces.as_array().unwrap().len() >= 1);
    }

    #[tokio::test]
    async fn test_get_workspace() {
        let graph = spawn_neo4j().await;
        let app = app(graph);

        let create_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/workspaces")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_string(&json!({
                            "name": "GetTest",
                            "description": "For get"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        let create_body = create_resp.into_body().collect().await.unwrap().to_bytes();
        let created: serde_json::Value = serde_json::from_slice(&create_body).unwrap();
        let id = created["id"].as_str().unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .uri(&format!("/api/workspace/{}", id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let workspace: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(workspace["name"], "GetTest");
    }

    #[tokio::test]
    async fn test_update_workspace() {
        let graph = spawn_neo4j().await;
        let app = app(graph);

        let create_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/workspaces")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_string(&json!({
                            "name": "Before",
                            "description": "Old desc"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        let create_body = create_resp.into_body().collect().await.unwrap().to_bytes();
        let created: serde_json::Value = serde_json::from_slice(&create_body).unwrap();
        let id = created["id"].as_str().unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri(&format!("/api/workspace/{}", id))
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_string(&json!({
                            "name": "After",
                            "description": "New desc"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let updated: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(updated["name"], "After");
        assert_eq!(updated["description"], "New desc");
    }

    #[tokio::test]
    async fn test_delete_workspace() {
        let graph = spawn_neo4j().await;
        let app = app(graph);

        let create_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/workspaces")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_string(&json!({
                            "name": "ToDelete",
                            "description": "Will be deleted"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        let create_body = create_resp.into_body().collect().await.unwrap().to_bytes();
        let created: serde_json::Value = serde_json::from_slice(&create_body).unwrap();
        let id = created["id"].as_str().unwrap();

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri(&format!("/api/workspace/{}", id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let get_resp = app
            .oneshot(
                Request::builder()
                    .uri(&format!("/api/workspace/{}", id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(get_resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_nonexistent_workspace() {
        let graph = spawn_neo4j().await;
        let app = app(graph);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/workspace/nonexistent-id")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_list_papers_empty() {
        let graph = spawn_neo4j().await;
        let app = app(graph);

        let create_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/workspaces")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_string(&json!({
                            "name": "EmptyPapers",
                            "description": ""
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        let create_body = create_resp.into_body().collect().await.unwrap().to_bytes();
        let created: serde_json::Value = serde_json::from_slice(&create_body).unwrap();
        let ws_id = created["id"].as_str().unwrap();

        let response = app
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
        assert_eq!(papers.as_array().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_list_authors_empty() {
        let graph = spawn_neo4j().await;
        let app = app(graph);

        let create_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/workspaces")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_string(&json!({
                            "name": "EmptyAuthors",
                            "description": ""
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        let create_body = create_resp.into_body().collect().await.unwrap().to_bytes();
        let created: serde_json::Value = serde_json::from_slice(&create_body).unwrap();
        let ws_id = created["id"].as_str().unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .uri(&format!("/api/authors?workspace_id={}", ws_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let authors: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(authors.as_array().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_graph_data_empty() {
        let graph = spawn_neo4j().await;
        let app = app(graph);

        let create_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/workspaces")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_string(&json!({
                            "name": "EmptyGraph",
                            "description": ""
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        let create_body = create_resp.into_body().collect().await.unwrap().to_bytes();
        let created: serde_json::Value = serde_json::from_slice(&create_body).unwrap();
        let ws_id = created["id"].as_str().unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .uri(&format!("/api/graph?workspace_id={}", ws_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let graph_data: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(graph_data["nodes"].as_array().unwrap().len(), 0);
        assert_eq!(graph_data["links"].as_array().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_search_missing_params() {
        let graph = spawn_neo4j().await;
        let app = app(graph);

        let create_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/workspaces")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_string(&json!({
                            "name": "SearchTest",
                            "description": ""
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        let create_body = create_resp.into_body().collect().await.unwrap().to_bytes();
        let created: serde_json::Value = serde_json::from_slice(&create_body).unwrap();
        let ws_id = created["id"].as_str().unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .uri(&format!("/api/search?workspace_id={}", ws_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_search_by_keyword_empty() {
        let graph = spawn_neo4j().await;
        let app = app(graph);

        let create_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/workspaces")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_string(&json!({
                            "name": "SearchKW",
                            "description": ""
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        let create_body = create_resp.into_body().collect().await.unwrap().to_bytes();
        let created: serde_json::Value = serde_json::from_slice(&create_body).unwrap();
        let ws_id = created["id"].as_str().unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .uri(&format!(
                        "/api/search?workspace_id={}&q=nonexistent",
                        ws_id
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let result: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(result["mode"], "keyword");
        assert_eq!(result["results"].as_array().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_update_paper_notes() {
        let graph = spawn_neo4j().await;
        let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph.clone());
        let app = app(graph);

        let ws = repo
            .create_workspace("test-uuid-ws", "NotesTest", "", "2025-01-01T00:00:00Z")
            .await
            .unwrap();
        let paper = repo
            .create_paper_if_not_exists(
                "test-uuid-paper",
                "Test Paper for Notes",
                Some("10.1234/test"),
                None,
                Some("Abstract text"),
                Some(2024),
                Some("Test Journal"),
                "2025-01-01T00:00:00Z",
            )
            .await
            .unwrap();
        repo.add_paper_to_workspace(&ws.id, &paper.id, "2025-01-01T00:00:00Z")
            .await
            .unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri(&format!("/api/paper/{}", paper.id))
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_string(&json!({
                            "user_notes": "# My Notes\n\nThis is **important**.\n\n- Point 1\n- Point 2"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let updated: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(updated["user_notes"]
            .as_str()
            .unwrap()
            .contains("# My Notes"));
    }

    #[tokio::test]
    async fn test_get_paper_detail() {
        let graph = spawn_neo4j().await;
        let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph.clone());
        let app = app(graph);

        let ws = repo
            .create_workspace("test-uuid-detail", "DetailTest", "", "2025-01-01T00:00:00Z")
            .await
            .unwrap();
        let paper = repo
            .create_paper_if_not_exists(
                "test-uuid-detail-paper",
                "Detail Paper",
                Some("10.1234/detail"),
                None,
                Some("Abstract for detail test"),
                Some(2024),
                Some("Detail Journal"),
                "2025-01-01T00:00:00Z",
            )
            .await
            .unwrap();
        repo.add_paper_to_workspace(&ws.id, &paper.id, "2025-01-01T00:00:00Z")
            .await
            .unwrap();

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
        assert_eq!(detail["paper"]["title"], "Detail Paper");
    }

    #[tokio::test]
    async fn test_delete_paper_from_workspace() {
        let graph = spawn_neo4j().await;
        let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph.clone());
        let app = app(graph);

        let ws = repo
            .create_workspace("test-uuid-rm", "RmTest", "", "2025-01-01T00:00:00Z")
            .await
            .unwrap();
        let paper = repo
            .create_paper_if_not_exists(
                "test-uuid-rm-paper",
                "Paper to Remove",
                Some("10.1234/rm"),
                None,
                None,
                Some(2023),
                None,
                "2025-01-01T00:00:00Z",
            )
            .await
            .unwrap();
        repo.add_paper_to_workspace(&ws.id, &paper.id, "2025-01-01T00:00:00Z")
            .await
            .unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri(&format!(
                        "/api/paper-rm?workspace_id={}&paper_id={}",
                        ws.id, paper.id
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let result: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(result["removed"], true);
    }

    #[tokio::test]
    async fn test_export_workspace() {
        let graph = spawn_neo4j().await;
        let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph.clone());
        let app = app(graph);

        let ws = repo
            .create_workspace("test-uuid-export", "ExportTest", "Export desc", "2025-01-01T00:00:00Z")
            .await
            .unwrap();
        let paper = repo
            .create_paper_if_not_exists(
                "test-uuid-export-paper",
                "Export Paper",
                Some("10.1234/export"),
                None,
                Some("Abstract for export"),
                Some(2024),
                Some("Export Journal"),
                "2025-01-01T00:00:00Z",
            )
            .await
            .unwrap();
        repo.add_paper_to_workspace(&ws.id, &paper.id, "2025-01-01T00:00:00Z")
            .await
            .unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(&format!("/api/export?workspace_id={}", ws.id))
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_string(&json!({
                            "format": "markdown"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let markdown = String::from_utf8(body.to_vec()).unwrap();
        assert!(markdown.contains("# 工作区: ExportTest"));
        assert!(markdown.contains("Export Paper"));
    }

    #[tokio::test]
    async fn test_author_papers() {
        let graph = spawn_neo4j().await;
        let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph.clone());
        let app = app(graph);

        let ws = repo
            .create_workspace("test-uuid-ap", "AuthorPaperTest", "", "2025-01-01T00:00:00Z")
            .await
            .unwrap();
        let paper = repo
            .create_paper_if_not_exists(
                "test-uuid-ap-paper",
                "Author Paper Test",
                Some("10.1234/ap"),
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
            .create_author_if_not_exists("test-uuid-ap-author", "Test Author", None)
            .await
            .unwrap();
        repo.link_first_author(&author.id, &paper.id)
            .await
            .unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .uri(&format!("/api/author-papers/{}", author.id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let result: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(result["papers"].as_array().unwrap().len() >= 1);
    }
}
