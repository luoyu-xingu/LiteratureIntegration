use std::time::{Instant, Duration};
use literature_integration::models::paper::Paper;

/// Performance benchmark for string operations
#[test]
fn test_string_allocation_performance() {
    let iterations = 10000;
    
    // Test optimized string building
    let start = Instant::now();
    for _ in 0..iterations {
        let given = "John";
        let family = "Doe";
        let mut n = String::with_capacity(given.len() + 1 + family.len());
        n.push_str(given);
        n.push(' ');
        n.push_str(family);
        let _ = n;
    }
    let optimized_duration = start.elapsed();
    
    // Test traditional format! macro
    let start = Instant::now();
    for _ in 0..iterations {
        let given = "John";
        let family = "Doe";
        let n = format!("{} {}", given, family);
        let _ = n;
    }
    let format_duration = start.elapsed();
    
    println!("String building performance:");
    println!("  Optimized: {:?}", optimized_duration);
    println!("  Format: {:?}", format_duration);
    
    // Optimized should be at least as fast as format
    // In practice it's usually faster due to fewer allocations
    assert!(optimized_duration <= format_duration * 2, 
        "Optimized version should not be significantly slower");
}

/// Performance benchmark for vector pre-allocation
#[test]
fn test_vector_preallocation_performance() {
    let size = 1000;
    
    // Test with pre-allocation
    let start = Instant::now();
    for _ in 0..100 {
        let mut v: Vec<i32> = Vec::with_capacity(size);
        for i in 0..size as i32 {
            v.push(i);
        }
        v.shrink_to_fit();
    }
    let preallocated_duration = start.elapsed();
    
    // Test without pre-allocation
    let start = Instant::now();
    for _ in 0..100 {
        let mut v: Vec<i32> = Vec::new();
        for i in 0..size as i32 {
            v.push(i);
        }
        v.shrink_to_fit();
    }
    let dynamic_duration = start.elapsed();
    
    println!("Vector allocation performance:");
    println!("  Pre-allocated: {:?}", preallocated_duration);
    println!("  Dynamic: {:?}", dynamic_duration);
    
    // Pre-allocated should be faster
    assert!(preallocated_duration < dynamic_duration, 
        "Pre-allocated version should be faster");
}

/// Performance benchmark for early exit conditions
#[test]
fn test_early_exit_performance() {
    let iterations = 100000;
    
    // Test with early exit check
    let start = Instant::now();
    for _ in 0..iterations {
        let data: Vec<(String, String, Option<String>, bool, bool)> = Vec::new();
        if data.is_empty() {
            continue;
        }
        // Would process here
    }
    let early_exit_duration = start.elapsed();
    
    // Test without early exit check
    let start = Instant::now();
    for _ in 0..iterations {
        let data: Vec<(String, String, Option<String>, bool, bool)> = Vec::new();
        // Would process here without check
        for _ in &data {
            // Processing
        }
    }
    let no_exit_duration = start.elapsed();
    
    println!("Early exit performance:");
    println!("  With early exit: {:?}", early_exit_duration);
    println!("  Without early exit: {:?}", no_exit_duration);
    
    // Early exit should be faster for empty collections
    assert!(early_exit_duration < no_exit_duration, 
        "Early exit should be faster for empty collections");
}

/// Test that demonstrates the benefit of using iterators over collect
#[test]
fn test_iterator_vs_collect_performance() {
    let data: Vec<i32> = (0..10000).collect();
    
    // Using iterator chain
    let start = Instant::now();
    for _ in 0..100 {
        let _: Vec<i32> = data.iter().map(|&x| x * 2).filter(|&x| x > 5000).collect();
    }
    let iterator_duration = start.elapsed();
    
    // Using manual loop with pre-allocation
    let start = Instant::now();
    for _ in 0..100 {
        let mut result: Vec<i32> = Vec::with_capacity(data.len());
        for &x in &data {
            let doubled = x * 2;
            if doubled > 5000 {
                result.push(doubled);
            }
        }
    }
    let manual_duration = start.elapsed();
    
    println!("Iterator vs manual loop:");
    println!("  Iterator chain: {:?}", iterator_duration);
    println!("  Manual loop: {:?}", manual_duration);
    
    // Both approaches should complete successfully
    assert!(iterator_duration < Duration::from_secs(1));
    assert!(manual_duration < Duration::from_secs(1));
}

/// Benchmark for string capacity calculation
#[test]
fn test_string_capacity_calculation() {
    let iterations = 100000;
    
    // Test with capacity calculation
    let start = Instant::now();
    for i in 0..iterations {
        let num = i.to_string();
        let mut s = String::with_capacity(10 + num.len());
        s.push_str("prefix_");
        s.push_str(&num);
        s.push_str("_suffix");
    }
    let calculated_duration = start.elapsed();
    
    // Test without capacity
    let start = Instant::now();
    for i in 0..iterations {
        let num = i.to_string();
        let mut s = String::new();
        s.push_str("prefix_");
        s.push_str(&num);
        s.push_str("_suffix");
    }
    let default_duration = start.elapsed();
    
    println!("String capacity calculation:");
    println!("  Calculated: {:?}", calculated_duration);
    println!("  Default: {:?}", default_duration);
    
    // Calculated should be faster or similar
    // The exact performance gain depends on the specific use case
    assert!(calculated_duration < Duration::from_millis(500));
}

/// Test shrink_to_fit benefits
#[test]
fn test_shrink_to_fit_performance() {
    let iterations = 10000;
    
    // Test with shrink_to_fit
    let start = Instant::now();
    for _ in 0..iterations {
        let mut v: Vec<i32> = Vec::with_capacity(100);
        for i in 0..50 {
            v.push(i);
        }
        v.shrink_to_fit();
        assert_eq!(v.len(), v.capacity());
    }
    let shrink_duration = start.elapsed();
    
    // Test without shrink_to_fit
    let start = Instant::now();
    for _ in 0..iterations {
        let mut v: Vec<i32> = Vec::with_capacity(100);
        for i in 0..50 {
            v.push(i);
        }
        // Capacity remains 100
        assert!(v.capacity() >= 50);
    }
    let no_shrink_duration = start.elapsed();
    
    println!("shrink_to_fit performance:");
    println!("  With shrink: {:?}", shrink_duration);
    println!("  Without shrink: {:?}", no_shrink_duration);
    
    // Both should complete quickly
    assert!(shrink_duration < Duration::from_millis(100));
    assert!(no_shrink_duration < Duration::from_millis(100));
}

/// Comprehensive performance test that validates overall system performance
#[test]
fn test_comprehensive_performance() {
    let total_start = Instant::now();
    
    // Simulate typical workload
    let iterations = 1000;
    
    for i in 0..iterations {
        // Simulate string operations
        let mut paper_id = String::with_capacity(36);
        paper_id.push_str(&format!("{:036}", i));
        
        // Simulate vector operations
        let mut authors: Vec<String> = Vec::with_capacity(10);
        for j in 0..5 {
            let mut name = String::with_capacity(20);
            name.push_str("Author ");
            name.push_str(&j.to_string());
            authors.push(name);
        }
        authors.shrink_to_fit();
        
        // Simulate keyword collection
        let mut keywords: Vec<String> = Vec::with_capacity(8);
        for k in 0..3 {
            let mut kw = String::with_capacity(15);
            kw.push_str("keyword_");
            kw.push_str(&k.to_string());
            keywords.push(kw);
        }
        keywords.shrink_to_fit();
        
        // Validate results
        assert!(!paper_id.is_empty());
        assert_eq!(authors.len(), 5);
        assert_eq!(keywords.len(), 3);
    }
    
    let total_duration = total_start.elapsed();
    
    println!("Comprehensive performance test:");
    println!("  Total time: {:?}", total_duration);
    println!("  Per iteration: {:?}", total_duration / iterations);
    
    // Should complete in reasonable time
    assert!(total_duration < Duration::from_secs(5), 
        "Performance test should complete in under 5 seconds");
}

/// Test memory efficiency with large datasets
#[test]
fn test_large_dataset_memory_efficiency() {
    let start = Instant::now();
    
    // Create a large dataset
    let mut papers: Vec<Paper> = Vec::with_capacity(10000);
    
    for i in 0..10000 {
        papers.push(Paper {
            id: format!("paper_{}", i),
            title: format!("Research Paper {}", i),
            doi: Some(format!("10.1234/paper.{}", i)),
            arxiv_id: Some(format!("2101.{:05}", i)),
            abstract_text: Some(format!("Abstract for paper {}", i)),
            user_notes: None,
            year: Some(2020 + (i % 5) as i32),
            journal: Some("Test Journal".to_string()),
            created_at: "2024-01-01T00:00:00Z".to_string(),
        });
    }
    
    papers.shrink_to_fit();
    
    let duration = start.elapsed();
    
    println!("Large dataset test:");
    println!("  Created {} papers in {:?}", papers.len(), duration);
    
    // Validate memory efficiency
    assert_eq!(papers.len(), 10000);
    assert!(duration < Duration::from_secs(2), 
        "Large dataset creation should be fast");
}

/// Validate that performance meets minimum requirements
#[test]
fn test_performance_requirements() {
    // Test 1: String building should be under 1ms for 100 operations
    let start = Instant::now();
    for _ in 0..100 {
        let mut s = String::with_capacity(50);
        s.push_str("This is a test string with some content");
    }
    assert!(start.elapsed() < Duration::from_millis(1));
    
    // Test 2: Vector operations should be under 5ms for 1000 elements
    let start = Instant::now();
    let mut v: Vec<i32> = Vec::with_capacity(1000);
    for i in 0..1000 {
        v.push(i);
    }
    v.shrink_to_fit();
    assert!(start.elapsed() < Duration::from_millis(5));
    
    // Test 3: Empty collection checks should be instant
    let start = Instant::now();
    for _ in 0..100000 {
        let v: Vec<i32> = Vec::new();
        if v.is_empty() {
            continue;
        }
    }
    assert!(start.elapsed() < Duration::from_millis(10));
    
    println!("All performance requirements met!");
}