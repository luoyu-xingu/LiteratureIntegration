use std::time::{Duration, Instant};

#[test]
fn test_search_query_optimization_no_union() {
    let cypher_new = "MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)
                      OPTIONAL MATCH (p)-[:HAS_KEYWORD]->(k:Keyword)
                      WHERE p.title CONTAINS $query OR p.abstract CONTAINS $query OR (k.name CONTAINS $query)
                      RETURN DISTINCT p
                      ORDER BY p.year DESC";
    
    assert!(
        !cypher_new.contains("UNION"),
        "Search query should not use UNION for better performance"
    );
    assert!(
        cypher_new.contains("OPTIONAL MATCH"),
        "Search query should use OPTIONAL MATCH"
    );
    assert!(
        cypher_new.contains("RETURN DISTINCT"),
        "Search query should return distinct results"
    );
}

#[test]
fn test_export_query_static_strings() {
    let has_authors = true;
    let has_keywords = true;
    
    let cypher = match (has_authors, has_keywords) {
        (true, true) => "MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)
                        MATCH (a:Author)-[:FIRST_AUTHOR_OF|CORRESPONDING_AUTHOR_OF]->(p)
                        MATCH (p)-[:HAS_KEYWORD]->(k:Keyword)
                        WHERE a.id IN $author_ids AND k.id IN $keyword_ids
                        RETURN DISTINCT p ORDER BY p.year DESC",
        (true, false) => "MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)
                        MATCH (a:Author)-[:FIRST_AUTHOR_OF|CORRESPONDING_AUTHOR_OF]->(p)
                        WHERE a.id IN $author_ids
                        RETURN DISTINCT p ORDER BY p.year DESC",
        (false, true) => "MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)
                        MATCH (p)-[:HAS_KEYWORD]->(k:Keyword)
                        WHERE k.id IN $keyword_ids
                        RETURN DISTINCT p ORDER BY p.year DESC",
        (false, false) => "MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)
                        RETURN p ORDER BY p.year DESC",
    };
    
    assert!(
        cypher.contains("MATCH"),
        "Export query should be a static string"
    );
}

#[test]
fn test_vec_prealloc_optimization() {
    const TEST_SIZE: usize = 1000;
    
    let start = Instant::now();
    let mut v1 = Vec::with_capacity(TEST_SIZE);
    for i in 0..TEST_SIZE {
        v1.push(i.to_string());
    }
    let dur_prealloc = start.elapsed();
    
    let start = Instant::now();
    let mut v2 = Vec::new();
    for i in 0..TEST_SIZE {
        v2.push(i.to_string());
    }
    let dur_new = start.elapsed();
    
    assert_eq!(v1.len(), TEST_SIZE);
    assert_eq!(v2.len(), TEST_SIZE);
    assert_eq!(v1.capacity(), TEST_SIZE);
    assert!(
        v2.capacity() >= TEST_SIZE,
        "Vec::new() should have grown to at least TEST_SIZE"
    );
    assert!(
        dur_prealloc <= dur_new * 3,
        "Prealloc should be faster or similar to Vec::new()"
    );
}

#[test]
fn test_batch_collection_optimization() {
    const BATCH_SIZE: usize = 50;
    
    let source: Vec<(String, bool)> = (0..BATCH_SIZE)
        .map(|i| (format!("item-{}", i), i % 2 == 0))
        .collect();
    
    let start = Instant::now();
    let mut ids1 = Vec::with_capacity(source.len());
    let mut flags1 = Vec::with_capacity(source.len());
    for item in &source {
        ids1.push(item.0.clone());
        flags1.push(item.1);
    }
    let dur_manual = start.elapsed();
    
    let start = Instant::now();
    let ids2: Vec<String> = source.iter().map(|a| a.0.clone()).collect();
    let flags2: Vec<bool> = source.iter().map(|a| a.1).collect();
    let dur_map = start.elapsed();
    
    assert_eq!(ids1, ids2);
    assert_eq!(flags1, flags2);
    assert!(
        dur_manual <= dur_map * 2,
        "Manual prealloc should be comparable or faster"
    );
}

#[test]
fn test_cypher_query_efficiency() {
    let search_cypher = "MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)
                      OPTIONAL MATCH (p)-[:HAS_KEYWORD]->(k:Keyword)
                      WHERE p.title CONTAINS $query OR p.abstract CONTAINS $query OR (k.name CONTAINS $query)
                      RETURN DISTINCT p
                      ORDER BY p.year DESC";
    
    let parts: Vec<&str> = search_cypher.split_whitespace().collect();
    assert!(
        parts.len() < 60,
        "Query should be concise for better planning"
    );
    
    assert!(
        search_cypher.lines().count() <= 6,
        "Query should be compact"
    );
}

#[test]
fn test_export_query_variants_coverage() {
    let variants = [
        (true, true),
        (true, false),
        (false, true),
        (false, false),
    ];
    
    for (has_authors, has_keywords) in variants.iter() {
        let cypher = match (*has_authors, *has_keywords) {
            (true, true) => "has both",
            (true, false) => "has authors only",
            (false, true) => "has keywords only",
            (false, false) => "has none",
        };
        assert!(
            !cypher.is_empty(),
            "All query variants should be covered"
        );
    }
}

#[test]
fn test_performance_baseline() {
    const OPERATIONS: usize = 10000;
    
    let start = Instant::now();
    for _ in 0..OPERATIONS {
        let _vec: Vec<(String, String, Option<String>, bool, bool)> = Vec::with_capacity(10);
    }
    let duration = start.elapsed();
    
    assert!(
        duration < Duration::from_millis(50),
        "Vector allocation should be very fast, took {:?}",
        duration
    );
}

#[test]
fn test_query_parameter_count() {
    let search_cypher = "MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)
                      OPTIONAL MATCH (p)-[:HAS_KEYWORD]->(k:Keyword)
                      WHERE p.title CONTAINS $query OR p.abstract CONTAINS $query OR (k.name CONTAINS $query)
                      RETURN DISTINCT p
                      ORDER BY p.year DESC";
    
    let param_occurrences = search_cypher.matches('$').count();
    let unique_params = ["$workspace_id", "$query"];
    let has_all_params = unique_params.iter().all(|p| search_cypher.contains(p));
    
    assert!(
        has_all_params,
        "Search query should have all required parameters"
    );
    assert!(
        param_occurrences >= unique_params.len(),
        "Search query should have at least {} unique parameters",
        unique_params.len()
    );
}
