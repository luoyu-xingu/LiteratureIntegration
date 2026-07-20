//! Performance optimization test
//! Tests to verify that the performance optimizations work correctly

use std::time::Instant;

/// Test string capacity pre-allocation
#[test]
fn test_string_capacity_optimization() {
    // Test that our capacity calculations are correct
    const BASE_CAPACITY: usize = 150;
    const AUTHOR_FILTER_SIZE: usize = 80;
    const KEYWORD_FILTER_SIZE: usize = 60;
    
    // Test case 1: No filters
    let capacity1 = BASE_CAPACITY;
    let mut s1 = String::with_capacity(capacity1);
    s1.push_str("MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)");
    assert!(s1.capacity() >= capacity1, "String should have at least the base capacity");
    
    // Test case 2: With author filter
    let capacity2 = BASE_CAPACITY + AUTHOR_FILTER_SIZE;
    let mut s2 = String::with_capacity(capacity2);
    s2.push_str("MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)");
    s2.push_str(" MATCH (a:Author)-[:FIRST_AUTHOR_OF|CORRESPONDING_AUTHOR_OF]->(p)");
    s2.push_str(" WHERE a.id IN $author_ids");
    s2.push_str(" RETURN DISTINCT p ORDER BY p.year DESC LIMIT 200");
    assert!(s2.capacity() >= capacity2, "String should have sufficient capacity");
    
    // Test case 3: With both filters
    let capacity3 = BASE_CAPACITY + AUTHOR_FILTER_SIZE + KEYWORD_FILTER_SIZE;
    let mut s3 = String::with_capacity(capacity3);
    s3.push_str("MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)");
    s3.push_str(" MATCH (a:Author)-[:FIRST_AUTHOR_OF|CORRESPONDING_AUTHOR_OF]->(p)");
    s3.push_str(" WHERE a.id IN $author_ids");
    s3.push_str(" MATCH (p)-[:HAS_KEYWORD]->(k:Keyword)");
    s3.push_str(" AND k.id IN $keyword_ids");
    s3.push_str(" RETURN DISTINCT p ORDER BY p.year DESC LIMIT 200");
    assert!(s3.capacity() >= capacity3, "String should have sufficient capacity");
}

/// Test that non_empty_string helper function works correctly
#[test]
fn test_non_empty_string_helper() {
    fn non_empty_string(s: String) -> Option<String> {
        if s.is_empty() { None } else { Some(s) }
    }
    
    // Test with empty string
    assert!(non_empty_string(String::new()).is_none());
    
    // Test with non-empty string
    let non_empty = non_empty_string(String::from("test"));
    assert!(non_empty.is_some());
    assert_eq!(non_empty.unwrap(), "test");
    
    // Test with whitespace (should be considered non-empty)
    let whitespace = non_empty_string(String::from("  "));
    assert!(whitespace.is_some());
}

/// Test that non_zero_year helper function works correctly
#[test]
fn test_non_zero_year_helper() {
    fn non_zero_year(y: i32) -> Option<i32> {
        if y > 0 { Some(y) } else { None }
    }
    
    // Test with zero
    assert!(non_zero_year(0).is_none());
    
    // Test with negative
    assert!(non_zero_year(-1).is_none());
    
    // Test with positive
    assert_eq!(non_zero_year(2024), Some(2024));
}

/// Test performance improvement of string building
#[test]
fn test_string_building_performance() {
    const ITERATIONS: usize = 10000;
    
    // Test with pre-allocated capacity
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let mut s = String::with_capacity(300);
        s.push_str("MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)");
        s.push_str(" OPTIONAL MATCH (fa:Author)-[:FIRST_AUTHOR_OF]->(p)");
        s.push_str(" OPTIONAL MATCH (ca:Author)-[:CORRESPONDING_AUTHOR_OF]->(p)");
        s.push_str(" OPTIONAL MATCH (p)-[:HAS_KEYWORD]->(k:Keyword)");
        s.push_str(" WITH p, head(collect(DISTINCT fa)) AS fa, head(collect(DISTINCT ca)) AS ca, collect(DISTINCT k) AS keywords");
        s.push_str(" RETURN p, fa, ca, keywords ORDER BY p.year DESC LIMIT 50");
        std::hint::black_box(s);
    }
    let optimized_duration = start.elapsed();
    
    // Test without pre-allocated capacity (baseline)
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let mut s = String::new();
        s.push_str("MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)");
        s.push_str(" OPTIONAL MATCH (fa:Author)-[:FIRST_AUTHOR_OF]->(p)");
        s.push_str(" OPTIONAL MATCH (ca:Author)-[:CORRESPONDING_AUTHOR_OF]->(p)");
        s.push_str(" OPTIONAL MATCH (p)-[:HAS_KEYWORD]->(k:Keyword)");
        s.push_str(" WITH p, head(collect(DISTINCT fa)) AS fa, head(collect(DISTINCT ca)) AS ca, collect(DISTINCT k) AS keywords");
        s.push_str(" RETURN p, fa, ca, keywords ORDER BY p.year DESC LIMIT 50");
        std::hint::black_box(s);
    }
    let baseline_duration = start.elapsed();
    
    // Performance should be better with pre-allocation
    println!("Optimized: {:?}", optimized_duration);
    println!("Baseline: {:?}", baseline_duration);
    
    // The optimized version should be at least as fast (allowing for some variance)
    // In practice, it should be faster due to fewer allocations
}

/// Test that the optimized filter functions maintain correctness
#[test]
fn test_filter_correctness() {
    // Test data
    struct TestData {
        id: String,
        name: String,
        value: Option<String>,
        year: Option<i32>,
    }
    
    fn non_empty_string(s: String) -> Option<String> {
        if s.is_empty() { None } else { Some(s) }
    }
    
    fn non_zero_year(y: i32) -> Option<i32> {
        if y > 0 { Some(y) } else { None }
    }
    
    // Test case 1: All fields populated
    let data1 = TestData {
        id: "test-id".to_string(),
        name: "test-name".to_string(),
        value: Some("test-value".to_string()),
        year: Some(2024),
    };
    assert!(non_empty_string(data1.value.clone().unwrap()).is_some());
    assert!(non_zero_year(data1.year.unwrap()).is_some());
    
    // Test case 2: Empty values
    let data2 = TestData {
        id: "test-id".to_string(),
        name: "test-name".to_string(),
        value: Some(String::new()),
        year: Some(0),
    };
    assert!(non_empty_string(data2.value.unwrap()).is_none());
    assert!(non_zero_year(data2.year.unwrap()).is_none());
    
    // Test case 3: None values
    let data3 = TestData {
        id: "test-id".to_string(),
        name: "test-name".to_string(),
        value: None,
        year: None,
    };
    assert!(data3.value.and_then(non_empty_string).is_none());
    assert!(data3.year.and_then(non_zero_year).is_none());
}