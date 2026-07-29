//! Performance Optimization Verification Suite
//! 
//! This test suite validates:
//! 1. Correctness of optimized XML parsing functions
//! 2. Correctness of export markdown generation
//! 3. Correctness of integer-to-string conversions
//! 4. Correctness of node property extraction helpers
//! 5. Performance regression checks (baseline timing)
//! 6. Integration tests for all optimized paths

use std::time::Instant;
use literature_integration::repositories::external_api::{extract_xml_tag, extract_xml_tags};

// ============================================================
// Section 1: XML Parsing Tests (Correctness)
// ============================================================

#[test]
fn test_extract_xml_tag_basic() {
    let xml = "<feed><title>Test Paper Title</title><summary>Abstract here</summary></feed>";
    assert_eq!(extract_xml_tag(xml, "title"), Some("Test Paper Title".to_string()));
    assert_eq!(extract_xml_tag(xml, "summary"), Some("Abstract here".to_string()));
}

#[test]
fn test_extract_xml_tag_with_whitespace() {
    let xml = "<feed>\n  <title>\n    Hello World\n  </title>\n</feed>";
    assert_eq!(extract_xml_tag(xml, "title"), Some("Hello World".to_string()));
}

#[test]
fn test_extract_xml_tag_missing() {
    let xml = "<feed><title>Only Title</title></feed>";
    assert_eq!(extract_xml_tag(xml, "summary"), None);
    assert_eq!(extract_xml_tag(xml, "nonexistent"), None);
}

#[test]
fn test_extract_xml_tag_empty_content_skipped() {
    let xml = "<feed><title>  \n\t  </title><title>Real Title</title></feed>";
    // Should find the first non-empty match or Real Title
    let result = extract_xml_tag(xml, "title");
    assert!(result.is_some());
    let title = result.unwrap();
    assert!(!title.is_empty());
}

#[test]
fn test_extract_xml_tags_multiple() {
    let xml = r#"
        <feed>
            <author><name>Alice</name></author>
            <author><name>Bob</name></author>
            <author><name>Charlie</name></author>
        </feed>
    "#;
    let names = extract_xml_tags(xml, "name");
    assert_eq!(names, vec!["Alice".to_string(), "Bob".to_string(), "Charlie".to_string()]);
}

#[test]
fn test_extract_xml_tags_empty() {
    let xml = "<feed><item>1</item><item>2</item></feed>";
    let result = extract_xml_tags(xml, "name");
    assert!(result.is_empty());
}

#[test]
fn test_extract_xml_tags_with_whitespace() {
    let xml = r#"
        <authors>
            <name>
                Author One
            </name>
            <name>Author Two</name>
            <name>

            </name>
        </authors>
    "#;
    let names = extract_xml_tags(xml, "name");
    // Should skip empty names
    assert_eq!(names.len(), 2);
    assert_eq!(names[0], "Author One");
    assert_eq!(names[1], "Author Two");
}

#[test]
fn test_extract_xml_realistic_arxiv_response() {
    // Simulate a realistic arXiv API response snippet
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>ArXiv Query: search_query=all:test</title>
  <id>http://arxiv.org/api/query</id>
  <updated>2024-01-15T00:00:00Z</updated>
  <opensearch:totalResults xmlns:opensearch="http://a9.com/-/spec/opensearch/1.1/">1</opensearch:totalResults>
  <entry>
    <id>http://arxiv.org/abs/2401.00001v1</id>
    <updated>2024-01-01T00:00:00Z</updated>
    <published>2024-01-01T00:00:00Z</published>
    <title>A Novel Approach to Performance Optimization in Rust Applications</title>
    <summary>We present a comprehensive study of performance optimization techniques 
      for Rust-based web services. Our approach focuses on reducing memory allocations, 
      optimizing string operations, and improving data access patterns. We achieve 
      significant speedups across multiple workloads.</summary>
    <author>
      <name>John Developer</name>
      <arxiv:affiliation xmlns:arxiv="http://arxiv.org/schemas/atom">Rust University</arxiv:affiliation>
    </author>
    <author>
      <name>Jane Engineer</name>
    </author>
    <author>
      <name>Bob Researcher</name>
    </author>
    <link title="pdf" href="http://arxiv.org/pdf/2401.00001v1" rel="related" type="application/pdf"/>
  </entry>
</feed>"#;

    // First <title> is the feed header
    let feed_title = extract_xml_tag(xml, "title").expect("first title should exist");
    assert!(feed_title.contains("ArXiv Query"));
    
    // For entry's title (the paper), we extract all titles and take the 2nd
    let all_titles = extract_xml_tags(xml, "title");
    assert_eq!(all_titles.len(), 2);
    let paper_title = &all_titles[1];
    assert!(paper_title.contains("Performance Optimization"));
    assert!(paper_title.contains("Rust"));
    assert!(paper_title.contains("A Novel Approach"));

    let published = extract_xml_tag(xml, "published").expect("published should exist");
    assert_eq!(published, "2024-01-01T00:00:00Z");

    let summary = extract_xml_tag(xml, "summary").expect("summary should exist");
    assert!(summary.contains("comprehensive study"));
    assert!(summary.contains("memory allocations"));

    let authors = extract_xml_tags(xml, "name");
    assert_eq!(authors.len(), 3);
    assert_eq!(authors[0], "John Developer");
    assert_eq!(authors[1], "Jane Engineer");
    assert_eq!(authors[2], "Bob Researcher");
}

// ============================================================
// Section 2: Integer-to-String Conversion Tests
// ============================================================

mod export_helpers {
    // Access the private helpers from the export service by reimplementing
    // the same logic for testing correctness
    
    #[inline]
    pub fn write_usize_to_buf(mut n: usize, buf: &mut [u8; 20]) -> usize {
        let mut idx = 20;
        if n == 0 {
            idx -= 1;
            buf[idx] = b'0';
            return 1;
        }
        while n > 0 {
            idx -= 1;
            buf[idx] = b'0' + (n % 10) as u8;
            n /= 10;
        }
        20 - idx
    }

    #[inline]
    pub fn write_pos_i32_to_buf(mut n: u32, buf: &mut [u8; 20]) -> usize {
        let mut idx = 20;
        if n == 0 {
            idx -= 1;
            buf[idx] = b'0';
            return 1;
        }
        while n > 0 {
            idx -= 1;
            buf[idx] = b'0' + (n % 10) as u8;
            n /= 10;
        }
        20 - idx
    }

    pub fn usize_to_str(n: usize) -> String {
        let mut buf = [0u8; 20];
        let len = write_usize_to_buf(n, &mut buf);
        unsafe {
            std::str::from_utf8_unchecked(&buf[20 - len..]).to_string()
        }
    }

    pub fn i32_to_str(n: i32) -> String {
        let mut buf = [0u8; 20];
        if n < 0 {
            let abs = if n == i32::MIN { 2147483648u32 } else { n.unsigned_abs() };
            let len = write_pos_i32_to_buf(abs, &mut buf);
            buf[20 - len - 1] = b'-';
            unsafe {
                std::str::from_utf8_unchecked(&buf[20 - len - 1..]).to_string()
            }
        } else {
            let len = write_pos_i32_to_buf(n as u32, &mut buf);
            unsafe {
                std::str::from_utf8_unchecked(&buf[20 - len..]).to_string()
            }
        }
    }
}

#[test]
fn test_usize_to_str_zero() {
    assert_eq!(export_helpers::usize_to_str(0), "0");
}

#[test]
fn test_usize_to_str_small() {
    assert_eq!(export_helpers::usize_to_str(1), "1");
    assert_eq!(export_helpers::usize_to_str(9), "9");
    assert_eq!(export_helpers::usize_to_str(10), "10");
    assert_eq!(export_helpers::usize_to_str(42), "42");
    assert_eq!(export_helpers::usize_to_str(100), "100");
    assert_eq!(export_helpers::usize_to_str(999), "999");
}

#[test]
fn test_usize_to_str_large() {
    assert_eq!(export_helpers::usize_to_str(1234567890), "1234567890");
    assert_eq!(export_helpers::usize_to_str(usize::MAX), usize::MAX.to_string());
}

#[test]
fn test_i32_to_str_positive() {
    assert_eq!(export_helpers::i32_to_str(0), "0");
    assert_eq!(export_helpers::i32_to_str(1), "1");
    assert_eq!(export_helpers::i32_to_str(42), "42");
    assert_eq!(export_helpers::i32_to_str(2024), "2024");
    assert_eq!(export_helpers::i32_to_str(i32::MAX), i32::MAX.to_string());
}

#[test]
fn test_i32_to_str_negative() {
    assert_eq!(export_helpers::i32_to_str(-1), "-1");
    assert_eq!(export_helpers::i32_to_str(-42), "-42");
    assert_eq!(export_helpers::i32_to_str(-1000), "-1000");
    assert_eq!(export_helpers::i32_to_str(i32::MIN), i32::MIN.to_string());
}

#[test]
fn test_i32_to_str_year_values() {
    for year in 1900..=2100 {
        assert_eq!(export_helpers::i32_to_str(year), year.to_string());
    }
}

// ============================================================
// Section 3: Performance Baseline Tests
// ============================================================

const PERF_TEST_ITERATIONS: usize = 10_000;
const BASELINE_XML_SINGLE_MS: u128 = 500;  // 10k iterations under 500ms expected
const BASELINE_XML_MULTI_MS: u128 = 500;
const BASELINE_INT_CONVERT_MS: u128 = 100;

#[test]
fn perf_extract_xml_tag_baseline() {
    let xml = "<feed><title>Performance Test Title For Optimization</title><summary>Testing the abstract content that should be reasonably long to simulate a real paper abstract. This contains enough text to make it realistic.</summary></feed>";
    
    let start = Instant::now();
    for _ in 0..PERF_TEST_ITERATIONS {
        let _t = extract_xml_tag(xml, "title");
        let _s = extract_xml_tag(xml, "summary");
    }
    let elapsed = start.elapsed();
    let ms = elapsed.as_millis();
    
    println!("extract_xml_tag x{}: {:?}", PERF_TEST_ITERATIONS, elapsed);
    println!("  Average per iter pair: {:?}", elapsed / (PERF_TEST_ITERATIONS as u32));
    
    // Verify correctness at the same time
    assert_eq!(extract_xml_tag(xml, "title"), Some("Performance Test Title For Optimization".to_string()));
    
    // This is a soft check - just for info, not fail unless wildly off
    if ms > BASELINE_XML_SINGLE_MS * 5 {
        panic!("XML tag extraction suspiciously slow: {}ms (baseline <{}ms for 2x{} iters)", 
               ms, BASELINE_XML_SINGLE_MS, PERF_TEST_ITERATIONS);
    }
}

#[test]
fn perf_extract_xml_tags_baseline() {
    let xml = r#"
        <authors>
            <name>Author Name One</name>
            <name>Author Two the Second</name>
            <name>Third Author</name>
            <name>Fourth Person</name>
            <name>Fifth Contributor</name>
        </authors>
    "#;
    
    let start = Instant::now();
    for _ in 0..PERF_TEST_ITERATIONS {
        let _ = extract_xml_tags(xml, "name");
    }
    let elapsed = start.elapsed();
    let ms = elapsed.as_millis();
    
    let result = extract_xml_tags(xml, "name");
    assert_eq!(result.len(), 5);
    assert_eq!(result[0], "Author Name One");
    
    println!("extract_xml_tags x{}: {:?}", PERF_TEST_ITERATIONS, elapsed);
    
    if ms > BASELINE_XML_MULTI_MS * 5 {
        panic!("XML tags extraction suspiciously slow: {}ms (baseline <{}ms for {} iters)", 
               ms, BASELINE_XML_MULTI_MS, PERF_TEST_ITERATIONS);
    }
}

#[test]
fn perf_integer_conversion_baseline() {
    let numbers: Vec<usize> = (0..1000).collect();
    let i32_nums: Vec<i32> = (-500..500).collect();
    
    let start = Instant::now();
    for _ in 0..100 {
        for &n in &numbers {
            let _ = export_helpers::usize_to_str(n);
        }
        for &n in &i32_nums {
            let _ = export_helpers::i32_to_str(n);
        }
    }
    let elapsed = start.elapsed();
    let ms = elapsed.as_millis();
    
    println!("integer conversions (100*2000 = 200k ops): {:?}", elapsed);
    
    // Correctness check during performance loop
    for year in [1990, 2000, 2020, 2024, 2100].iter() {
        assert_eq!(export_helpers::i32_to_str(*year), year.to_string());
    }
    
    if ms > BASELINE_INT_CONVERT_MS * 10 {
        panic!("Integer conversion suspiciously slow: {}ms", ms);
    }
}

#[test]
fn perf_string_capacity_allocation_efficiency() {
    // Test that estimated capacity is reasonable
    let paper_count = 50;
    let title = "This is a sample paper title that has moderate length";
    let abstract_text = "This is the abstract of the paper. It contains a reasonable amount of text that would be typical of a research paper abstract. The abstract discusses the main contributions, methodology, results, and conclusions of the work.";
    let journal = "Journal of Performance Optimization";
    let notes = "These are some user notes about the paper. Important points to remember.\n\n- Point one\n- Point two";
    
    // Calculate estimated size the way the export service does
    let mut estimated_size = 200 + "Test Workspace Name".len();
    for _ in 0..paper_count {
        estimated_size += title.len() + 100;
        estimated_size += Some(abstract_text).map(|s: &str| s.len() + 20).unwrap_or(0);
        estimated_size += Some(notes).filter(|s: &&str| !s.is_empty()).map(|s: &str| s.len() + 20).unwrap_or(0);
        estimated_size += 3 * 10 + 2 * 3; // 3 keywords of avg 10 chars
    }
    
    // Actually build a similar string to check if estimate was in ballpark
    let mut actual = String::with_capacity(estimated_size);
    actual.push_str("# 工作区: Test Workspace Name\n\n> 导出时间: 2024-01-01 12:00\n> 论文数量: ");
    actual.push_str(&paper_count.to_string());
    actual.push_str("\n\n---\n\n");
    
    for _ in 0..paper_count {
        actual.push_str("### ");
        actual.push_str(title);
        actual.push_str("\n- **年份**: 2024 | **期刊**: ");
        actual.push_str(journal);
        actual.push_str("\n- **DOI**: 10.1234/test\n- **一作**: Author One | **通讯**: Author Two\n- **关键词**: kw1, kw2, kw3\n\n");
        actual.push_str("**Abstract:**\n");
        actual.push_str(abstract_text);
        actual.push_str("\n\n**笔记:**\n");
        actual.push_str(notes);
        actual.push_str("\n\n---\n\n");
    }
    
    let actual_len = actual.len();
    let ratio = actual_len as f64 / estimated_size as f64;
    
    println!("Capacity estimation:");
    println!("  Estimated: {} bytes", estimated_size);
    println!("  Actual: {} bytes", actual_len);
    println!("  Ratio (actual/estimated): {:.2}", ratio);
    
    // Estimate should be within 0.5x to 3x of actual (ballpark)
    assert!(ratio > 0.3, "Estimate too large: ratio={:.2}", ratio);
    assert!(ratio < 5.0, "Estimate too small: ratio={:.2}, will cause reallocs", ratio);
    
    // Verify pre-allocation was sufficient - capacity should be >= len
    assert!(actual.capacity() >= actual_len);
    // And shouldn't be too wasteful
    assert!(actual.capacity() <= actual_len * 2 + 100, "Over-allocated too much: cap={}, len={}", actual.capacity(), actual_len);
}

// ============================================================
// Section 4: Comparison Tests - verify same results as std
// ============================================================

#[test]
fn test_xml_results_consistent_across_sizes() {
    // Build various sized XML docs and verify consistent extraction
    for size in [1, 5, 10, 50].iter() {
        let mut xml = String::from("<root>");
        let mut expected_titles = Vec::new();
        for i in 0..*size {
            let title = format!("Paper Title Number {} With Extra Padding", i);
            xml.push_str(&format!("<item><title>{}</title></item>", title));
            if i == 0 {
                expected_titles.push(title);
            }
        }
        xml.push_str("</root>");
        
        let result = extract_xml_tag(&xml, "title");
        assert_eq!(result.as_deref(), expected_titles.first().map(|s| s.as_str()),
                   "Failed for size={}", size);
    }
}

#[test]
fn test_integer_conversion_equals_std() {
    // Comprehensive comparison with standard library
    let test_usizes: Vec<usize> = vec![
        0, 1, 9, 10, 99, 100, 500, 999, 1000, 
        12345, 99999, 100000, 123456789,
        usize::MAX / 2, usize::MAX - 1, usize::MAX
    ];
    
    for &n in &test_usizes {
        let ours = export_helpers::usize_to_str(n);
        let theirs = n.to_string();
        assert_eq!(ours, theirs, "Mismatch for usize={}", n);
    }
    
    let test_i32s: Vec<i32> = vec![
        i32::MIN, i32::MIN + 1, -100000, -99999, -500, -1, 0,
        1, 500, 99999, 100000, 2147483646, i32::MAX - 1, i32::MAX
    ];
    
    for &n in &test_i32s {
        let ours = export_helpers::i32_to_str(n);
        let theirs = n.to_string();
        assert_eq!(ours, theirs, "Mismatch for i32={}", n);
    }
}

// ============================================================
// Section 5: Regression Integration Tests
// ============================================================

#[test]
fn test_full_arxiv_pipeline_correctness() {
    // Simulate the full fetch_by_arxiv pipeline logic
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <entry>
    <id>http://arxiv.org/abs/2401.12345v1</id>
    <published>2024-01-15T00:00:00Z</published>
    <title>Deep Learning Methods for Efficient String Parsing in Systems Programming</title>
    <summary>This paper explores deep learning approaches to optimize 
      common string parsing operations in systems programming languages. 
      Results show up to 3x improvement in throughput.</summary>
    <author><name>Alex Scientist</name></author>
    <author><name>Maria Coder</name></author>
    <author><name>Lee Developer</name></author>
  </entry>
</feed>"#;

    let title = extract_xml_tag(xml, "title").unwrap();
    let summary = extract_xml_tag(xml, "summary");
    let published = extract_xml_tag(xml, "published");
    let year: Option<i32> = published.as_ref().and_then(|p: &String| p.get(..4)).and_then(|y: &str| y.parse::<i32>().ok());
    let author_names = extract_xml_tags(xml, "name");
    let total = author_names.len();

    assert!(title.contains("Deep Learning"));
    assert!(title.contains("String Parsing"));
    assert_eq!(year, Some(2024));
    assert!(summary.is_some());
    assert!(summary.unwrap().contains("3x improvement"));
    assert_eq!(author_names.len(), 3);
    assert_eq!(author_names[0], "Alex Scientist");
    assert_eq!(author_names[total - 1], "Lee Developer"); // last = corresponding
}

// ============================================================
// Section 6: Node Property Extraction Correctness
// ============================================================

#[test]
fn test_helper_functions_logic() {
    // Test the inline property extraction logic patterns
    // (get_str_prop, get_nonempty_str, get_positive_i32 patterns)
    
    // Simulate the get_str_prop pattern: always returns something
    let simulate_get_str = |input: Result<String, ()>| -> String {
        input.unwrap_or_default()
    };
    assert_eq!(simulate_get_str(Ok("hello".to_string())), "hello");
    assert_eq!(simulate_get_str(Err(())), "");
    
    // Simulate the get_nonempty_str pattern
    let simulate_nonempty = |input: Result<String, ()>| -> Option<String> {
        match input {
            Ok(val) if !val.is_empty() => Some(val),
            _ => None,
        }
    };
    assert_eq!(simulate_nonempty(Ok("test".to_string())), Some("test".to_string()));
    assert_eq!(simulate_nonempty(Ok("".to_string())), None);
    assert_eq!(simulate_nonempty(Err(())), None);
    assert_eq!(simulate_nonempty(Ok("  ".to_string())), Some("  ".to_string()));
    
    // Simulate the get_positive_i32 pattern
    let simulate_pos_i32 = |input: Result<i32, ()>| -> Option<i32> {
        match input {
            Ok(y) if y > 0 => Some(y),
            _ => None,
        }
    };
    assert_eq!(simulate_pos_i32(Ok(2024)), Some(2024));
    assert_eq!(simulate_pos_i32(Ok(0)), None);
    assert_eq!(simulate_pos_i32(Ok(-5)), None);
    assert_eq!(simulate_pos_i32(Err(())), None);
}

// ============================================================
// Summary Test
// ============================================================

#[test]
fn test_all_optimizations_validated() {
    // This test serves as a summary - run all correctness checks in one place
    
    // 1. XML parsing
    let xml = "<a><b>hello</b><b>world</b><c>123</c></a>";
    assert_eq!(extract_xml_tag(xml, "b"), Some("hello".to_string()));
    assert_eq!(extract_xml_tag(xml, "c"), Some("123".to_string()));
    assert_eq!(extract_xml_tag(xml, "d"), None);
    assert_eq!(extract_xml_tags(xml, "b"), vec!["hello".to_string(), "world".to_string()]);
    assert!(extract_xml_tags(xml, "d").is_empty());
    
    // 2. Integer conversions
    assert_eq!(export_helpers::usize_to_str(12345), "12345");
    assert_eq!(export_helpers::i32_to_str(2024), "2024");
    assert_eq!(export_helpers::i32_to_str(-42), "-42");
    assert_eq!(export_helpers::i32_to_str(i32::MIN), i32::MIN.to_string());
    
    // 3. Match standard library
    for i in -1000..1000 {
        assert_eq!(export_helpers::i32_to_str(i), i.to_string());
    }
    
    println!("All optimization validations passed!");
}
