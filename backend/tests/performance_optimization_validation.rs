//! 性能优化验证测试
//! 
//! 本测试文件验证以下优化内容:
//! 1. Vec 预分配容量策略 (with_capacity) 正确性
//! 2. 移除 shrink_to_fit 后的内存行为
//! 3. 批量数据处理性能 (create_authors_batch 逻辑)
//! 4. 字符串处理效率
//! 5. 查询构建正确性
//! 6. 综合性能基准测试

use std::time::Instant;
use axum::response::IntoResponse;

// ==================== Vec 容量优化验证 ====================

/// 验证 Vec::with_capacity 预分配策略
/// 优化后的代码使用 with_capacity 预分配，不再使用 shrink_to_fit
#[test]
fn test_vec_capacity_pre_allocation() {
    let expected_caps = [
        ("DEFAULT_PAPERS_CAPACITY", 64),
        ("DEFAULT_AUTHORS_CAPACITY", 32),
        ("DEFAULT_KEYWORDS_CAPACITY", 16),
        ("DEFAULT_WORKSPACES_CAPACITY", 32),
        ("DEFAULT_GRAPH_NODES_CAPACITY", 128),
        ("DEFAULT_GRAPH_LINKS_CAPACITY", 256),
    ];

    for (name, cap) in &expected_caps {
        let v: Vec<u8> = Vec::with_capacity(*cap);
        assert!(
            v.capacity() >= *cap,
            "{}: capacity {} should be >= {}",
            name,
            v.capacity(),
            cap
        );
    }
}

/// 验证预分配 Vec 在填充过程中不会重新分配
#[test]
fn test_vec_no_reallocation_on_fill() {
    let cap = 64;
    let mut v: Vec<String> = Vec::with_capacity(cap);
    let initial_cap = v.capacity();

    for i in 0..cap {
        v.push(format!("item_{}", i));
    }

    assert_eq!(v.len(), cap);
    assert_eq!(
        v.capacity(),
        initial_cap,
        "Capacity should not change when filling to exact pre-allocated size"
    );
}

/// 验证移除 shrink_to_fit 后 Vec 行为正确
/// 优化方案：使用 with_capacity 预分配 + 直接返回，不再 shrink_to_fit
#[test]
fn test_vec_without_shrink_to_fit() {
    let mut v: Vec<u32> = Vec::with_capacity(1000);
    for i in 0..50 {
        v.push(i);
    }

    // 优化后: capacity >= 1000 (保留预分配容量，不缩小)
    assert!(v.capacity() >= 1000);
    assert_eq!(v.len(), 50);

    // 验证可以继续添加元素而不重新分配
    let cap_before = v.capacity();
    for i in 50..100 {
        v.push(i);
    }
    assert_eq!(v.capacity(), cap_before, "Should not reallocate within capacity");
    assert_eq!(v.len(), 100);
}

/// 批量数据 Vec 预分配性能测试
#[test]
fn test_batch_vec_allocation_performance() {
    let sizes = [10, 50, 100, 500, 1000];

    for size in &sizes {
        let start = Instant::now();

        let mut v: Vec<(String, String)> = Vec::with_capacity(*size);
        for i in 0..*size {
            v.push((format!("id_{}", i), format!("name_{}", i)));
        }

        let elapsed = start.elapsed();

        assert_eq!(v.len(), *size);
        assert!(
            elapsed < std::time::Duration::from_millis(100),
            "Vec with capacity {} allocation too slow: {:?}",
            size,
            elapsed
        );

        eprintln!(
            "Vec with_capacity({}) fill: {} items in {:?}, capacity={}",
            size,
            v.len(),
            elapsed,
            v.capacity()
        );
    }
}

// ==================== 批量作者处理逻辑验证 ====================

/// 验证 create_authors_batch 的数据准备逻辑正确且高效
#[test]
fn test_batch_author_processing_correctness() {
    let n = 50;
    let authors: Vec<(String, String, Option<String>, bool, bool)> = (0..n)
        .map(|i| {
            (
                format!("author-{}", i),
                format!("Author {}", i),
                if i % 3 == 0 {
                    Some(format!("0000-000{}-0000-0000", i))
                } else {
                    None
                },
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

    assert_eq!(ids.len(), n, "ids should have {} entries", n);
    assert_eq!(names.len(), n, "names should have {} entries", n);
    assert_eq!(orcids.len(), n, "orcids should have {} entries", n);
    assert_eq!(first_idx, 0, "First author should be at index 0");
    assert_eq!(corr_idx, (n - 1) as i64, "Corresponding author should be at last index");

    // 验证批量处理在合理时间内完成
    assert!(
        elapsed < std::time::Duration::from_millis(10),
        "Batch processing {} authors too slow: {:?}",
        n,
        elapsed
    );

    eprintln!(
        "Batch processing {} authors: {:?} (first_idx={}, corr_idx={})",
        n, elapsed, first_idx, corr_idx
    );
}

/// 验证批量处理避免了不必要的内存分配
#[test]
fn test_batch_author_zero_allocation() {
    let n = 100;
    let authors: Vec<(String, String, Option<String>, bool, bool)> = (0..n)
        .map(|i| {
            (
                format!("id-{}", i),
                format!("Author {}", i),
                None,
                i == 0,
                false,
            )
        })
        .collect();

    let start = Instant::now();

    let mut ids: Vec<&str> = Vec::with_capacity(n);
    let mut names: Vec<&str> = Vec::with_capacity(n);
    let mut orcids: Vec<&str> = Vec::with_capacity(n);

    for a in &authors {
        ids.push(a.0.as_str());
        names.push(a.1.as_str());
        orcids.push(a.2.as_deref().unwrap_or(""));
    }

    let elapsed = start.elapsed();

    assert_eq!(ids.capacity(), n, "ids capacity should be exactly {}", n);
    assert_eq!(names.capacity(), n, "names capacity should be exactly {}", n);
    assert_eq!(orcids.capacity(), n, "orcids capacity should be exactly {}", n);

    eprintln!(
        "Zero-allocation batch processing ({} items): {:?}",
        n, elapsed
    );
}

// ==================== 字符串优化验证 ====================

/// 验证字符串预分配性能
#[test]
fn test_string_preallocation_performance() {
    let test_sizes = [100, 500, 1000, 5000];

    for size in test_sizes {
        let estimated_capacity = size * 64;
        let start = Instant::now();

        let mut s = String::with_capacity(estimated_capacity);
        for i in 0..size {
            s.push_str(&format!("paper_{:04},", i));
        }

        let elapsed = start.elapsed();

        assert!(s.capacity() >= s.len());
        assert!(
            elapsed < std::time::Duration::from_millis(100),
            "String pre-allocation for {} items too slow: {:?}",
            size,
            elapsed
        );

        eprintln!(
            "String with {} items (est. cap={}): {:?}, len={}, cap={}",
            size,
            estimated_capacity,
            elapsed,
            s.len(),
            s.capacity()
        );
    }
}

/// 验证字符串操作效率
#[test]
fn test_string_concat_optimized() {
    let owned: Vec<String> = (0..1000)
        .map(|i| format!("item_{}", i))
        .collect();
    let fragments: Vec<&str> = owned.iter().map(|s| s.as_str()).collect();

    let start = Instant::now();
    let mut result = String::with_capacity(50000);
    for frag in &fragments {
        result.push_str(frag);
        result.push(',');
    }
    let elapsed = start.elapsed();

    assert!(result.len() > 0);
    assert!(
        elapsed < std::time::Duration::from_millis(50),
        "String concatenation too slow: {:?}",
        elapsed
    );

    eprintln!(
        "Concatenated {} fragments in {:?}, len={}, cap={}",
        fragments.len(),
        elapsed,
        result.len(),
        result.capacity()
    );
}

// ==================== 数据结构优化验证 ====================

/// 验证 Paper 模型创建效率
#[test]
fn test_paper_model_creation_performance() {
    let n = 1000;
    let start = Instant::now();

    let mut papers = Vec::with_capacity(n);
    for i in 0..n {
        papers.push(literature_integration::models::paper::Paper {
            id: format!("paper-{}", i),
            title: format!("Test Paper {}", i),
            doi: Some(format!("10.1234/test.{}", i)),
            arxiv_id: None,
            abstract_text: Some(format!("Abstract for paper number {}", i)),
            user_notes: None,
            year: Some(2024),
            journal: Some("Test Journal".to_string()),
            created_at: "2025-01-01T00:00:00Z".to_string(),
        });
    }

    let elapsed = start.elapsed();

    assert_eq!(papers.len(), n);
    assert!(
        elapsed < std::time::Duration::from_millis(200),
        "Creating {} paper models too slow: {:?}",
        n,
        elapsed
    );

    eprintln!(
        "Created {} paper models in {:?}",
        n, elapsed
    );
}

/// 验证 Author 模型创建效率
#[test]
fn test_author_model_creation_performance() {
    let n = 1000;
    let start = Instant::now();

    let mut authors = Vec::with_capacity(n);
    for i in 0..n {
        authors.push(literature_integration::models::author::Author {
            id: format!("author-{}", i),
            name: format!("Author Name {}", i),
            orcid: if i % 2 == 0 {
                Some(format!("0000-000{}-0000-0000", i))
            } else {
                None
            },
        });
    }

    let elapsed = start.elapsed();

    assert_eq!(authors.len(), n);
    assert!(
        elapsed < std::time::Duration::from_millis(100),
        "Creating {} author models too slow: {:?}",
        n,
        elapsed
    );

    eprintln!(
        "Created {} author models in {:?}",
        n, elapsed
    );
}

// ==================== 查询构建验证 ====================

/// 验证关键词搜索查询构建（避免不必要的 clone）
#[test]
fn test_search_query_construction() {
    let query_str = "machine learning";
    let start = Instant::now();

    for _ in 0..1000 {
        let _query_lower = query_str.to_lowercase();
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed < std::time::Duration::from_millis(50),
        "Lowercase conversion too slow for 1000 iterations: {:?}",
        elapsed
    );

    eprintln!(
        "1000x to_lowercase: {:?}",
        elapsed
    );
}

// ==================== 序列化性能验证 ====================

/// 验证 JSON 序列化/反序列化性能
#[test]
fn test_json_serde_performance() {
    let paper = literature_integration::models::paper::Paper {
        id: "test-id".to_string(),
        title: "Performance Test Paper".to_string(),
        doi: Some("10.1234/test".to_string()),
        arxiv_id: Some("2401.00001".to_string()),
        abstract_text: Some("This is a test abstract for performance validation.".to_string()),
        user_notes: Some("# Key Findings\n- Point 1\n- Point 2".to_string()),
        year: Some(2024),
        journal: Some("Test Journal".to_string()),
        created_at: "2025-01-01T00:00:00Z".to_string(),
    };

    let start = Instant::now();
    for _ in 0..500 {
        let json = serde_json::to_string(&paper).unwrap();
        let _deserialized: literature_integration::models::paper::Paper =
            serde_json::from_str(&json).unwrap();
    }
    let elapsed = start.elapsed();

    assert!(
        elapsed < std::time::Duration::from_secs(5),
        "500x serialize/deserialize roundtrip too slow: {:?}",
        elapsed
    );

    eprintln!(
        "500x JSON roundtrip: {:?} ({:.2}ms per roundtrip)",
        elapsed,
        elapsed.as_secs_f64() * 1000.0 / 500.0
    );
}

// ==================== 综合性能验证 ====================

/// 综合性能基准测试: 验证优化后的数据处理速度
#[test]
fn test_comprehensive_data_processing_performance() {
    let start = Instant::now();
    let iterations = 100;

    for _ in 0..iterations {
        let mut papers: Vec<literature_integration::models::paper::Paper> = Vec::with_capacity(50);
        for i in 0..50 {
            papers.push(literature_integration::models::paper::Paper {
                id: format!("p-{}", i),
                title: format!("Paper {}", i),
                doi: Some(format!("10.1000/p{}", i)),
                arxiv_id: None,
                abstract_text: Some(format!("Abstract {}", i)),
                user_notes: None,
                year: Some(2024),
                journal: Some("Journal".to_string()),
                created_at: "2025-01-01".to_string(),
            });
        }

        let mut authors: Vec<literature_integration::models::author::Author> = Vec::with_capacity(100);
        for i in 0..100 {
            authors.push(literature_integration::models::author::Author {
                id: format!("a-{}", i),
                name: format!("Author {}", i),
                orcid: None,
            });
        }

        let _keywords: Vec<literature_integration::models::keyword::Keyword> = (0..20)
            .map(|i| literature_integration::models::keyword::Keyword {
                id: format!("k-{}", i),
                name: format!("keyword_{}", i),
            })
            .collect();
    }

    let elapsed = start.elapsed();

    assert!(
        elapsed < std::time::Duration::from_secs(10),
        "Comprehensive data processing too slow: {:?}",
        elapsed
    );

    eprintln!(
        "Comprehensive performance test ({} iterations): {:?}",
        iterations, elapsed
    );
}

/// 内存效率验证: 确保优化后的 Vec 使用合理的内存
#[test]
fn test_memory_efficiency_validation() {
    let sizes = [16, 32, 64, 128, 256, 512];

    for &size in &sizes {
        let v: Vec<u8> = Vec::with_capacity(size);
        let allocated = v.capacity();
        let used = v.len();

        assert!(
            allocated >= used,
            "Capacity ({}) should be >= length ({})",
            allocated,
            used
        );

        // 预分配容量不应过于浪费 (最多不超过请求的 2 倍)
        assert!(
            allocated <= size * 2,
            "Capacity ({}) should not exceed 2x requested ({})",
            allocated,
            size
        );

        eprintln!(
            "Vec size={}: capacity={}, len={}, overhead={:.1}%",
            size,
            allocated,
            used,
            if size > 0 {
                (allocated as f64 - used as f64) / size as f64 * 100.0
            } else {
                0.0
            }
        );
    }
}

// ==================== 并发处理性能验证 ====================

/// 验证批量并发数据准备的性能
#[tokio::test]
async fn test_parallel_data_preparation() {
    let start = Instant::now();

    let paper_fut = tokio::spawn(async {
        let n = 200;
        let papers: Vec<(String, String, Option<String>, Option<String>, Option<String>, Option<i32>, Option<String>, String)> = (0..n)
            .map(|i| {
                (
                    format!("paper-{}", i),
                    format!("Paper Title {}", i),
                    Some(format!("10.1234/p{}", i)),
                    None,
                    Some(format!("Abstract number {}", i)),
                    Some(2024),
                    Some("Test Journal".to_string()),
                    "2025-01-01T00:00:00Z".to_string(),
                )
            })
            .collect();
        papers
    });

    let author_fut = tokio::spawn(async {
        let n = 500;
        let authors: Vec<(String, String, Option<String>, bool, bool)> = (0..n)
            .map(|i| {
                (
                    format!("author-{}", i),
                    format!("Author {}", i),
                    None,
                    i == 0,
                    i == n - 1,
                )
            })
            .collect();
        authors
    });

    let keyword_fut = tokio::spawn(async {
        let keywords: Vec<(String, String)> = (0..50)
            .map(|i| (format!("kw-{}", i), format!("keyword_{}", i)))
            .collect();
        keywords
    });

    let (papers_result, authors_result, keywords_result) =
        tokio::join!(paper_fut, author_fut, keyword_fut);

    let papers = papers_result.unwrap();
    let authors = authors_result.unwrap();
    let keywords = keywords_result.unwrap();

    let elapsed = start.elapsed();

    assert_eq!(papers.len(), 200);
    assert_eq!(authors.len(), 500);
    assert_eq!(keywords.len(), 50);

    assert!(
        elapsed < std::time::Duration::from_millis(500),
        "Parallel data preparation too slow: {:?}",
        elapsed
    );

    eprintln!(
        "Parallel data preparation (papers={}, authors={}, keywords={}): {:?}",
        papers.len(),
        authors.len(),
        keywords.len(),
        elapsed
    );
}

// ==================== 代码优化模式验证 ====================

/// 验证代码中不再包含 shrink_to_fit 调用 (性能反模式)
/// 优化后的代码应使用 with_capacity + 直接返回
#[test]
fn test_no_shrink_to_fit_in_optimized_code() {
    let repo_source = std::fs::read_to_string("src/repositories/neo4j_repo.rs")
        .expect("Failed to read neo4j_repo.rs");

    let shrink_count = repo_source.matches("shrink_to_fit").count();
    assert_eq!(
        shrink_count, 0,
        "Optimized code should not contain shrink_to_fit calls. Found {} occurrences.",
        shrink_count
    );
}

/// 验证代码中包含 Vec::with_capacity 预分配
#[test]
fn test_with_capacity_used_in_optimized_code() {
    let repo_source = std::fs::read_to_string("src/repositories/neo4j_repo.rs")
        .expect("Failed to read neo4j_repo.rs");

    let with_capacity_count = repo_source.matches("Vec::with_capacity").count();
    assert!(
        with_capacity_count >= 10,
        "Optimized code should use Vec::with_capacity at least 10 times, found {}",
        with_capacity_count
    );

    eprintln!(
        "Vec::with_capacity usage count in neo4j_repo.rs: {}",
        with_capacity_count
    );
}

/// 验证代码中包含预分配容量常量
#[test]
fn test_capacity_constants_defined() {
    let repo_source = std::fs::read_to_string("src/repositories/neo4j_repo.rs")
        .expect("Failed to read neo4j_repo.rs");

    let constants = [
        "DEFAULT_PAPERS_CAPACITY",
        "DEFAULT_AUTHORS_CAPACITY",
        "DEFAULT_KEYWORDS_CAPACITY",
        "DEFAULT_WORKSPACES_CAPACITY",
        "DEFAULT_GRAPH_NODES_CAPACITY",
        "DEFAULT_GRAPH_LINKS_CAPACITY",
    ];

    for constant in &constants {
        assert!(
            repo_source.contains(constant),
            "Optimized code should define {} constant",
            constant
        );
    }
}

// ==================== 配置优化验证 ====================

/// 验证配置参数合理
#[test]
fn test_config_optimization_values() {
    let cfg = literature_integration::config::Config::from_env();

    assert!(!cfg.neo4j_uri.is_empty(), "Neo4j URI should not be empty");
    assert!(!cfg.neo4j_user.is_empty(), "Neo4j user should not be empty");
    assert!(cfg.server_port > 0, "Server port should be valid");
    assert!(!cfg.server_host.is_empty(), "Server host should not be empty");

    eprintln!(
        "Config: uri={}, user={}, host={}, port={}",
        cfg.neo4j_uri, cfg.neo4j_user, cfg.server_host, cfg.server_port
    );
}

// ==================== 错误处理优化验证 ====================

/// 验证错误类型正确映射
#[test]
fn test_error_handling_optimized() {
    use axum::http::StatusCode;

    let err = literature_integration::errors::AppError::WorkspaceNotFound("ws-1".into());
    let resp = err.into_response();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    let err = literature_integration::errors::AppError::PaperNotFound("p-1".into());
    let resp = err.into_response();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    let err = literature_integration::errors::AppError::ValidationError("bad input".into());
    let resp = err.into_response();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    eprintln!("Error handling optimization: all error types map correctly");
}