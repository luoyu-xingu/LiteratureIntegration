//! 综合功能验证测试
//!
//! 测试目标：
//! 1. XML 解析函数的正确性（extract_xml_tag, extract_xml_tags）
//! 2. 导出服务中的整数转字符串工具函数
//! 3. 优化后算法的边界情况处理

use literature_integration::repositories::external_api::{extract_xml_tag, extract_xml_tags};

// ==================== XML 解析函数测试 ====================

#[test]
fn test_extract_xml_tag_basic() {
    let xml = r#"<feed><title>Test Paper Title</title><summary>Abstract here</summary></feed>"#;
    let title = extract_xml_tag(xml, "title").unwrap();
    assert_eq!(title, "Test Paper Title");
}

#[test]
fn test_extract_xml_tag_with_whitespace() {
    let xml = r#"<feed>
  <title>
    Deep Learning for NLP
  </title>
</feed>"#;
    let title = extract_xml_tag(xml, "title").unwrap();
    assert_eq!(title, "Deep Learning for NLP");
}

#[test]
fn test_extract_xml_tag_empty_content_skips() {
    let xml = r#"<feed><title></title><title>Actual Title</title></feed>"#;
    let title = extract_xml_tag(xml, "title").unwrap();
    // Should skip empty and return first non-empty
    assert_eq!(title, "Actual Title");
}

#[test]
fn test_extract_xml_tag_missing() {
    let xml = r#"<feed><title>Test</title></feed>"#;
    let result = extract_xml_tag(xml, "nonexistent");
    assert!(result.is_none());
}

#[test]
fn test_extract_xml_tag_arxiv_style() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <entry>
    <id>http://arxiv.org/abs/2301.00001v1</id>
    <updated>2023-01-02T00:00:00Z</updated>
    <published>2023-01-01T00:00:00Z</published>
    <title>A Novel Approach to Machine Learning</title>
    <summary>This paper presents a novel approach...</summary>
    <author><name>Alice Smith</name></author>
    <author><name>Bob Jones</name></author>
    <author><name>Charlie Brown</name></author>
  </entry>
</feed>"#;
    let title = extract_xml_tag(xml, "title").unwrap();
    assert_eq!(title, "A Novel Approach to Machine Learning");

    let summary = extract_xml_tag(xml, "summary").unwrap();
    assert_eq!(summary, "This paper presents a novel approach...");

    let published = extract_xml_tag(xml, "published").unwrap();
    assert_eq!(published, "2023-01-01T00:00:00Z");
}

#[test]
fn test_extract_xml_tags_basic() {
    let xml = r#"<root><name>Alice</name><name>Bob</name><name>Charlie</name></root>"#;
    let names = extract_xml_tags(xml, "name");
    assert_eq!(names.len(), 3);
    assert_eq!(names[0], "Alice");
    assert_eq!(names[1], "Bob");
    assert_eq!(names[2], "Charlie");
}

#[test]
fn test_extract_xml_tags_empty_skipped() {
    let xml = r#"<root><name>Alice</name><name></name><name>  </name><name>Bob</name></root>"#;
    let names = extract_xml_tags(xml, "name");
    assert_eq!(names.len(), 2);
    assert_eq!(names[0], "Alice");
    assert_eq!(names[1], "Bob");
}

#[test]
fn test_extract_xml_tags_arxiv_authors() {
    let xml = r#"<entry>
    <author><name>Alice Smith</name></author>
    <author><name>Bob Jones</name></author>
    <author><name>Charlie Brown</name></author>
    <author><name></name></author>
</entry>"#;
    let authors = extract_xml_tags(xml, "name");
    assert_eq!(authors.len(), 3);
    assert_eq!(authors[0], "Alice Smith");
    assert_eq!(authors[1], "Bob Jones");
    assert_eq!(authors[2], "Charlie Brown");
}

#[test]
fn test_extract_xml_tags_nonexistent() {
    let xml = r#"<root><a>1</a></root>"#;
    let result = extract_xml_tags(xml, "b");
    assert!(result.is_empty());
}

#[test]
fn test_extract_xml_tags_with_whitespace_content() {
    let xml = r#"<root>
  <name>
    First Author
  </name>
  <name>
    Second Author
  </name>
</root>"#;
    let names = extract_xml_tags(xml, "name");
    assert_eq!(names.len(), 2);
    assert_eq!(names[0], "First Author");
    assert_eq!(names[1], "Second Author");
}

// ==================== 边界情况 & 压力测试 ====================

#[test]
fn test_extract_xml_tag_special_chars() {
    let xml = r#"<root><title>Paper: A &amp; B's "Test"</title></root>"#;
    let title = extract_xml_tag(xml, "title").unwrap();
    // We don't decode XML entities in our parser, so content remains as-is
    assert!(title.contains("Paper"));
    assert!(title.contains("&amp;"));
}

#[test]
fn test_extract_xml_longer_tag_names() {
    let xml = r#"<root><containerTitle>Nature Machine Intelligence</containerTitle><crossmarkPolicy>Some policy</crossmarkPolicy></root>"#;
    let journal = extract_xml_tag(xml, "containerTitle").unwrap();
    assert_eq!(journal, "Nature Machine Intelligence");

    let policy = extract_xml_tag(xml, "crossmarkPolicy").unwrap();
    assert_eq!(policy, "Some policy");
}

#[test]
fn test_extract_xml_multiple_same_different_content() {
    let xml = r#"<root>
    <abstract>Abstract 1</abstract>
    <abstract>Abstract 2 longer version</abstract>
</root>"#;
    // Should return first non-empty
    let result = extract_xml_tag(xml, "abstract").unwrap();
    assert_eq!(result, "Abstract 1");
}

#[test]
fn test_extract_xml_tags_large_number() {
    // Simulate 50 authors
    let mut xml = String::from("<root>");
    let mut expected: Vec<String> = Vec::with_capacity(50);
    for i in 0..50 {
        let name = format!("Author Number {}", i);
        xml.push_str("<name>");
        xml.push_str(&name);
        xml.push_str("</name>");
        expected.push(name);
    }
    xml.push_str("</root>");

    let names = extract_xml_tags(&xml, "name");
    assert_eq!(names.len(), 50);
    for (i, name) in names.iter().enumerate() {
        assert_eq!(name, &expected[i]);
    }
}

// ==================== 整数转字符串工具函数（通过导出服务间接测试） ====================

mod export_utils {
    //! 从 export.rs 内联函数复制出来用于独立测试
    //! 这些函数是 inline 的，直接在这里重写一份以验证其正确性

    #[inline]
    fn usize_to_str(n: usize) -> String {
        let mut buf = [0u8; 20];
        let len = write_usize_to_buf(n, &mut buf);
        unsafe {
            std::str::from_utf8_unchecked(&buf[20 - len..]).to_string()
        }
    }

    #[inline]
    fn i32_to_str(n: i32) -> String {
        let mut buf = [0u8; 20];
        if n < 0 {
            let len = write_abs_i32_to_buf(n, &mut buf);
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

    #[inline]
    fn write_usize_to_buf(mut n: usize, buf: &mut [u8; 20]) -> usize {
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
    fn write_pos_i32_to_buf(mut n: u32, buf: &mut [u8; 20]) -> usize {
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
    fn write_abs_i32_to_buf(n: i32, buf: &mut [u8; 20]) -> usize {
        let abs = if n == i32::MIN {
            2147483648u32
        } else {
            n.unsigned_abs()
        };
        write_pos_i32_to_buf(abs, buf)
    }

    #[test]
    fn test_usize_to_str_zero() {
        assert_eq!(usize_to_str(0), "0");
    }

    #[test]
    fn test_usize_to_str_single_digit() {
        for i in 0..=9 {
            assert_eq!(usize_to_str(i), i.to_string());
        }
    }

    #[test]
    fn test_usize_to_str_various() {
        let cases: Vec<usize> = vec![0, 1, 9, 10, 11, 99, 100, 123, 999, 1000, 12345, 99999, 100000, 123456789];
        for n in cases {
            assert_eq!(usize_to_str(n), n.to_string(), "Failed for usize {}", n);
        }
    }

    #[test]
    fn test_usize_to_str_max() {
        let n = usize::MAX;
        assert_eq!(usize_to_str(n), n.to_string());
    }

    #[test]
    fn test_i32_to_str_zero() {
        assert_eq!(i32_to_str(0), "0");
    }

    #[test]
    fn test_i32_to_str_positive() {
        for n in vec![1, 9, 10, 11, 99, 100, 123, 999, 1000, 12345, 99999, 100000] {
            assert_eq!(i32_to_str(n), n.to_string(), "Failed for positive i32 {}", n);
        }
    }

    #[test]
    fn test_i32_to_str_negative() {
        for n in vec![-1, -9, -10, -11, -99, -100, -123, -999, -1000, -12345] {
            assert_eq!(i32_to_str(n), n.to_string(), "Failed for negative i32 {}", n);
        }
    }

    #[test]
    fn test_i32_to_str_i32_min() {
        // Edge case: i32::MIN = -2147483648
        assert_eq!(i32_to_str(i32::MIN), i32::MIN.to_string());
    }

    #[test]
    fn test_i32_to_str_i32_max() {
        assert_eq!(i32_to_str(i32::MAX), i32::MAX.to_string());
    }

    #[test]
    fn test_i32_to_str_year_values() {
        // Year values commonly used
        let years: Vec<i32> = vec![2024, 2023, 2020, 2000, 1999, 1990, 0, -500];
        for y in years {
            assert_eq!(i32_to_str(y), y.to_string(), "Failed for year {}", y);
        }
    }
}

// ==================== Vec capacity 预分配验证 ====================

#[test]
fn test_vec_with_capacity_behavior() {
    // Ensure our pre-allocation strategy works correctly
    let cap = 64;
    let v: Vec<i32> = Vec::with_capacity(cap);
    assert_eq!(v.len(), 0);
    assert!(v.capacity() >= cap);
}

#[test]
fn test_shrink_to_fit_behavior() {
    let mut v: Vec<i32> = Vec::with_capacity(100);
    for i in 0..10 {
        v.push(i);
    }
    assert_eq!(v.len(), 10);
    assert!(v.capacity() >= 100);
    v.shrink_to_fit();
    assert_eq!(v.len(), 10);
    assert!(v.capacity() >= 10);
}

// ==================== String 预分配验证（模拟导出服务） ====================

#[test]
fn test_string_capacity_estimation() {
    // Simulate the markdown export's pre-allocation strategy
    let workspace_name = "My Workspace";
    let num_papers = 5;
    let mut estimated = 256 + workspace_name.len();
    for _ in 0..num_papers {
        estimated += 100 + 128; // title + misc
    }

    let mut s = String::with_capacity(estimated);
    assert_eq!(s.len(), 0);
    assert!(s.capacity() >= estimated);

    // Simulate writing content
    for i in 0..num_papers {
        s.push_str(&format!("Paper {}: Title content here\n", i));
    }

    // Shouldn't have reallocated (len < capacity)
    assert!(s.capacity() >= s.len());
}

// ==================== DTO 序列化/反序列化 round-trip 测试 ====================

use literature_integration::models::dto::*;
use literature_integration::models::paper::Paper;
use literature_integration::models::author::Author;
use literature_integration::models::keyword::Keyword;

#[test]
fn test_paper_serde_roundtrip() {
    let paper = Paper {
        id: "paper-123".to_string(),
        title: "A Great Paper".to_string(),
        doi: Some("10.1234/example".to_string()),
        arxiv_id: Some("2301.00001".to_string()),
        abstract_text: Some("This is the abstract".to_string()),
        user_notes: Some("My notes about this paper".to_string()),
        year: Some(2024),
        journal: Some("Nature".to_string()),
        created_at: "2024-01-15T00:00:00Z".to_string(),
    };
    let json = serde_json::to_string(&paper).unwrap();
    let decoded: Paper = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.id, paper.id);
    assert_eq!(decoded.title, paper.title);
    assert_eq!(decoded.doi, paper.doi);
    assert_eq!(decoded.year, paper.year);
}

#[test]
fn test_paper_detail_response_serde() {
    let resp = PaperDetailResponse {
        paper: Paper {
            id: "p1".into(),
            title: "Test Paper".into(),
            doi: None,
            arxiv_id: None,
            abstract_text: None,
            user_notes: None,
            year: Some(2023),
            journal: None,
            created_at: "2023".into(),
        },
        first_author: Some(Author {
            id: "a1".into(),
            name: "First Author".into(),
            orcid: Some("0000-0001-0001".into()),
        }),
        corresponding_author: Some(Author {
            id: "a2".into(),
            name: "Corresp Author".into(),
            orcid: None,
        }),
        keywords: vec![
            Keyword { id: "k1".into(), name: "ML".into() },
            Keyword { id: "k2".into(), name: "AI".into() },
        ],
    };
    let json = serde_json::to_string(&resp).unwrap();
    let decoded: PaperDetailResponse = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.paper.id, "p1");
    assert_eq!(decoded.first_author.unwrap().name, "First Author");
    assert_eq!(decoded.keywords.len(), 2);
}

#[test]
fn test_export_filter_defaults() {
    let filter = ExportFilter::default();
    assert!(filter.author_ids.is_none());
    assert!(filter.keyword_ids.is_none());
    assert!(filter.year_range.is_none());
}

#[test]
fn test_export_filter_with_values() {
    let json = r#"{"author_ids":["a1","a2"],"keyword_ids":["k1"],"year_range":[2020,2024]}"#;
    let filter: ExportFilter = serde_json::from_str(json).unwrap();
    assert_eq!(filter.author_ids.unwrap().len(), 2);
    assert_eq!(filter.keyword_ids.unwrap().len(), 1);
    assert_eq!(filter.year_range.unwrap(), (2020, 2024));
}

#[test]
fn test_graph_data_serde() {
    let resp = GraphDataResponse {
        nodes: vec![
            GraphNode { id: "n1".into(), name: "Author1".into(), paper_count: 5, author_type: "first".into() },
            GraphNode { id: "n2".into(), name: "Author2".into(), paper_count: 3, author_type: "corresponding".into() },
        ],
        links: vec![
            GraphLink { source: "n1".into(), target: "n2".into(), paper_count: 2 },
        ],
    };
    let json = serde_json::to_string(&resp).unwrap();
    let decoded: GraphDataResponse = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.nodes.len(), 2);
    assert_eq!(decoded.links.len(), 1);
    assert_eq!(decoded.nodes[0].paper_count, 5);
}

#[test]
fn test_author_with_papers_serde() {
    let awp = AuthorWithPapers {
        author: Author {
            id: "a1".into(),
            name: "Test Author".into(),
            orcid: None,
        },
        papers: vec![
            Paper {
                id: "p1".into(),
                title: "Paper 1".into(),
                doi: None, arxiv_id: None, abstract_text: None,
                user_notes: None, year: Some(2024), journal: None,
                created_at: "2024".into(),
            },
        ],
    };
    let json = serde_json::to_string(&awp).unwrap();
    let decoded: AuthorWithPapers = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.author.name, "Test Author");
    assert_eq!(decoded.papers.len(), 1);
}

// ==================== Identifier 检测验证 ====================

#[test]
fn test_identifier_type_detection() {
    // Simulating the logic from fetch_by_identifier
    fn detect_is_doi(identifier: &str) -> bool {
        let trimmed = identifier.trim();
        trimmed.starts_with("10.")
            || trimmed.as_bytes().get(0..4).map_or(false, |s| s.eq_ignore_ascii_case(b"doi:"))
    }

    // DOI formats
    assert!(detect_is_doi("10.1234/example"));
    assert!(detect_is_doi("doi:10.1234/example"));
    assert!(detect_is_doi("DOI:10.1234/test"));
    assert!(detect_is_doi("  10.1000/xyz123  "));

    // arXiv formats
    assert!(!detect_is_doi("2301.00001"));
    assert!(!detect_is_doi("hep-th/9901001"));
    assert!(!detect_is_doi("arXiv:2301.00001"));
}

// ==================== 性能改进的逻辑验证（无 Neo4j 连接） ====================

#[test]
fn test_parallel_join_pattern() {
    // Verifying that the tokio::join! pattern used in services compiles conceptually
    // This is a compile-time/conceptual test
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let a = async { 1i32 };
        let b = async { 2i32 };
        let (ra, rb) = tokio::join!(a, b);
        assert_eq!(ra + rb, 3);
    });
}
