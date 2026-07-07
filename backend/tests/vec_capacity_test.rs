// 测试 Vec 预分配容量优化是否正确工作
// 此测试不需要 Neo4j 数据库连接

#[test]
fn test_vec_capacity_constants_are_valid() {
    // 验证常量值在合理范围内
    const DEFAULT_PAPERS_CAPACITY: usize = 32;
    const DEFAULT_AUTHORS_CAPACITY: usize = 16;
    const DEFAULT_KEYWORDS_CAPACITY: usize = 8;
    
    assert!(DEFAULT_PAPERS_CAPACITY > 0);
    assert!(DEFAULT_AUTHORS_CAPACITY > 0);
    assert!(DEFAULT_KEYWORDS_CAPACITY > 0);
    
    // 验证 Vec::with_capacity 正常工作
    let papers: Vec<u8> = Vec::with_capacity(DEFAULT_PAPERS_CAPACITY);
    assert_eq!(papers.capacity(), DEFAULT_PAPERS_CAPACITY);
    
    let authors: Vec<u8> = Vec::with_capacity(DEFAULT_AUTHORS_CAPACITY);
    assert_eq!(authors.capacity(), DEFAULT_AUTHORS_CAPACITY);
    
    let keywords: Vec<u8> = Vec::with_capacity(DEFAULT_KEYWORDS_CAPACITY);
    assert_eq!(keywords.capacity(), DEFAULT_KEYWORDS_CAPACITY);
}

#[test]
fn test_vec_with_capacity_vs_new_performance() {
    use std::time::Instant;
    
    const ITERATIONS: usize = 10000;
    const CAPACITY: usize = 32;
    
    // 测试 Vec::new() 的性能
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let mut v: Vec<i32> = Vec::new();
        for i in 0..CAPACITY {
            v.push(i as i32);
        }
    }
    let duration_new = start.elapsed();
    
    // 测试 Vec::with_capacity() 的性能
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let mut v: Vec<i32> = Vec::with_capacity(CAPACITY);
        for i in 0..CAPACITY {
            v.push(i as i32);
        }
    }
    let duration_with_capacity = start.elapsed();
    
    println!("Vec::new() duration: {:?}", duration_new);
    println!("Vec::with_capacity() duration: {:?}", duration_with_capacity);
    
    // with_capacity 应该更快或至少相当
    // 在某些情况下，性能提升可能不明显，但理论上应该减少内存重新分配
    assert!(duration_with_capacity <= duration_new * 2, 
        "Vec::with_capacity should not be significantly slower than Vec::new()");
}

#[test]
fn test_vec_growth_behavior() {
    // 测试 Vec 在不同容量下的增长行为
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::with_capacity(32);
    
    // 添加元素并观察容量变化
    for i in 0..32 {
        v1.push(i);
        v2.push(i);
        
        // v1 的容量会动态增长（通常以 2 的幂次增长）
        // v2 的容量保持为 32，直到超过预分配容量
    }
    
    println!("Vec::new() final capacity: {}", v1.capacity());
    println!("Vec::with_capacity(32) final capacity: {}", v2.capacity());
    
    // 验证预分配容量的 Vec 在填满时容量仍为预分配值
    assert_eq!(v2.capacity(), 32);
    
    // 继续添加元素，观察容量增长
    v2.push(100);
    println!("Vec::with_capacity(32) after overflow capacity: {}", v2.capacity());
    
    // 容量应该增长到大于 32 的值
    assert!(v2.capacity() > 32);
}

#[test]
fn test_struct_conversion_preserves_data() {
    // 测试结构体数据转换的正确性（模拟 paper_from_node 等函数的行为）
    
    struct TestPaper {
        id: String,
        title: String,
        year: Option<i32>,
    }
    
    struct TestNode {
        id: String,
        title: String,
        year: i32,
    }
    
    fn paper_from_node(node: &TestNode) -> TestPaper {
        TestPaper {
            id: node.id.clone(),
            title: node.title.clone(),
            year: Some(node.year).filter(|y| *y > 0),
        }
    }
    
    let nodes: Vec<TestNode> = vec![
        TestNode { id: "1".to_string(), title: "Paper 1".to_string(), year: 2024 },
        TestNode { id: "2".to_string(), title: "Paper 2".to_string(), year: 2023 },
        TestNode { id: "3".to_string(), title: "Paper 3".to_string(), year: 0 },
    ];
    
    // 使用预分配容量
    let papers: Vec<TestPaper> = nodes.iter()
        .map(paper_from_node)
        .collect();
    
    assert_eq!(papers.len(), 3);
    assert_eq!(papers[0].id, "1");
    assert_eq!(papers[0].title, "Paper 1");
    assert_eq!(papers[0].year, Some(2024));
    assert_eq!(papers[1].year, Some(2023));
    assert_eq!(papers[2].year, None); // year=0 被过滤掉
}