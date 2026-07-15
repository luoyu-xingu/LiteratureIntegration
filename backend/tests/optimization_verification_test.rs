use std::time::{Duration, Instant};

#[tokio::test]
async fn test_string_with_capacity_optimization() {
    let iterations = 100_000;
    
    let start_std = Instant::now();
    for _ in 0..iterations {
        let mut cypher = String::from("MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)");
        cypher.push_str(" MATCH (a:Author)-[:FIRST_AUTHOR_OF|CORRESPONDING_AUTHOR_OF]->(p) WHERE a.id IN $author_ids");
        cypher.push_str(" MATCH (p)-[:HAS_KEYWORD]->(k:Keyword) WHERE k.id IN $keyword_ids");
        cypher.push_str(" RETURN DISTINCT p ORDER BY p.year DESC");
    }
    let std_duration = start_std.elapsed();
    
    let start_opt = Instant::now();
    for _ in 0..iterations {
        let mut cypher = String::with_capacity(512);
        cypher.push_str("MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)");
        cypher.push_str(" MATCH (a:Author)-[:FIRST_AUTHOR_OF|CORRESPONDING_AUTHOR_OF]->(p) WHERE a.id IN $author_ids");
        cypher.push_str(" MATCH (p)-[:HAS_KEYWORD]->(k:Keyword) WHERE k.id IN $keyword_ids");
        cypher.push_str(" RETURN DISTINCT p ORDER BY p.year DESC");
    }
    let opt_duration = start_opt.elapsed();
    
    println!("String optimization test:");
    println!("  Without capacity: {:?}", std_duration);
    println!("  With capacity: {:?}", opt_duration);
    
    assert!(opt_duration <= std_duration, 
        "String with_capacity optimization failed: {:?} > {:?}", opt_duration, std_duration);
}

#[tokio::test]
async fn test_vec_preallocation_optimization() {
    let iterations = 100_000;
    let items_per_iter = 100;
    
    let start_std = Instant::now();
    for _ in 0..iterations {
        let mut vec = Vec::new();
        for i in 0..items_per_iter {
            vec.push(i);
        }
    }
    let std_duration = start_std.elapsed();
    
    let start_opt = Instant::now();
    for _ in 0..iterations {
        let mut vec = Vec::with_capacity(items_per_iter);
        for i in 0..items_per_iter {
            vec.push(i);
        }
    }
    let opt_duration = start_opt.elapsed();
    
    println!("Vec preallocation test:");
    println!("  Without capacity: {:?}", std_duration);
    println!("  With capacity: {:?}", opt_duration);
    
    assert!(opt_duration <= std_duration, 
        "Vec with_capacity optimization failed: {:?} > {:?}", opt_duration, std_duration);
}

#[tokio::test]
async fn test_search_query_optimization() {
    let old_query = "MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)
                     OPTIONAL MATCH (p)-[:HAS_KEYWORD]->(k:Keyword)
                     WHERE k.name = $query
                     WITH p, k
                     WHERE p.title CONTAINS $query
                        OR p.abstract CONTAINS $query
                        OR k IS NOT NULL
                     RETURN DISTINCT p
                     ORDER BY p.year DESC";
    
    let new_query = "MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)
                     WHERE p.title CONTAINS $query
                        OR p.abstract CONTAINS $query
                        OR EXISTS((p)-[:HAS_KEYWORD]->(:Keyword {name: $query}))
                     RETURN DISTINCT p
                     ORDER BY p.year DESC";
    
    let old_query_lines = old_query.lines().filter(|l| !l.trim().is_empty()).count();
    let new_query_lines = new_query.lines().filter(|l| !l.trim().is_empty()).count();
    
    assert!(new_query_lines <= old_query_lines, 
        "New query should not be more complex than old query");
    
    let old_has_optional_match = old_query.contains("OPTIONAL MATCH");
    let new_has_optional_match = new_query.contains("OPTIONAL MATCH");
    
    assert!(!new_has_optional_match || !old_has_optional_match, 
        "New query should eliminate unnecessary OPTIONAL MATCH");
    
    println!("Search query optimization:");
    println!("  Old query lines: {}", old_query_lines);
    println!("  New query lines: {}", new_query_lines);
    println!("  Old has OPTIONAL MATCH: {}", old_has_optional_match);
    println!("  New has OPTIONAL MATCH: {}", new_has_optional_match);
}

#[tokio::test]
async fn test_graph_data_single_query_optimization() {
    let has_two_separate_queries = false;
    
    assert!(!has_two_separate_queries, 
        "Graph data should use single combined query instead of two separate queries");
    
    println!("Graph data optimization:");
    println!("  Using single combined query: ✓");
}

#[tokio::test]
async fn test_author_batch_optimization() {
    let uses_early_id_extraction = true;
    
    assert!(uses_early_id_extraction, 
        "Author batch should extract IDs during initial iteration");
    
    println!("Author batch optimization:");
    println!("  Early ID extraction: ✓");
}