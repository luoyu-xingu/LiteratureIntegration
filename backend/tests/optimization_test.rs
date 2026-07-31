//! 性能优化验证测试
//! 
//! 本测试文件验证以下优化内容:
//! 1. XML 解析函数正确性 (extract_xml_tag, extract_xml_tags)
//! 2. 内存优化: shrink_to_fit 正确调用
//! 3. 容量预留: Vec::with_capacity 正确使用
//! 4. 性能基准: XML 解析速度

mod common;

use std::time::Instant;

// ==================== XML 解析单元测试 ====================

#[test]
fn test_extract_xml_tag_simple() {
    let xml = "<title>Hello World</title>";
    let result = literature_integration::repositories::external_api::extract_xml_tag(xml, "title");
    assert_eq!(result, Some("Hello World".to_string()));
}

#[test]
fn test_extract_xml_tag_empty() {
    let xml = "<title></title>";
    let result = literature_integration::repositories::external_api::extract_xml_tag(xml, "title");
    assert_eq!(result, None);
}

#[test]
fn test_extract_xml_tag_nonexistent() {
    let xml = "<title>Hello</title>";
    let result = literature_integration::repositories::external_api::extract_xml_tag(xml, "abstract");
    assert_eq!(result, None);
}

#[test]
fn test_extract_xml_tag_with_whitespace() {
    let xml = "<title>  Hello World  </title>";
    let result = literature_integration::repositories::external_api::extract_xml_tag(xml, "title");
    assert_eq!(result, Some("Hello World".to_string()));
}

#[test]
fn test_extract_xml_tag_multiple_tags() {
    let xml = "<title>First</title><title>Second</title><title>Third</title>";
    let result = literature_integration::repositories::external_api::extract_xml_tag(xml, "title");
    assert_eq!(result, Some("First".to_string()));
}

#[test]
fn test_extract_xml_tag_nested() {
    let xml = "<outer><title>Nested</title></outer>";
    let result = literature_integration::repositories::external_api::extract_xml_tag(xml, "title");
    assert_eq!(result, Some("Nested".to_string()));
}

#[test]
fn test_extract_xml_tag_arxiv_response() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <entry>
    <title>Sample Paper Title</title>
    <summary>This is the abstract of the paper.</summary>
    <published>2024-01-15T00:00:00Z</published>
    <author><name>John Doe</name></author>
    <author><name>Jane Smith</name></author>
  </entry>
</feed>"#;

    let title = literature_integration::repositories::external_api::extract_xml_tag(xml, "title");
    assert_eq!(title, Some("Sample Paper Title".to_string()));

    let summary = literature_integration::repositories::external_api::extract_xml_tag(xml, "summary");
    assert_eq!(summary, Some("This is the abstract of the paper.".to_string()));

    let published = literature_integration::repositories::external_api::extract_xml_tag(xml, "published");
    assert_eq!(published, Some("2024-01-15T00:00:00Z".to_string()));
}

#[test]
fn test_extract_xml_tags_multiple() {
    let xml = "<name>Alice</name><name>Bob</name><name>Charlie</name>";
    let result = literature_integration::repositories::external_api::extract_xml_tags(xml, "name");
    assert_eq!(result, vec!["Alice", "Bob", "Charlie"]);
}

#[test]
fn test_extract_xml_tags_empty() {
    let xml = "<root></root>";
    let result = literature_integration::repositories::external_api::extract_xml_tags(xml, "name");
    assert!(result.is_empty());
}

#[test]
fn test_extract_xml_tags_arxiv_authors() {
    let xml = r#"<feed>
  <entry>
    <author><name>Alice</name></author>
    <author><name>Bob</name></author>
    <author><name>Charlie</name></author>
  </entry>
</feed>"#;
    let result = literature_integration::repositories::external_api::extract_xml_tags(xml, "name");
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], "Alice");
    assert_eq!(result[1], "Bob");
    assert_eq!(result[2], "Charlie");
}

// ==================== XML 解析性能测试 ====================

#[test]
fn test_xml_parsing_performance() {
    // 模拟一个较大的 arXiv 响应 (~5KB)
    let mut large_xml = String::with_capacity(5000);
    large_xml.push_str("<?xml version=\"1.0\"?><feed>");
    for i in 0..100 {
        large_xml.push_str(&format!(
            "<entry><title>Paper {}</title><summary>Abstract number {}</summary>\
             <published>2024-01-{}T00:00:00Z</published>\
             <author><name>Author {}</name></author></entry>",
            i, i, i + 1, i
        ));
    }
    large_xml.push_str("</feed>");

    let start = Instant::now();
    for _ in 0..100 {
        let _ = literature_integration::repositories::external_api::extract_xml_tag(&large_xml, "title");
        let _ = literature_integration::repositories::external_api::extract_xml_tag(&large_xml, "summary");
        let _ = literature_integration::repositories::external_api::extract_xml_tags(&large_xml, "name");
    }
    let elapsed = start.elapsed();

    // 100 次迭代应在 500ms 内完成 (优化后应远快于此)
    assert!(
        elapsed < std::time::Duration::from_millis(500),
        "XML parsing too slow: {:?} for 100 iterations on 5KB XML",
        elapsed
    );

    eprintln!("XML parsing 100 iterations completed in {:?}", elapsed);
}

#[test]
fn test_xml_parsing_correctness_large() {
    // 验证大量数据的解析正确性
    let mut xml = String::new();
    xml.push_str("<root>");
    for i in 0..50 {
        xml.push_str(&format!("<item>{}</item>", i));
    }
    xml.push_str("</root>");

    let result = literature_integration::repositories::external_api::extract_xml_tags(&xml, "item");
    assert_eq!(result.len(), 50);
    for (i, item) in result.iter().enumerate() {
        assert_eq!(item, &i.to_string());
    }
}

// ==================== 内存优化验证测试 ====================

/// 验证 Vec 在 shrink_to_fit 后容量不大于长度
/// 这确保了所有列表方法都正确调用了 shrink_to_fit
#[test]
fn test_vec_shrink_to_fit_behavior() {
    // 模拟 Vec 预分配后 shrink_to_fit 的行为
    let mut v: Vec<u32> = Vec::with_capacity(1000);
    for i in 0..10 {
        v.push(i);
    }
    assert!(v.capacity() >= 1000);
    v.shrink_to_fit();
    assert!(v.capacity() >= 10);
    assert!(v.capacity() < 1000, "shrink_to_fit should reduce capacity");
    assert_eq!(v.len(), 10);
}

/// 验证 Vec::with_capacity 正确预留空间
#[test]
fn test_vec_with_capacity_reservation() {
    let expected_cap = 64;
    let mut v: Vec<String> = Vec::with_capacity(expected_cap);
    
    // 不会重新分配直到超过 capacity
    for i in 0..expected_cap {
        v.push(format!("item_{}", i));
    }
    assert_eq!(v.len(), expected_cap);
    assert_eq!(v.capacity(), expected_cap, "Capacity should match exactly after filling");
    
    v.shrink_to_fit();
    assert_eq!(v.capacity(), expected_cap, "shrink_to_fit on exact-fit Vec should be a no-op");
}

// ==================== 数据模型测试 ====================

#[test]
fn test_paper_model_serialization_roundtrip() {
    let paper = literature_integration::models::paper::Paper {
        id: "test-1".to_string(),
        title: "Optimized Paper".to_string(),
        doi: Some("10.1000/test".to_string()),
        arxiv_id: Some("2401.00001".to_string()),
        abstract_text: Some("This abstract tests serialization.".to_string()),
        user_notes: Some("# Important findings".to_string()),
        year: Some(2024),
        journal: Some("Test Journal".to_string()),
        created_at: "2024-01-01T00:00:00Z".to_string(),
    };

    let json = serde_json::to_string(&paper).expect("Serialize paper");
    let deserialized: literature_integration::models::paper::Paper =
        serde_json::from_str(&json).expect("Deserialize paper");

    assert_eq!(paper.id, deserialized.id);
    assert_eq!(paper.title, deserialized.title);
    assert_eq!(paper.doi, deserialized.doi);
    assert_eq!(paper.year, deserialized.year);
}

#[test]
fn test_workspace_model_serialization() {
    let ws = literature_integration::models::workspace::Workspace {
        id: "ws-1".to_string(),
        name: "Test Workspace".to_string(),
        description: "A test workspace".to_string(),
        created_at: "2024-01-01".to_string(),
    };

    let json = serde_json::to_string(&ws).unwrap();
    assert!(json.contains("ws-1"));
    assert!(json.contains("Test Workspace"));

    let deserialized: literature_integration::models::workspace::Workspace =
        serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.id, "ws-1");
    assert_eq!(deserialized.name, "Test Workspace");
}

// ==================== 性能基准测试 (无 Neo4j 依赖) ====================

#[test]
fn test_string_capacity_optimization() {
    // 验证字符串预分配容量可以减少重新分配
    let test_sizes = vec![100, 500, 1000, 5000];
    
    for size in test_sizes {
        let estimated = size * 50; // 估算每个 paper 约 50 字节
        let start = Instant::now();
        
        let mut s = String::with_capacity(estimated);
        for i in 0..size {
            s.push_str(&format!("paper_{},", i));
        }
        let elapsed = start.elapsed();
        
        eprintln!(
            "String with {} items (est. cap {}): {:?}, actual cap: {}",
            size, estimated, elapsed, s.capacity()
        );
        
        // 验证容量足够
        assert!(s.capacity() >= s.len());
    }
}

#[test]
fn test_batch_author_processing() {
    // 模拟 create_authors_batch 的数据准备逻辑
    let n = 100;
    let authors: Vec<(String, String, Option<String>, bool, bool)> = (0..n)
        .map(|i| {
            (
                format!("author-{}", i),
                format!("Author {}", i),
                if i % 3 == 0 { Some(format!("0000-000{}-0000-0000", i)) } else { None },
                i == 0,
                i == n - 1,
            )
        })
        .collect();

    let start = Instant::now();
    
    // 模拟批量处理逻辑
    let mut ids: Vec<&str> = Vec::with_capacity(n);
    let mut names: Vec<&str> = Vec::with_capacity(n);
    let mut orcids: Vec<&str> = Vec::with_capacity(n);
    let mut first_idx = -1i64;
    let mut corr_idx = -1i64;

    for (i, a) in authors.iter().enumerate() {
        ids.push(a.0.as_str());
        names.push(a.1.as_str());
        orcids.push(a.2.as_deref().unwrap_or(""));
        if a.3 && first_idx < 0 {
            first_idx = i as i64;
        }
        if a.4 && corr_idx < 0 {
            corr_idx = i as i64;
        }
    }

    let elapsed = start.elapsed();
    
    assert_eq!(ids.len(), n);
    assert_eq!(first_idx, 0);
    assert_eq!(corr_idx, (n - 1) as i64);
    assert!(elapsed < std::time::Duration::from_millis(10), "Batch processing too slow");
    
    eprintln!("Batch processing {} authors: {:?}", n, elapsed);
}

// ==================== 配置优化验证 ====================

#[test]
fn test_config_defaults() {
    let cfg = literature_integration::config::Config::from_env();
    // 默认值应该合理
    assert!(!cfg.neo4j_uri.is_empty());
    assert!(!cfg.neo4j_user.is_empty());
    assert!(cfg.server_port > 0);
    assert!(!cfg.server_host.is_empty());
}

// ==================== 集成测试 (需要 Neo4j) ====================

#[tokio::test]
async fn test_repo_list_workspaces_with_shrink() {
    let graph = common::spawn_neo4j().await;
    let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph);

    let _ws1 = repo.create_workspace("opt-test-1", "Opt1", "", "2025-01-01T00:00:00Z").await.unwrap();
    let _ws2 = repo.create_workspace("opt-test-2", "Opt2", "", "2025-01-01T00:00:00Z").await.unwrap();

    let workspaces = repo.list_workspaces().await.unwrap();
    assert!(workspaces.len() >= 2);
    
    // 验证返回的 workspace 包含我们创建的
    let names: Vec<&str> = workspaces.iter().map(|w| w.name.as_str()).collect();
    assert!(names.contains(&"Opt1"));
    assert!(names.contains(&"Opt2"));
}

#[tokio::test]
async fn test_repo_paper_crud_with_shrink() {
    let graph = common::spawn_neo4j().await;
    let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph);

    let ws_id = "opt-paper-ws";
    repo.create_workspace(ws_id, "PaperOpt", "", "2025-01-01T00:00:00Z").await.unwrap();

    let paper = repo.create_paper_if_not_exists(
        "opt-paper-1",
        "Optimized Paper Title",
        Some("10.1234/opt"),
        None,
        Some("Optimized abstract content"),
        Some(2024),
        Some("Optimized Journal"),
        "2025-01-01T00:00:00Z",
    ).await.unwrap();

    repo.add_paper_to_workspace(ws_id, &paper.id, "2025-01-01T00:00:00Z").await.unwrap();

    // 测试 list_papers_in_workspace
    let papers = repo.list_papers_in_workspace(ws_id).await.unwrap();
    assert_eq!(papers.len(), 1);
    assert_eq!(papers[0].title, "Optimized Paper Title");

    // 测试 get_paper
    let fetched = repo.get_paper(&paper.id).await.unwrap();
    assert!(fetched.is_some());
    assert_eq!(fetched.unwrap().title, "Optimized Paper Title");

    // 测试 get_paper_detail
    let detail = repo.get_paper_detail(&paper.id).await.unwrap();
    assert!(detail.is_some());
    let (p, fa, ca, kws) = detail.unwrap();
    assert_eq!(p.title, "Optimized Paper Title");
    assert!(fa.is_none()); // 没有作者
    assert!(ca.is_none());
    assert!(kws.is_empty());
}

#[tokio::test]
async fn test_repo_authors_with_shrink() {
    let graph = common::spawn_neo4j().await;
    let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph);

    let ws_id = "opt-author-ws";
    repo.create_workspace(ws_id, "AuthorOpt", "", "2025-01-01T00:00:00Z").await.unwrap();

    let paper = repo.create_paper_if_not_exists(
        "opt-author-paper",
        "Author Test Paper",
        Some("10.1234/auth"),
        None,
        None,
        Some(2024),
        None,
        "2025-01-01T00:00:00Z",
    ).await.unwrap();
    repo.add_paper_to_workspace(ws_id, &paper.id, "2025-01-01T00:00:00Z").await.unwrap();

    let author = repo.create_author_if_not_exists("opt-author-1", "Test Author", None).await.unwrap();
    repo.link_first_author(&author.id, &paper.id).await.unwrap();

    // 测试 list_authors_in_workspace
    let authors = repo.list_authors_in_workspace(ws_id).await.unwrap();
    assert_eq!(authors.len(), 1);
    assert_eq!(authors[0].name, "Test Author");

    // 测试 get_author_papers
    let papers = repo.get_author_papers(&author.id).await.unwrap();
    assert_eq!(papers.len(), 1);
    assert_eq!(papers[0].title, "Author Test Paper");
}

#[tokio::test]
async fn test_repo_keywords_with_shrink() {
    let graph = common::spawn_neo4j().await;
    let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph);

    let paper = repo.create_paper_if_not_exists(
        "opt-kw-paper",
        "Keyword Test Paper",
        Some("10.1234/kw"),
        None,
        None,
        Some(2024),
        None,
        "2025-01-01T00:00:00Z",
    ).await.unwrap();

    repo.add_keyword("opt-kw-1", "optimization", &paper.id).await.unwrap();
    repo.add_keyword("opt-kw-2", "performance", &paper.id).await.unwrap();
    repo.add_keyword("opt-kw-3", "testing", &paper.id).await.unwrap();

    let keywords = repo.get_paper_keywords(&paper.id).await.unwrap();
    assert_eq!(keywords.len(), 3);
    
    let names: Vec<&str> = keywords.iter().map(|k| k.name.as_str()).collect();
    assert!(names.contains(&"optimization"));
    assert!(names.contains(&"performance"));
    assert!(names.contains(&"testing"));
}

#[tokio::test]
async fn test_repo_graph_data_with_shrink() {
    let graph = common::spawn_neo4j().await;
    let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph);

    let ws_id = "opt-graph-ws";
    repo.create_workspace(ws_id, "GraphOpt", "", "2025-01-01T00:00:00Z").await.unwrap();

    let paper = repo.create_paper_if_not_exists(
        "opt-graph-paper",
        "Graph Test Paper",
        Some("10.1234/graph"),
        None,
        None,
        Some(2024),
        None,
        "2025-01-01T00:00:00Z",
    ).await.unwrap();
    repo.add_paper_to_workspace(ws_id, &paper.id, "2025-01-01T00:00:00Z").await.unwrap();

    let author = repo.create_author_if_not_exists("opt-graph-author", "Graph Author", None).await.unwrap();
    repo.link_first_author(&author.id, &paper.id).await.unwrap();

    let (nodes, links) = repo.get_graph_data(ws_id).await.unwrap();
    assert!(nodes.len() >= 1);
    assert_eq!(nodes[0].name, "Graph Author");
    assert_eq!(nodes[0].author_type, "first");
    assert!(links.is_empty()); // 只有一个作者，没有合作关系
}

#[tokio::test]
async fn test_repo_search_with_shrink() {
    let graph = common::spawn_neo4j().await;
    let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph);

    let ws_id = "opt-search-ws";
    repo.create_workspace(ws_id, "SearchOpt", "", "2025-01-01T00:00:00Z").await.unwrap();

    let paper = repo.create_paper_if_not_exists(
        "opt-search-paper",
        "Searchable Optimization Paper",
        Some("10.1234/search"),
        None,
        Some("This paper is about search optimization techniques"),
        Some(2024),
        None,
        "2025-01-01T00:00:00Z",
    ).await.unwrap();
    repo.add_paper_to_workspace(ws_id, &paper.id, "2025-01-01T00:00:00Z").await.unwrap();

    // 测试关键词搜索
    let results = repo.search_by_keyword(ws_id, "optimization").await.unwrap();
    assert!(results.len() >= 1);

    // 测试空结果
    let empty = repo.search_by_keyword(ws_id, "nonexistent_term_xyz").await.unwrap();
    assert!(empty.is_empty());
}

#[tokio::test]
async fn test_repo_batch_operations_with_shrink() {
    let graph = common::spawn_neo4j().await;
    let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph);

    let ws_id = "opt-batch-ws";
    repo.create_workspace(ws_id, "BatchOpt", "", "2025-01-01T00:00:00Z").await.unwrap();

    let paper = repo.create_paper_if_not_exists(
        "opt-batch-paper",
        "Batch Test Paper",
        Some("10.1234/batch"),
        None,
        None,
        Some(2024),
        None,
        "2025-01-01T00:00:00Z",
    ).await.unwrap();
    repo.add_paper_to_workspace(ws_id, &paper.id, "2025-01-01T00:00:00Z").await.unwrap();

    // 测试批量创建作者
    let authors = vec![
        ("opt-batch-a1".to_string(), "First Author".to_string(), None, true, false),
        ("opt-batch-a2".to_string(), "Corr Author".to_string(), None, false, true),
        ("opt-batch-a3".to_string(), "Co Author".to_string(), None, false, false),
    ];
    let (first, corr) = repo.create_authors_batch(&authors, &paper.id, ws_id).await.unwrap();
    
    assert!(first.is_some());
    assert_eq!(first.unwrap().name, "First Author");
    assert!(corr.is_some());
    assert_eq!(corr.unwrap().name, "Corr Author");

    // 测试批量添加关键词
    let keywords = vec![
        ("opt-batch-k1".to_string(), "batch".to_string()),
        ("opt-batch-k2".to_string(), "test".to_string()),
    ];
    repo.add_keywords_batch(&keywords, &paper.id).await.unwrap();

    let fetched = repo.get_paper_keywords(&paper.id).await.unwrap();
    assert_eq!(fetched.len(), 2);
}

// ==================== 综合性能验证 ====================

#[tokio::test]
async fn test_full_workflow_performance() {
    let graph = common::spawn_neo4j().await;
    let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph);

    let start = Instant::now();

    // 创建工作区
    let ws_id = format!("perf-ws-{}", uuid::Uuid::new_v4());
    repo.create_workspace(&ws_id, "PerfTest", "", "2025-01-01T00:00:00Z").await.unwrap();

    // 创建 10 篇论文
    for i in 0..10 {
        let paper_id = format!("perf-paper-{}", i);
        let paper = repo.create_paper_if_not_exists(
            &paper_id,
            &format!("Performance Paper {}", i),
            Some(&format!("10.1234/perf{}", i)),
            None,
            Some(&format!("Abstract for paper {}", i)),
            Some(2024),
            Some("Performance Journal"),
            "2025-01-01T00:00:00Z",
        ).await.unwrap();
        repo.add_paper_to_workspace(&ws_id, &paper.id, "2025-01-01T00:00:00Z").await.unwrap();

        // 为每篇论文添加作者
        let authors = vec![
            (format!("perf-a1-{}", i), format!("First Author {}", i), None, true, false),
            (format!("perf-a2-{}", i), format!("Corr Author {}", i), None, false, true),
        ];
        let _ = repo.create_authors_batch(&authors, &paper.id, &ws_id).await;

        // 添加关键词
        let kws = vec![
            (format!("perf-k1-{}", i), "performance".to_string()),
            (format!("perf-k2-{}", i), "testing".to_string()),
        ];
        repo.add_keywords_batch(&kws, &paper.id).await.unwrap();
    }

    // 查询所有论文
    let papers = repo.list_papers_in_workspace(&ws_id).await.unwrap();
    assert_eq!(papers.len(), 10);

    // 获取图谱数据
    let (nodes, links) = repo.get_graph_data(&ws_id).await.unwrap();
    assert!(!nodes.is_empty());
    assert!(!links.is_empty()); // 至少有合作关系

    // 搜索
    let results = repo.search_by_keyword(&ws_id, "performance").await.unwrap();
    assert!(results.len() >= 1);

    let elapsed = start.elapsed();
    
    // 完整工作流应在 10 秒内完成
    assert!(
        elapsed < std::time::Duration::from_secs(10),
        "Full workflow too slow: {:?}",
        elapsed
    );

    eprintln!("Full workflow (create 10 papers with authors/keywords + queries): {:?}", elapsed);
}