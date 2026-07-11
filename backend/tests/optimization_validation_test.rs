use neo4rs::Graph;
use literature_integration::repositories::neo4j_repo::Neo4jRepo;

async fn get_graph() -> Graph {
    dotenvy::dotenv().ok();
    let uri = std::env::var("NEO4J_URI").unwrap_or_else(|_| "bolt://localhost:7687".into());
    let user = std::env::var("NEO4J_USER").unwrap_or_else(|_| "neo4j".into());
    let password = std::env::var("NEO4J_PASSWORD").unwrap_or_else(|_| "password".into());

    let config = neo4rs::ConfigBuilder::default()
        .uri(&uri)
        .user(&user)
        .password(&password)
        .max_connections(4)
        .fetch_size(1000)
        .build()
        .unwrap();

    Graph::connect(config).await.unwrap()
}

async fn setup_basic_test_data(repo: &Neo4jRepo) -> (String, String) {
    let ws_id = uuid::Uuid::new_v4().to_string();
    let paper_id = uuid::Uuid::new_v4().to_string();
    
    repo.create_workspace(&ws_id, "ValidationTest", "Test workspace", "2025-01-01T00:00:00Z").await.unwrap();
    
    repo.create_paper_if_not_exists(
        &paper_id,
        "Validation Test Paper",
        Some("10.1234/validation"),
        None,
        Some("Abstract for validation testing"),
        Some(2024),
        Some("Validation Journal"),
        "2025-01-01T00:00:00Z",
    ).await.unwrap();
    
    repo.add_paper_to_workspace(&ws_id, &paper_id, "2025-01-01T00:00:00Z").await.unwrap();
    
    (ws_id, paper_id)
}

#[tokio::test]
async fn test_create_authors_batch_returns_correct_authors() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (ws_id, paper_id) = setup_basic_test_data(&repo).await;
    
    let authors = vec![
        (uuid::Uuid::new_v4().to_string(), "First Author".to_string(), Some("0000-0001-1111-1111".to_string()), true, false),
        (uuid::Uuid::new_v4().to_string(), "Corresponding Author".to_string(), Some("0000-0002-2222-2222".to_string()), false, true),
        (uuid::Uuid::new_v4().to_string(), "Co Author 1".to_string(), None, false, false),
        (uuid::Uuid::new_v4().to_string(), "Co Author 2".to_string(), None, false, false),
    ];
    
    let (first_author, corresponding_author) = repo.create_authors_batch(&authors, &paper_id, &ws_id).await.unwrap();
    
    assert!(first_author.is_some(), "First author should be returned");
    assert!(corresponding_author.is_some(), "Corresponding author should be returned");
    
    let first = first_author.unwrap();
    let corr = corresponding_author.unwrap();
    
    assert_eq!(first.name, "First Author");
    assert_eq!(first.orcid, Some("0000-0001-1111-1111".to_string()));
    
    assert_eq!(corr.name, "Corresponding Author");
    assert_eq!(corr.orcid, Some("0000-0002-2222-2222".to_string()));
    
    assert_ne!(first.id, corr.id, "First and corresponding authors should have different ids");
}

#[tokio::test]
async fn test_create_authors_batch_single_author_both_roles() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (ws_id, paper_id) = setup_basic_test_data(&repo).await;
    
    let authors = vec![
        (uuid::Uuid::new_v4().to_string(), "Both Roles Author".to_string(), None, true, true),
    ];
    
    let (first_author, corresponding_author) = repo.create_authors_batch(&authors, &paper_id, &ws_id).await.unwrap();
    
    assert!(first_author.is_some());
    assert!(corresponding_author.is_some());
    
    let first = first_author.unwrap();
    let corr = corresponding_author.unwrap();
    
    assert_eq!(first.name, "Both Roles Author");
    assert_eq!(corr.name, "Both Roles Author");
    assert_eq!(first.id, corr.id);
}

#[tokio::test]
async fn test_create_authors_batch_empty() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (ws_id, paper_id) = setup_basic_test_data(&repo).await;
    
    let authors: Vec<(String, String, Option<String>, bool, bool)> = vec![];
    
    let (first_author, corresponding_author) = repo.create_authors_batch(&authors, &paper_id, &ws_id).await.unwrap();
    
    assert!(first_author.is_none());
    assert!(corresponding_author.is_none());
}

#[tokio::test]
async fn test_add_keywords_batch_correct_count() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (_ws_id, paper_id) = setup_basic_test_data(&repo).await;
    
    let keywords = vec![
        (uuid::Uuid::new_v4().to_string(), "keyword1".to_string()),
        (uuid::Uuid::new_v4().to_string(), "keyword2".to_string()),
        (uuid::Uuid::new_v4().to_string(), "keyword3".to_string()),
        (uuid::Uuid::new_v4().to_string(), "keyword4".to_string()),
        (uuid::Uuid::new_v4().to_string(), "keyword5".to_string()),
    ];
    
    repo.add_keywords_batch(&keywords, &paper_id).await.unwrap();
    
    let paper_keywords = repo.get_paper_keywords(&paper_id).await.unwrap();
    assert_eq!(paper_keywords.len(), 5);
    
    let keyword_names: Vec<&str> = paper_keywords.iter().map(|k| k.name.as_str()).collect();
    for kw in &keywords {
        assert!(keyword_names.contains(&kw.1.as_str()), "Keyword {} should be present", kw.1);
    }
}

#[tokio::test]
async fn test_add_keywords_batch_empty() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (_ws_id, paper_id) = setup_basic_test_data(&repo).await;
    
    let keywords: Vec<(String, String)> = vec![];
    
    let result = repo.add_keywords_batch(&keywords, &paper_id).await;
    assert!(result.is_ok());
    
    let paper_keywords = repo.get_paper_keywords(&paper_id).await.unwrap();
    assert_eq!(paper_keywords.len(), 0);
}

#[tokio::test]
async fn test_search_by_keyword_finds_title_matches() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (ws_id, paper_id) = setup_basic_test_data(&repo).await;
    
    let results = repo.search_by_keyword(&ws_id, "Validation Test").await.unwrap();
    assert!(!results.is_empty(), "Should find paper by title match");
    
    let found = results.iter().any(|p| p.id == paper_id);
    assert!(found, "The test paper should be in search results");
}

#[tokio::test]
async fn test_search_by_keyword_finds_abstract_matches() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (ws_id, paper_id) = setup_basic_test_data(&repo).await;
    
    let results = repo.search_by_keyword(&ws_id, "validation testing").await.unwrap();
    assert!(!results.is_empty(), "Should find paper by abstract match");
    
    let found = results.iter().any(|p| p.id == paper_id);
    assert!(found, "The test paper should be in search results");
}

#[tokio::test]
async fn test_search_by_keyword_finds_keyword_matches() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (ws_id, paper_id) = setup_basic_test_data(&repo).await;
    
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "uniquesearchkw", &paper_id).await.unwrap();
    
    let results = repo.search_by_keyword(&ws_id, "uniquesearchkw").await.unwrap();
    assert!(!results.is_empty(), "Should find paper by keyword match");
    
    let found = results.iter().any(|p| p.id == paper_id);
    assert!(found, "The test paper should be in search results");
}

#[tokio::test]
async fn test_search_by_keyword_no_duplicates() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (ws_id, paper_id) = setup_basic_test_data(&repo).await;
    
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "Validation", &paper_id).await.unwrap();
    
    let results = repo.search_by_keyword(&ws_id, "Validation").await.unwrap();
    
    let mut ids = std::collections::HashSet::new();
    for p in &results {
        ids.insert(p.id.clone());
    }
    assert_eq!(ids.len(), results.len(), "Search results should not contain duplicates");
}

#[tokio::test]
async fn test_get_graph_data_returns_correct_structure() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (ws_id, paper_id) = setup_basic_test_data(&repo).await;
    
    let author1_id = uuid::Uuid::new_v4().to_string();
    let author2_id = uuid::Uuid::new_v4().to_string();
    
    repo.create_author_if_not_exists(&author1_id, "Graph Author 1", None).await.unwrap();
    repo.create_author_if_not_exists(&author2_id, "Graph Author 2", None).await.unwrap();
    
    repo.link_first_author(&author1_id, &paper_id).await.unwrap();
    repo.link_corresponding_author(&author2_id, &paper_id).await.unwrap();
    repo.link_co_authors(&author1_id, &author2_id, &ws_id).await.unwrap();
    
    let (nodes, links) = repo.get_graph_data(&ws_id).await.unwrap();
    
    assert!(!nodes.is_empty(), "Should have graph nodes");
    assert!(!links.is_empty(), "Should have graph links");
    
    let node_ids: Vec<&str> = nodes.iter().map(|n| n.id.as_str()).collect();
    assert!(node_ids.contains(&author1_id.as_str()), "Author 1 should be a node");
    assert!(node_ids.contains(&author2_id.as_str()), "Author 2 should be a node");
    
    let has_link = links.iter().any(|l| 
        (l.source == author1_id && l.target == author2_id) ||
        (l.source == author2_id && l.target == author1_id)
    );
    assert!(has_link, "Should have a link between author 1 and author 2");
}

#[tokio::test]
async fn test_get_graph_data_empty_workspace() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let ws_id = uuid::Uuid::new_v4().to_string();
    repo.create_workspace(&ws_id, "EmptyGraph", "", "2025-01-01T00:00:00Z").await.unwrap();
    
    let (nodes, links) = repo.get_graph_data(&ws_id).await.unwrap();
    
    assert_eq!(nodes.len(), 0);
    assert_eq!(links.len(), 0);
}

#[tokio::test]
async fn test_get_paper_detail_complete() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (_ws_id, paper_id) = setup_basic_test_data(&repo).await;
    
    let author1_id = uuid::Uuid::new_v4().to_string();
    let author2_id = uuid::Uuid::new_v4().to_string();
    
    repo.create_author_if_not_exists(&author1_id, "Detail First", None).await.unwrap();
    repo.create_author_if_not_exists(&author2_id, "Detail Corresponding", None).await.unwrap();
    
    repo.link_first_author(&author1_id, &paper_id).await.unwrap();
    repo.link_corresponding_author(&author2_id, &paper_id).await.unwrap();
    
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "detailkw1", &paper_id).await.unwrap();
    repo.add_keyword(&uuid::Uuid::new_v4().to_string(), "detailkw2", &paper_id).await.unwrap();
    
    let detail = repo.get_paper_detail(&paper_id).await.unwrap();
    assert!(detail.is_some());
    
    let (paper, first_author, corresponding_author, keywords) = detail.unwrap();
    
    assert_eq!(paper.id, paper_id);
    assert_eq!(paper.title, "Validation Test Paper");
    
    assert!(first_author.is_some());
    assert_eq!(first_author.unwrap().name, "Detail First");
    
    assert!(corresponding_author.is_some());
    assert_eq!(corresponding_author.unwrap().name, "Detail Corresponding");
    
    assert_eq!(keywords.len(), 2);
}

#[tokio::test]
async fn test_get_paper_detail_nonexistent() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let detail = repo.get_paper_detail("nonexistent-paper-id").await.unwrap();
    assert!(detail.is_none());
}

#[tokio::test]
async fn test_list_papers_in_workspace_ordered() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let ws_id = uuid::Uuid::new_v4().to_string();
    repo.create_workspace(&ws_id, "OrderedPapers", "", "2025-01-01T00:00:00Z").await.unwrap();
    
    let paper1_id = uuid::Uuid::new_v4().to_string();
    let paper2_id = uuid::Uuid::new_v4().to_string();
    let paper3_id = uuid::Uuid::new_v4().to_string();
    
    repo.create_paper_if_not_exists(&paper1_id, "Old Paper", None, None, None, Some(2020), None, "2025-01-01T00:00:00Z").await.unwrap();
    repo.create_paper_if_not_exists(&paper2_id, "New Paper", None, None, None, Some(2024), None, "2025-01-01T00:00:00Z").await.unwrap();
    repo.create_paper_if_not_exists(&paper3_id, "Middle Paper", None, None, None, Some(2022), None, "2025-01-01T00:00:00Z").await.unwrap();
    
    repo.add_paper_to_workspace(&ws_id, &paper1_id, "2025-01-01T00:00:00Z").await.unwrap();
    repo.add_paper_to_workspace(&ws_id, &paper2_id, "2025-01-01T00:00:00Z").await.unwrap();
    repo.add_paper_to_workspace(&ws_id, &paper3_id, "2025-01-01T00:00:00Z").await.unwrap();
    
    let papers = repo.list_papers_in_workspace(&ws_id).await.unwrap();
    assert_eq!(papers.len(), 3);
    
    assert_eq!(papers[0].year, Some(2024), "First paper should be newest (2024)");
    assert_eq!(papers[1].year, Some(2022), "Second paper should be middle (2022)");
    assert_eq!(papers[2].year, Some(2020), "Third paper should be oldest (2020)");
}

#[tokio::test]
async fn test_workspace_crud_operations() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let ws_id = uuid::Uuid::new_v4().to_string();
    
    let created = repo.create_workspace(&ws_id, "CRUD Test", "Initial description", "2025-01-01T00:00:00Z").await.unwrap();
    assert_eq!(created.id, ws_id);
    assert_eq!(created.name, "CRUD Test");
    assert_eq!(created.description, "Initial description");
    
    let found = repo.get_workspace(&ws_id).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "CRUD Test");
    
    let updated = repo.update_workspace(&ws_id, Some("Updated Name"), Some("Updated desc")).await.unwrap();
    assert!(updated.is_some());
    let updated = updated.unwrap();
    assert_eq!(updated.name, "Updated Name");
    assert_eq!(updated.description, "Updated desc");
    
    let deleted = repo.delete_workspace(&ws_id).await.unwrap();
    assert!(deleted);
    
    let not_found = repo.get_workspace(&ws_id).await.unwrap();
    assert!(not_found.is_none());
}

#[tokio::test]
async fn test_search_by_author_returns_papers() {
    let graph = get_graph().await;
    let repo = Neo4jRepo::new(graph);
    
    let (ws_id, paper_id) = setup_basic_test_data(&repo).await;
    
    let author_id = uuid::Uuid::new_v4().to_string();
    repo.create_author_if_not_exists(&author_id, "Searchable Author Name", None).await.unwrap();
    repo.link_first_author(&author_id, &paper_id).await.unwrap();
    
    let results = repo.search_by_author(&ws_id, "Searchable").await.unwrap();
    assert!(!results.is_empty());
    
    let found = results.iter().any(|a| a.author.name.contains("Searchable"));
    assert!(found, "Should find the searchable author");
    
    let author_result = results.iter().find(|a| a.author.id == author_id).unwrap();
    assert!(!author_result.papers.is_empty(), "Author should have papers");
    assert!(author_result.papers.iter().any(|p| p.id == paper_id));
}
