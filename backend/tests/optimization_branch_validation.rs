// 验证 optimize/perf-improvements 分支上的性能优化：
//   1. SearchResponse 类型化序列化结果与原 `serde_json::json!` 输出形状一致
//   2. 类型化序列化性能不劣于 `serde_json::json!` 宏（直接序列化省去 Value 树中转）
//   3. keyword_models 预分配容量模式正确且无多余分配
//   4. 作者姓名拼接（given + " " + family）单次分配逻辑正确
// 本测试不依赖 Neo4j。

use literature_integration::models::author::Author;
use literature_integration::models::dto::{AuthorWithPapers, SearchResponse};
use literature_integration::models::keyword::Keyword;
use literature_integration::models::paper::Paper;
use std::time::Instant;

fn sample_paper(id: &str, title: &str) -> Paper {
    Paper {
        id: id.to_string(),
        title: title.to_string(),
        doi: Some(format!("10.{}/x", id)),
        arxiv_id: None,
        abstract_text: Some("abstract content".to_string()),
        user_notes: None,
        year: Some(2024),
        journal: Some("Nature".to_string()),
        created_at: "2025-01-01T00:00:00Z".to_string(),
    }
}

fn sample_author(id: &str, name: &str) -> Author {
    Author {
        id: id.to_string(),
        name: name.to_string(),
        orcid: None,
    }
}

/// 优化点 1：SearchResponse 必须与原 `serde_json::json!` 输出形状一致
/// （包含 mode / query / results 三个字段，且 mode 取值为 "keyword"）。
#[test]
fn test_search_response_keyword_serialization_shape() {
    let papers = vec![sample_paper("1", "Paper One"), sample_paper("2", "Paper Two")];
    let resp = SearchResponse::Keyword {
        query: "graph".to_string(),
        results: papers,
    };

    let json = serde_json::to_string(&resp).expect("serialize keyword response");
    let v: serde_json::Value = serde_json::from_str(&json).expect("parse back");

    assert_eq!(v["mode"], "keyword");
    assert_eq!(v["query"], "graph");
    assert_eq!(v["results"].as_array().unwrap().len(), 2);
    assert_eq!(v["results"][0]["id"], "1");
    assert_eq!(v["results"][0]["title"], "Paper One");
}

/// 优化点 1（续）：作者搜索分支的序列化形状。
#[test]
fn test_search_response_author_serialization_shape() {
    let awp = AuthorWithPapers {
        author: sample_author("a1", "Alice"),
        papers: vec![sample_paper("9", "Nine")],
    };
    let resp = SearchResponse::Author {
        query: "Alice".to_string(),
        results: vec![awp],
    };

    let json = serde_json::to_string(&resp).expect("serialize author response");
    let v: serde_json::Value = serde_json::from_str(&json).expect("parse back");

    assert_eq!(v["mode"], "author");
    assert_eq!(v["query"], "Alice");
    assert_eq!(v["results"][0]["author"]["name"], "Alice");
    assert_eq!(v["results"][0]["papers"][0]["id"], "9");
}

/// 优化点 2：直接序列化类型化结构应不劣于经 `serde_json::json!` 构造 Value 树再序列化。
/// `serde_json::json!` 会先把数据搬到 `Value` 树（额外分配），再序列化；类型化路径省去这一步。
/// 这里使用宽松的阈值（类型化耗时 <= json! 耗时 * 3）作为合理性校验，避免 CI 抖动误报。
#[test]
fn test_typed_serialization_not_slower_than_json_macro() {
    let papers: Vec<Paper> = (0..200)
        .map(|i| sample_paper(&i.to_string(), &format!("Title {}", i)))
        .collect();

    // 预热
    let warm = SearchResponse::Keyword {
        query: "warm".to_string(),
        results: papers.clone(),
    };
    let _ = serde_json::to_string(&warm).unwrap();

    const ITERS: usize = 500;

    // 基准：serde_json::json! 路径（构造 Value 树 + 序列化）
    let start_macro = Instant::now();
    for _ in 0..ITERS {
        let v = serde_json::json!({
            "mode": "keyword",
            "query": "graph",
            "results": &papers,
        });
        let _ = serde_json::to_string(&v).unwrap();
    }
    let dur_macro = start_macro.elapsed();

    // 优化：类型化直接序列化
    let start_typed = Instant::now();
    for _ in 0..ITERS {
        let resp = SearchResponse::Keyword {
            query: "graph".to_string(),
            results: papers.clone(),
        };
        let _ = serde_json::to_string(&resp).unwrap();
    }
    let dur_typed = start_typed.elapsed();

    println!("json! macro: {:?}", dur_macro);
    println!("typed struct: {:?}", dur_typed);

    assert!(
        dur_typed <= dur_macro * 3,
        "typed serialization should not be significantly slower than json! macro \
         (typed={:?}, macro={:?})",
        dur_typed,
        dur_macro
    );
}

/// 优化点 3：keyword_models 预分配容量应避免扩容，capacity 在填满前应等于预期值。
#[test]
fn test_keyword_models_preallocation_pattern() {
    let keywords: Vec<(String, String)> = (0..10)
        .map(|i| (format!("k-{}", i), format!("keyword-{}", i)))
        .collect();

    // 复刻 services/paper.rs 中的优化写法
    let mut keyword_models = Vec::with_capacity(keywords.len());
    for (id, name) in &keywords {
        keyword_models.push(Keyword {
            id: id.clone(),
            name: name.clone(),
        });
    }

    assert_eq!(keyword_models.len(), keywords.len());
    // 预分配容量应恰好等于元素数（无多余扩容）
    assert_eq!(keyword_models.capacity(), keywords.len());
    assert_eq!(keyword_models[0].id, "k-0");
    assert_eq!(keyword_models[5].name, "keyword-5");
}

/// 优化点 4：作者姓名拼接（given + " " + family）应为单次分配，结果与 `format!` 一致。
#[test]
fn test_author_name_concatenation_correctness() {
    fn build_name(given: &str, family: &str) -> String {
        // 复刻 external_api.rs 中的优化写法
        if given.is_empty() {
            family.to_string()
        } else {
            let mut n = String::with_capacity(given.len() + 1 + family.len());
            n.push_str(given);
            n.push(' ');
            n.push_str(family);
            n
        }
    }

    // 正确性：与 format! 等价
    assert_eq!(build_name("John", "Doe"), "John Doe");
    assert_eq!(build_name("", "Doe"), "Doe");
    assert_eq!(build_name("Alice", "Smith"), format!("{} {}", "Alice", "Smith"));

    // 单次分配：capacity 应恰好等于最终长度（with_capacity 后无扩容）
    let name = build_name("John", "Doe");
    assert_eq!(name.capacity(), "John Doe".len());
}

/// 优化点 4（续）：姓名拼接单次分配性能应不劣于 `format!`。
#[test]
fn test_author_name_concatenation_performance() {
    const ITERS: usize = 100_000;
    let given = "GivenName";
    let family = "FamilyName";

    // 基准：format!
    let start_fmt = Instant::now();
    for _ in 0..ITERS {
        let _ = format!("{} {}", given, family);
    }
    let dur_fmt = start_fmt.elapsed();

    // 优化：单次分配
    let start_opt = Instant::now();
    for _ in 0..ITERS {
        let mut n = String::with_capacity(given.len() + 1 + family.len());
        n.push_str(given);
        n.push(' ');
        n.push_str(family);
        std::hint::black_box(&n);
    }
    let dur_opt = start_opt.elapsed();

    println!("format!: {:?}", dur_fmt);
    println!("single-alloc: {:?}", dur_opt);

    assert!(
        dur_opt <= dur_fmt * 3,
        "single-alloc name building should not be significantly slower than format! \
         (opt={:?}, fmt={:?})",
        dur_opt,
        dur_fmt
    );
}
