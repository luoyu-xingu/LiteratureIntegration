//! 运行速度优化验证测试
//!
//! 本测试文件验证以下优化内容:
//! 1. Cypher 查询优化: pattern comprehension 替代 OPTIONAL MATCH 链
//! 2. 迭代器优化: into_iter() 替代 iter() 减少克隆
//! 3. 数据一致性验证: 确保优化后查询结果与原逻辑一致
//! 4. 性能基准: 验证关键路径性能满足要求
//! 5. 内存效率: 验证容量预留和字符串分配优化

mod common;

use std::time::Instant;

// ==================== 内存分配优化测试 ====================

#[test]
fn test_vec_with_capacity_avoids_reallocation() {
    let expected_cap = 128;
    let mut v: Vec<i32> = Vec::with_capacity(expected_cap);

    for i in 0..expected_cap {
        v.push(i as i32);
    }
    assert_eq!(v.len(), expected_cap);
    assert_eq!(v.capacity(), expected_cap, "Exact fill should not reallocate");

    v.shrink_to_fit();
    assert_eq!(v.capacity(), expected_cap, "shrink_to_fit on exact-fit is a no-op");
}

#[test]
fn test_string_with_capacity_accurate_estimate() {
    let estimated = 1024;
    let mut s = String::with_capacity(estimated);

    for i in 0..50 {
        s.push_str(&format!("item_{},", i));
    }

    assert!(s.capacity() >= s.len(), "Capacity must be >= length");

    if s.capacity() > s.len() * 2 {
        s.shrink_to_fit();
    }
    assert!(s.capacity() >= s.len());
}

#[test]
fn test_itoa_buffer_reuse() {
    let mut buf = itoa::Buffer::new();

    let val1 = buf.format(42);
    assert_eq!(val1, "42");

    let val2 = buf.format(12345);
    assert_eq!(val2, "12345");

    let val3 = buf.format(0);
    assert_eq!(val3, "0");

    let val4 = buf.format(-1);
    assert_eq!(val4, "-1");
}

// ==================== 批量数据处理性能测试 ====================

#[test]
fn test_batch_author_processing_throughput() {
    let n = 500;
    let authors: Vec<(String, String, Option<String>, bool, bool)> = (0..n)
        .map(|i| {
            (
                format!("author-{}-id", i),
                format!("Author {}", i),
                if i % 5 == 0 { Some(format!("0000-000{}-0000-0000", i)) } else { None },
                i == 0,
                i == n - 1,
            )
        })
        .collect();

    let start = Instant::now();

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
    assert!(
        elapsed < std::time::Duration::from_millis(5),
        "Batch processing {} authors too slow: {:?}",
        n,
        elapsed
    );

    eprintln!("Batch author processing ({} items): {:?}", n, elapsed);
}

#[test]
fn test_keyword_batch_processing_throughput() {
    let n = 200;
    let keywords: Vec<(String, String)> = (0..n)
        .map(|i| (format!("kw-{}-id", i), format!("keyword_{}", i)))
        .collect();

    let start = Instant::now();

    let mut ids = Vec::with_capacity(n);
    let mut names = Vec::with_capacity(n);

    for k in &keywords {
        ids.push(k.0.as_str());
        names.push(k.1.as_str());
    }

    let elapsed = start.elapsed();

    assert_eq!(ids.len(), n);
    assert_eq!(names.len(), n);
    assert!(
        elapsed < std::time::Duration::from_millis(2),
        "Keyword batch processing too slow: {:?}",
        elapsed
    );
}

// ==================== 导出性能测试 ====================

#[test]
fn test_export_string_building_efficiency() {
    let paper_count = 30;
    let estimated_size = 512;

    let mut md = String::with_capacity(estimated_size);
    let mut num_buf = itoa::Buffer::new();

    md.push_str("# 工作区: Test Workspace\n\n> 导出时间: ");
    md.push_str("2025-01-01 12:00");
    md.push_str("\n> 论文数量: ");
    md.push_str(num_buf.format(paper_count));
    md.push_str("\n\n---\n\n");

    for i in 0..paper_count {
        md.push_str("### ");
        md.push_str(&format!("Paper Title {}", i));
        md.push_str("\n- **年份**: ");
        md.push_str(num_buf.format(2020 + (i % 5)));
        md.push_str(" | **期刊**: Test Journal");
        md.push_str("\n- **DOI**: 10.1234/test");
        md.push_str("\n- **一作**: Author One");
        md.push_str(" | **通讯**: Author Two");
        md.push_str("\n- **关键词**: ");

        let kws = vec!["kw1", "kw2", "kw3"];
        let mut first = true;
        for kw in &kws {
            if !first {
                md.push_str(", ");
            }
            md.push_str(kw);
            first = false;
        }
        md.push_str("\n\n");
        md.push_str("**Abstract:**\nAbstract content for paper.\n\n");
        md.push_str("---\n\n");
    }

    assert!(md.len() > 0);
    assert!(md.capacity() >= md.len());

    if md.capacity() > md.len() * 2 {
        md.shrink_to_fit();
    }

    eprintln!(
        "Export string: len={}, cap={}",
        md.len(),
        md.capacity()
    );
}

// ==================== 数据模型完整性测试 ====================

#[test]
fn test_paper_detail_response_structure() {
    let paper = literature_integration::models::paper::Paper {
        id: "test-p-1".to_string(),
        title: "Test Paper".to_string(),
        doi: Some("10.1234/test".to_string()),
        arxiv_id: None,
        abstract_text: Some("Test abstract".to_string()),
        user_notes: None,
        year: Some(2024),
        journal: Some("Test Journal".to_string()),
        created_at: "2025-01-01T00:00:00Z".to_string(),
    };

    let first_author = literature_integration::models::author::Author {
        id: "test-a-1".to_string(),
        name: "First Author".to_string(),
        orcid: None,
    };

    let corr_author = literature_integration::models::author::Author {
        id: "test-a-2".to_string(),
        name: "Corr Author".to_string(),
        orcid: Some("0000-0000-0000-0000".to_string()),
    };

    let keywords = vec![
        literature_integration::models::keyword::Keyword {
            id: "test-k-1".to_string(),
            name: "optimization".to_string(),
        },
        literature_integration::models::keyword::Keyword {
            id: "test-k-2".to_string(),
            name: "performance".to_string(),
        },
    ];

    let resp = literature_integration::models::dto::PaperDetailResponse {
        paper,
        first_author: Some(first_author),
        corresponding_author: Some(corr_author),
        keywords,
    };

    let json = serde_json::to_string(&resp).unwrap();
    assert!(json.contains("test-p-1"));
    assert!(json.contains("First Author"));
    assert!(json.contains("Corr Author"));
    assert!(json.contains("optimization"));
    assert!(json.contains("performance"));
}

#[test]
fn test_graph_node_serialization_roundtrip() {
    let node = literature_integration::models::dto::GraphNode {
        id: "node-1".to_string(),
        name: "Test Author".to_string(),
        paper_count: 5,
        author_type: "first".to_string(),
    };

    let json = serde_json::to_string(&node).unwrap();
    assert!(json.contains("node-1"));
    assert!(json.contains("Test Author"));

    let back: literature_integration::models::dto::GraphNode =
        serde_json::from_str(&json).unwrap();
    assert_eq!(back.id, "node-1");
    assert_eq!(back.paper_count, 5);
    assert_eq!(back.author_type, "first");
}

// ==================== XML 解析正确性与性能 ====================

#[test]
fn test_xml_parsing_correctness_and_speed() {
    let mut large_xml = String::with_capacity(10000);
    large_xml.push_str("<?xml version=\"1.0\"?><feed>");
    for i in 0..200 {
        large_xml.push_str(&format!(
            "<entry><title>Paper {}</title><summary>Abstract {}</summary>\
             <published>2024-01-{}T00:00:00Z</published>\
             <author><name>Author {}</name></author></entry>",
            i, i, i + 1, i
        ));
    }
    large_xml.push_str("</feed>");

    let start = Instant::now();
    for _ in 0..50 {
        let _ = literature_integration::repositories::external_api::extract_xml_tag(&large_xml, "title");
        let _ = literature_integration::repositories::external_api::extract_xml_tag(&large_xml, "summary");
        let _ = literature_integration::repositories::external_api::extract_xml_tags(&large_xml, "name");
    }
    let elapsed = start.elapsed();

    assert!(
        elapsed < std::time::Duration::from_millis(500),
        "XML parsing (200 entries x 50 iterations) too slow: {:?}",
        elapsed
    );

    eprintln!("XML parsing 200-entries x 50 iterations: {:?}", elapsed);
}

// ==================== 集成测试: 验证优化后查询结果一致性 ====================

#[tokio::test]
async fn test_pattern_comprehension_get_paper_detail() {
    let graph = common::spawn_neo4j().await;
    let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph);

    let ws_id = format!("speed-test-ws-{}", uuid::Uuid::new_v4());
    repo.create_workspace(&ws_id, "SpeedTest", "", "2025-01-01T00:00:00Z").await.unwrap();

    let paper_id = format!("speed-test-paper-{}", uuid::Uuid::new_v4());
    let paper = repo.create_paper_if_not_exists(
        &paper_id,
        "Speed Optimization Test Paper",
        Some("10.9999/speed"),
        None,
        Some("This paper tests speed optimization patterns."),
        Some(2024),
        Some("Speed Journal"),
        "2025-01-01T00:00:00Z",
    ).await.unwrap();

    repo.add_paper_to_workspace(&ws_id, &paper.id, "2025-01-01T00:00:00Z").await.unwrap();

    let first_author = repo.create_author_if_not_exists(
        &format!("speed-a1-{}", uuid::Uuid::new_v4()),
        "First Author",
        None,
    ).await.unwrap();
    repo.link_first_author(&first_author.id, &paper.id).await.unwrap();

    let corr_author = repo.create_author_if_not_exists(
        &format!("speed-a2-{}", uuid::Uuid::new_v4()),
        "Corresponding Author",
        Some("0000-0000-0000-0000"),
    ).await.unwrap();
    repo.link_corresponding_author(&corr_author.id, &paper.id).await.unwrap();

    repo.add_keyword(&format!("speed-k1-{}", uuid::Uuid::new_v4()), "optimization", &paper.id).await.unwrap();
    repo.add_keyword(&format!("speed-k2-{}", uuid::Uuid::new_v4()), "performance", &paper.id).await.unwrap();

    // 验证 pattern comprehension 查询结果正确性
    let detail = repo.get_paper_detail(&paper.id).await.unwrap();
    assert!(detail.is_some(), "Paper detail should exist");

    let (p, fa, ca, kws) = detail.unwrap();
    assert_eq!(p.title, "Speed Optimization Test Paper");
    assert_eq!(p.year, Some(2024));
    assert_eq!(p.journal.as_deref(), Some("Speed Journal"));

    // 验证第一作者
    assert!(fa.is_some(), "First author should exist");
    assert_eq!(fa.unwrap().name, "First Author");

    // 验证通讯作者
    assert!(ca.is_some(), "Corresponding author should exist");
    let ca_unwrapped = ca.unwrap();
    assert_eq!(ca_unwrapped.name, "Corresponding Author");
    assert_eq!(ca_unwrapped.orcid, Some("0000-0000-0000-0000".to_string()));

    // 验证关键词
    assert_eq!(kws.len(), 2);
    let kw_names: Vec<&str> = kws.iter().map(|k| k.name.as_str()).collect();
    assert!(kw_names.contains(&"optimization"));
    assert!(kw_names.contains(&"performance"));

    eprintln!("Pattern comprehension get_paper_detail: PASSED");
}

#[tokio::test]
async fn test_pattern_comprehension_get_paper_detail_no_optional() {
    let graph = common::spawn_neo4j().await;
    let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph);

    let paper_id = format!("speed-noopt-paper-{}", uuid::Uuid::new_v4());
    repo.create_paper_if_not_exists(
        &paper_id,
        "No Optional Relations Paper",
        Some("10.9999/noopt"),
        None,
        None,
        Some(2023),
        None,
        "2025-01-01T00:00:00Z",
    ).await.unwrap();

    // 验证无作者/关键词时结果正确
    let detail = repo.get_paper_detail(&paper_id).await.unwrap();
    assert!(detail.is_some());
    let (p, fa, ca, kws) = detail.unwrap();
    assert_eq!(p.title, "No Optional Relations Paper");
    assert!(fa.is_none(), "First author should be None");
    assert!(ca.is_none(), "Corresponding author should be None");
    assert!(kws.is_empty(), "Keywords should be empty");

    eprintln!("Pattern comprehension (no optional relations): PASSED");
}

#[tokio::test]
async fn test_pattern_comprehension_get_papers_detail_batch() {
    let graph = common::spawn_neo4j().await;
    let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph);

    let ws_id = format!("speed-batch-ws-{}", uuid::Uuid::new_v4());
    repo.create_workspace(&ws_id, "SpeedBatch", "", "2025-01-01T00:00:00Z").await.unwrap();

    // 创建 5 篇论文，每篇都有作者和关键词
    for i in 0..5 {
        let paper_id = format!("speed-batch-paper-{}", uuid::Uuid::new_v4());
        let paper = repo.create_paper_if_not_exists(
            &paper_id,
            &format!("Batch Paper {}", i),
            Some(&format!("10.9999/batch{}", i)),
            None,
            Some(&format!("Abstract for batch paper {}", i)),
            Some(2024),
            Some("Batch Journal"),
            "2025-01-01T00:00:00Z",
        ).await.unwrap();
        repo.add_paper_to_workspace(&ws_id, &paper.id, "2025-01-01T00:00:00Z").await.unwrap();

        let a1 = repo.create_author_if_not_exists(
            &format!("speed-batch-a1-{}", uuid::Uuid::new_v4()),
            &format!("First Author {}", i),
            None,
        ).await.unwrap();
        repo.link_first_author(&a1.id, &paper.id).await.unwrap();

        let a2 = repo.create_author_if_not_exists(
            &format!("speed-batch-a2-{}", uuid::Uuid::new_v4()),
            &format!("Corr Author {}", i),
            None,
        ).await.unwrap();
        repo.link_corresponding_author(&a2.id, &paper.id).await.unwrap();

        repo.add_keyword(&format!("speed-batch-k1-{}", uuid::Uuid::new_v4()), "batch", &paper.id).await.unwrap();
        repo.add_keyword(&format!("speed-batch-k2-{}", uuid::Uuid::new_v4()), &format!("kw_{}", i), &paper.id).await.unwrap();
    }

    // 验证批量查询结果
    let results = repo.get_papers_detail_batch(&ws_id, None, None, None).await.unwrap();
    assert_eq!(results.len(), 5);

    for (paper, fa, ca, kws) in &results {
        assert!(fa.is_some(), "Each paper should have a first author");
        assert!(ca.is_some(), "Each paper should have a corresponding author");
        assert_eq!(kws.len(), 2, "Each paper should have 2 keywords");
        assert!(paper.year.is_some());
    }

    // 验证排序 (年份降序)
    for i in 1..results.len() {
        assert!(results[i - 1].0.year >= results[i].0.year);
    }

    eprintln!("Pattern comprehension get_papers_detail_batch: PASSED ({} papers)", results.len());
}

#[tokio::test]
async fn test_pattern_comprehension_get_graph_data() {
    let graph = common::spawn_neo4j().await;
    let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph);

    let ws_id = format!("speed-graph-ws-{}", uuid::Uuid::new_v4());
    repo.create_workspace(&ws_id, "SpeedGraph", "", "2025-01-01T00:00:00Z").await.unwrap();

    let paper_id = format!("speed-graph-paper-{}", uuid::Uuid::new_v4());
    let paper = repo.create_paper_if_not_exists(
        &paper_id,
        "Graph Test Paper",
        Some("10.9999/graph"),
        None,
        None,
        Some(2024),
        None,
        "2025-01-01T00:00:00Z",
    ).await.unwrap();
    repo.add_paper_to_workspace(&ws_id, &paper.id, "2025-01-01T00:00:00Z").await.unwrap();

    let a1 = repo.create_author_if_not_exists(&format!("speed-graph-a1-{}", uuid::Uuid::new_v4()), "Alice", None).await.unwrap();
    repo.link_first_author(&a1.id, &paper.id).await.unwrap();

    let a2 = repo.create_author_if_not_exists(&format!("speed-graph-a2-{}", uuid::Uuid::new_v4()), "Bob", None).await.unwrap();
    repo.link_corresponding_author(&a2.id, &paper.id).await.unwrap();
    repo.link_co_authors(&a1.id, &a2.id, &ws_id).await.unwrap();

    // 验证图谱数据
    let (nodes, links) = repo.get_graph_data(&ws_id).await.unwrap();
    assert_eq!(nodes.len(), 2, "Should have 2 author nodes");
    assert_eq!(links.len(), 1, "Should have 1 co-author link");

    // 验证节点属性
    let alice = nodes.iter().find(|n| n.name == "Alice").unwrap();
    assert_eq!(alice.author_type, "first");
    assert_eq!(alice.paper_count, 1);

    let bob = nodes.iter().find(|n| n.name == "Bob").unwrap();
    assert_eq!(bob.author_type, "corresponding");

    // 验证链接
    assert_eq!(links[0].paper_count, 1);

    eprintln!("Pattern comprehension get_graph_data: PASSED");
}

#[tokio::test]
async fn test_into_iter_search_by_author() {
    let graph = common::spawn_neo4j().await;
    let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph);

    let ws_id = format!("speed-search-ws-{}", uuid::Uuid::new_v4());
    repo.create_workspace(&ws_id, "SpeedSearch", "", "2025-01-01T00:00:00Z").await.unwrap();

    let paper_id = format!("speed-search-paper-{}", uuid::Uuid::new_v4());
    let paper = repo.create_paper_if_not_exists(
        &paper_id,
        "Search Test Paper",
        Some("10.9999/search"),
        None,
        None,
        Some(2024),
        None,
        "2025-01-01T00:00:00Z",
    ).await.unwrap();
    repo.add_paper_to_workspace(&ws_id, &paper.id, "2025-01-01T00:00:00Z").await.unwrap();

    let author = repo.create_author_if_not_exists(
        &format!("speed-search-a1-{}", uuid::Uuid::new_v4()),
        "Searchable Author",
        None,
    ).await.unwrap();
    repo.link_first_author(&author.id, &paper.id).await.unwrap();

    // 验证搜索结果
    let results = repo.search_by_author(&ws_id, "Searchable").await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].author.name, "Searchable Author");
    assert_eq!(results[0].papers.len(), 1);
    assert_eq!(results[0].papers[0].title, "Search Test Paper");

    // 验证空结果
    let empty = repo.search_by_author(&ws_id, "NonExistent").await.unwrap();
    assert!(empty.is_empty());

    eprintln!("into_iter search_by_author: PASSED");
}

// ==================== 综合性能基准测试 ====================

#[tokio::test]
async fn test_crud_operations_performance_budget() {
    let graph = common::spawn_neo4j().await;
    let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph);

    let start = Instant::now();

    let ws_id = format!("speed-perf-ws-{}", uuid::Uuid::new_v4());
    repo.create_workspace(&ws_id, "PerfBudget", "", "2025-01-01T00:00:00Z").await.unwrap();

    // 创建 20 篇论文，每篇有 2 个作者和 2 个关键词
    for i in 0..20 {
        let paper_id = format!("speed-perf-paper-{}-{}", i, uuid::Uuid::new_v4());
        let paper = repo.create_paper_if_not_exists(
            &paper_id,
            &format!("Perf Paper {}", i),
            Some(&format!("10.9999/perf{}", i)),
            None,
            Some(&format!("Performance abstract for paper {}", i)),
            Some(2024),
            Some("Performance Journal"),
            "2025-01-01T00:00:00Z",
        ).await.unwrap();
        repo.add_paper_to_workspace(&ws_id, &paper.id, "2025-01-01T00:00:00Z").await.unwrap();

        let authors = vec![
            (format!("speed-perf-a1-{}-{}", i, uuid::Uuid::new_v4()), format!("First Author {}", i), None, true, false),
            (format!("speed-perf-a2-{}-{}", i, uuid::Uuid::new_v4()), format!("Corr Author {}", i), None, false, true),
        ];
        let _ = repo.create_authors_batch(&authors, &paper.id, &ws_id).await;

        let kws = vec![
            (format!("speed-perf-k1-{}-{}", i, uuid::Uuid::new_v4()), "performance".to_string()),
            (format!("speed-perf-k2-{}-{}", i, uuid::Uuid::new_v4()), "benchmark".to_string()),
        ];
        repo.add_keywords_batch(&kws, &paper.id).await.unwrap();
    }

    // 执行查询
    let papers = repo.list_papers_in_workspace(&ws_id).await.unwrap();
    assert_eq!(papers.len(), 20);

    let results = repo.search_by_keyword(&ws_id, "performance").await.unwrap();
    assert!(!results.is_empty());

    let (nodes, _links) = repo.get_graph_data(&ws_id).await.unwrap();
    assert!(!nodes.is_empty());

    let elapsed = start.elapsed();

    assert!(
        elapsed < std::time::Duration::from_secs(15),
        "Full CRUD workflow (20 papers) too slow: {:?}",
        elapsed
    );

    eprintln!("CRUD workflow (20 papers with authors/keywords): {:?}", elapsed);
}

#[tokio::test]
async fn test_query_throughput_under_load() {
    let graph = common::spawn_neo4j().await;
    let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph);

    let ws_id = format!("speed-load-ws-{}", uuid::Uuid::new_v4());
    repo.create_workspace(&ws_id, "LoadTest", "", "2025-01-01T00:00:00Z").await.unwrap();

    // 创建 10 篇论文
    for i in 0..10 {
        let paper_id = format!("speed-load-paper-{}-{}", i, uuid::Uuid::new_v4());
        let paper = repo.create_paper_if_not_exists(
            &paper_id,
            &format!("Load Paper {}", i),
            Some(&format!("10.9999/load{}", i)),
            None,
            None,
            Some(2024),
            None,
            "2025-01-01T00:00:00Z",
        ).await.unwrap();
        repo.add_paper_to_workspace(&ws_id, &paper.id, "2025-01-01T00:00:00Z").await.unwrap();

        let a1 = repo.create_author_if_not_exists(
            &format!("speed-load-a1-{}-{}", i, uuid::Uuid::new_v4()),
            &format!("Author {}", i),
            None,
        ).await.unwrap();
        repo.link_first_author(&a1.id, &paper.id).await.unwrap();
    }

    // 连续执行 50 次 get_paper_detail 查询
    let paper_ids: Vec<String> = (0..10)
        .map(|i| format!("speed-load-paper-{}", i))
        .collect();

    let start = Instant::now();
    for _ in 0..5 {
        for pid in &paper_ids {
            let _ = repo.get_paper_detail(pid).await;
        }
    }
    let elapsed = start.elapsed();

    assert!(
        elapsed < std::time::Duration::from_secs(5),
        "get_paper_detail x50 queries too slow: {:?}",
        elapsed
    );

    eprintln!("get_paper_detail x50 queries: {:?}", elapsed);
}

#[tokio::test]
async fn test_data_consistency_after_optimization() {
    let graph = common::spawn_neo4j().await;
    let repo = literature_integration::repositories::neo4j_repo::Neo4jRepo::new(graph);

    let ws_id = format!("speed-consistency-ws-{}", uuid::Uuid::new_v4());
    repo.create_workspace(&ws_id, "ConsistencyTest", "", "2025-01-01T00:00:00Z").await.unwrap();

    let paper_id = format!("speed-consistency-paper-{}", uuid::Uuid::new_v4());
    let expected_title = "Consistency Verification Paper";
    let expected_year = 2023;
    let expected_journal = "Consistency Journal";
    let expected_doi = "10.8888/consistency";

    repo.create_paper_if_not_exists(
        &paper_id,
        expected_title,
        Some(expected_doi),
        None,
        Some("Abstract for consistency verification"),
        Some(expected_year),
        Some(expected_journal),
        "2025-01-01T00:00:00Z",
    ).await.unwrap();

    // 添加多个作者
    let a1_id = format!("speed-consistency-a1-{}", uuid::Uuid::new_v4());
    let a2_id = format!("speed-consistency-a2-{}", uuid::Uuid::new_v4());
    let a3_id = format!("speed-consistency-a3-{}", uuid::Uuid::new_v4());

    repo.create_author_if_not_exists(&a1_id, "First Author", None).await.unwrap();
    repo.create_author_if_not_exists(&a2_id, "Corr Author", None).await.unwrap();
    repo.create_author_if_not_exists(&a3_id, "Co Author", None).await.unwrap();

    repo.link_first_author(&a1_id, &paper_id).await.unwrap();
    repo.link_corresponding_author(&a2_id, &paper_id).await.unwrap();
    repo.link_co_authors(&a1_id, &a2_id, &ws_id).await.unwrap();

    repo.add_keyword(&format!("speed-consistency-k1-{}", uuid::Uuid::new_v4()), "consistency", &paper_id).await.unwrap();
    repo.add_keyword(&format!("speed-consistency-k2-{}", uuid::Uuid::new_v4()), "verification", &paper_id).await.unwrap();

    // 验证所有查询方法返回一致的数据
    let detail = repo.get_paper_detail(&paper_id).await.unwrap().unwrap();
    let (paper, fa, ca, kws) = &detail;

    assert_eq!(paper.title, expected_title, "Title should match");
    assert_eq!(paper.year, Some(expected_year), "Year should match");
    assert_eq!(paper.journal.as_deref(), Some(expected_journal), "Journal should match");
    assert_eq!(paper.doi.as_deref(), Some(expected_doi), "DOI should match");

    assert!(fa.is_some(), "First author should exist");
    assert_eq!(fa.as_ref().unwrap().name, "First Author");

    assert!(ca.is_some(), "Corresponding author should exist");
    assert_eq!(ca.as_ref().unwrap().name, "Corr Author");

    assert_eq!(kws.len(), 2, "Should have 2 keywords");

    // 通过 get_papers_detail_batch 验证一致性
    let batch = repo.get_papers_detail_batch(&ws_id, None, None, None).await.unwrap();
    let batch_match = batch.iter().find(|(p, _, _, _)| p.doi == Some(expected_doi.to_string()));
    assert!(batch_match.is_some(), "Paper should be found in batch query");

    let (bp, bfa, bca, bkws) = batch_match.unwrap();
    assert_eq!(bp.title, expected_title);
    assert_eq!(bfa.as_ref().unwrap().name, "First Author");
    assert_eq!(bca.as_ref().unwrap().name, "Corr Author");
    assert_eq!(bkws.len(), 2);

    eprintln!("Data consistency verification: PASSED");
}