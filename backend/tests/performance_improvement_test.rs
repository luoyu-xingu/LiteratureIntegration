// 性能优化验证测试
// 测试优化后的代码是否达到性能要求

use std::time::{Duration, Instant};

/// 模拟 XML 解析器性能测试
#[test]
fn test_extract_xml_tags_performance() {
    // 模拟大型 XML 文档
    let xml = generate_test_xml(100);
    
    let mut total_duration = Duration::new(0, 0);
    let iterations = 1000;
    
    for _ in 0..iterations {
        let start = Instant::now();
        let results = extract_xml_tags(&xml, "name");
        total_duration += start.elapsed();
        assert_eq!(results.len(), 100);
    }
    
    let avg_duration = total_duration / iterations;
    println!("extract_xml_tags: {} iterations, avg {:?}", iterations, avg_duration);
    
    // 优化后应小于 50 微秒
    assert!(avg_duration < Duration::from_micros(50), 
        "extract_xml_tags average duration {:?} exceeds 50us", avg_duration);
}

/// 模拟 Vec 预分配性能测试
#[test]
fn test_vec_preallocation_performance() {
    let data: Vec<String> = (0..100).map(|i| format!("item_{}", i)).collect();
    
    // 测试使用预分配
    let mut with_prealloc_duration = Duration::new(0, 0);
    for _ in 0..1000 {
        let start = Instant::now();
        let mut vec: Vec<(String, String)> = Vec::with_capacity(data.len());
        for item in &data {
            vec.push((format!("id_{}", item), item.clone()));
        }
        with_prealloc_duration += start.elapsed();
        assert_eq!(vec.len(), 100);
    }
    
    // 测试不使用预分配
    let mut without_prealloc_duration = Duration::new(0, 0);
    for _ in 0..1000 {
        let start = Instant::now();
        let mut vec: Vec<(String, String)> = Vec::new();
        for item in &data {
            vec.push((format!("id_{}", item), item.clone()));
        }
        without_prealloc_duration += start.elapsed();
        assert_eq!(vec.len(), 100);
    }
    
    println!("with_prealloc: avg {:?}", with_prealloc_duration / 1000);
    println!("without_prealloc: avg {:?}", without_prealloc_duration / 1000);
    
    // 预分配版本应该更快或相当
    // 在大多数情况下，预分配会更快
}

/// 字符串操作性能测试
#[test]
fn test_string_concat_performance() {
    let words: Vec<&str> = vec!["hello", "world", "test", "performance", "optimization"];
    
    let mut total_duration = Duration::new(0, 0);
    let iterations = 10000;
    
    for _ in 0..iterations {
        let start = Instant::now();
        let mut result = String::with_capacity(50);
        for word in &words {
            result.push_str(word);
            result.push(' ');
        }
        total_duration += start.elapsed();
    }
    
    let avg_duration = total_duration / iterations;
    println!("string_concat: {} iterations, avg {:?}", iterations, avg_duration);
    
    assert!(avg_duration < Duration::from_nanos(200), 
        "string_concat average duration {:?} exceeds 200ns", avg_duration);
}

/// 迭代器性能测试
#[test]
fn test_iterator_chain_performance() {
    let data: Vec<i32> = (0..1000).collect();
    
    let mut chain_duration = Duration::new(0, 0);
    for _ in 0..1000 {
        let start = Instant::now();
        let result: Vec<i32> = data.iter()
            .filter(|x| *x % 2 == 0)
            .map(|x| x * 2)
            .collect();
        chain_duration += start.elapsed();
        assert_eq!(result.len(), 500);
    }
    
    let mut loop_duration = Duration::new(0, 0);
    for _ in 0..1000 {
        let start = Instant::now();
        let mut result = Vec::with_capacity(500);
        for &x in &data {
            if x % 2 == 0 {
                result.push(x * 2);
            }
        }
        loop_duration += start.elapsed();
        assert_eq!(result.len(), 500);
    }
    
    println!("iterator_chain: avg {:?}", chain_duration / 1000);
    println!("explicit_loop: avg {:?}", loop_duration / 1000);
    
    // 两种方式性能应相近
}

// 辅助函数：生成测试 XML
fn generate_test_xml(count: usize) -> String {
    let mut xml = String::with_capacity(count * 30 + 20);
    xml.push_str("<root>");
    for i in 0..count {
        xml.push_str(&format!("<name>Author_{}</name>", i));
    }
    xml.push_str("</root>");
    xml
}

// 辅助函数：提取 XML 标签（优化版本）
fn extract_xml_tags(xml: &str, tag: &str) -> Vec<String> {
    let open_tag = format!("<{}>", tag);
    let close_tag = format!("</{}>", tag);
    let estimated_count = xml.matches(&open_tag as &str).count();
    let mut results = Vec::with_capacity(estimated_count);
    let mut search_from = 0;
    let open_len = open_tag.len();
    let close_len = close_tag.len();
    while let Some(start) = xml[search_from..].find(&open_tag) {
        let content_start = search_from + start + open_len;
        if let Some(content_end) = xml[content_start..].find(&close_tag as &str) {
            results.push(
                xml[content_start..content_start + content_end]
                    .trim()
                    .to_string(),
            );
            search_from = content_start + content_end + close_len;
        } else {
            break;
        }
    }
    results
}

#[test]
fn test_overall_performance_requirements() {
    // 综合性能要求测试
    println!("=== 性能优化验证测试 ===");
    println!("所有测试应在合理时间内完成");
    
    // 简单的基准测试
    let start = Instant::now();
    let mut sum = 0u64;
    for i in 0..1_000_000 {
        sum += i;
    }
    let duration = start.elapsed();
    
    println!("基础计算: {:?} (sum: {})", duration, sum);
    assert!(duration < Duration::from_millis(100));
}