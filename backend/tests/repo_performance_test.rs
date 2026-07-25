mod common;

#[cfg(test)]
mod tests {
    use crate::common::spawn_neo4j;
    use literature_integration::repositories::neo4j_repo::Neo4jRepo;
    use std::time::Instant;

    async fn setup_test_data(repo: &Neo4jRepo, count: usize) -> (String, Vec<String>) {
        let ws_id = uuid::Uuid::new_v4().to_string();
        repo.create_workspace(&ws_id, "RepoPerfTest", "", "2025-01-01T00:00:00Z")
            .await
            .unwrap();
        
        let mut paper_ids = Vec::with_capacity(count);
        for i in 0..count {
            let paper_id = uuid::Uuid::new_v4().to_string();
            let title = format!("Repo Performance Test Paper {}", i);
            let abstract_text = format!("This is abstract number {} for repository performance testing", i);
            
            repo.create_paper_if_not_exists(
                &paper_id,
                &title,
                Some(&format!("10.1234/repo{}", i)),
                None,
                Some(&abstract_text),
                Some(2024),
                Some("Repo Journal"),
                "2025-01-01T00:00:00Z",
            )
            .await
            .unwrap();
            
            repo.add_paper_to_workspace(&ws_id, &paper_id, "2025-01-01T00:00:00Z")
                .await
                .unwrap();
            
            let author_id = uuid::Uuid::new_v4().to_string();
            repo.create_author_if_not_exists(&author_id, &format!("Repo Author {}", i), None)
                .await
                .unwrap();
            repo.link_first_author(&author_id, &paper_id).await.unwrap();
            
            paper_ids.push(paper_id);
        }
        
        (ws_id, paper_ids)
    }

    #[tokio::test]
    async fn test_search_by_keyword_performance() {
        let graph = spawn_neo4j().await;
        let repo = Neo4jRepo::new(graph);
        
        let (ws_id, _) = setup_test_data(&repo, 50).await;
        
        let start = Instant::now();
        for _ in 0..10 {
            let result = repo.search_by_keyword(&ws_id, "Performance").await;
            assert!(result.is_ok());
            let papers = result.unwrap();
            assert!(!papers.is_empty());
        }
        let duration = start.elapsed();
        
        println!("Search by keyword performance: {} ms for 10 requests", duration.as_millis());
        
        assert!(duration.as_millis() < 5000, 
            "Search took too long: {} ms", duration.as_millis());
    }

    #[tokio::test]
    async fn test_get_paper_detail_performance() {
        let graph = spawn_neo4j().await;
        let repo = Neo4jRepo::new(graph);
        
        let (_ws_id, paper_ids) = setup_test_data(&repo, 20).await;
        
        let start = Instant::now();
        for paper_id in &paper_ids {
            let result = repo.get_paper_detail(paper_id).await;
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
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
        let repo = Neo4jRepo::new(graph);
        
        let (ws_id, _) = setup_test_data(&repo, 100).await;
        
        let start = Instant::now();
        for _ in 0..5 {
            let result = repo.list_papers_in_workspace(&ws_id).await;
            assert!(result.is_ok());
            assert_eq!(result.unwrap().len(), 100);
        }
        let duration = start.elapsed();
        
        println!("List papers performance: {} ms for 5 requests", duration.as_millis());
        
        assert!(duration.as_millis() < 2000, 
            "List papers took too long: {} ms", duration.as_millis());
    }

    #[tokio::test]
    async fn test_get_graph_data_performance() {
        let graph = spawn_neo4j().await;
        let repo = Neo4jRepo::new(graph);
        
        let (ws_id, _) = setup_test_data(&repo, 50).await;
        
        let start = Instant::now();
        for _ in 0..5 {
            let result = repo.get_graph_data(&ws_id).await;
            assert!(result.is_ok());
        }
        let duration = start.elapsed();
        
        println!("Get graph data performance: {} ms for 5 requests", duration.as_millis());
        
        assert!(duration.as_millis() < 3000, 
            "Get graph data took too long: {} ms", duration.as_millis());
    }

    #[tokio::test]
    async fn test_get_papers_detail_batch_performance() {
        let graph = spawn_neo4j().await;
        let repo = Neo4jRepo::new(graph);
        
        let (ws_id, _) = setup_test_data(&repo, 30).await;
        
        let start = Instant::now();
        for _ in 0..3 {
            let result = repo.get_papers_detail_batch(&ws_id, None, None, None).await;
            assert!(result.is_ok());
        }
        let duration = start.elapsed();
        
        println!("Get papers detail batch performance: {} ms for 3 requests", duration.as_millis());
        
        assert!(duration.as_millis() < 3000, 
            "Get papers detail batch took too long: {} ms", duration.as_millis());
    }

    #[tokio::test]
    async fn test_search_by_author_performance() {
        let graph = spawn_neo4j().await;
        let repo = Neo4jRepo::new(graph);
        
        let (ws_id, _) = setup_test_data(&repo, 50).await;
        
        let start = Instant::now();
        for _ in 0..10 {
            let result = repo.search_by_author(&ws_id, "Author").await;
            assert!(result.is_ok());
            assert!(!result.unwrap().is_empty());
        }
        let duration = start.elapsed();
        
        println!("Search by author performance: {} ms for 10 requests", duration.as_millis());
        
        assert!(duration.as_millis() < 5000, 
            "Search by author took too long: {} ms", duration.as_millis());
    }
}