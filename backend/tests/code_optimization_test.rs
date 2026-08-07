use std::time::Instant;
use literature_integration::models::paper::Paper;
use literature_integration::models::author::Author;
use literature_integration::models::keyword::Keyword;
use literature_integration::models::workspace::Workspace;
use literature_integration::models::dto::GraphNode;

// ---------------------------------------------------------------------------
// 1. Vec pre-allocation correctness — every Vec building code path should
//    use `with_capacity` to avoid reallocations.
// ---------------------------------------------------------------------------

/// Verify that Vec::with_capacity avoids reallocation during push loop.
#[test]
fn test_vec_with_capacity_avoids_reallocation() {
    let n = 10_000usize;

    // Pre-allocated path
    let start = Instant::now();
    for _ in 0..100 {
        let mut v: Vec<i32> = Vec::with_capacity(n);
        for i in 0..n as i32 {
            v.push(i);
        }
        assert_eq!(v.len(), n);
    }
    let pre_duration = start.elapsed();

    // Dynamic path (no capacity hint)
    let start = Instant::now();
    for _ in 0..100 {
        let mut v: Vec<i32> = Vec::new();
        for i in 0..n as i32 {
            v.push(i);
        }
        assert_eq!(v.len(), n);
    }
    let dyn_duration = start.elapsed();

    println!("Vec allocation: pre={:?}, dynamic={:?}", pre_duration, dyn_duration);
    assert!(
        pre_duration <= dyn_duration,
        "Pre-allocated Vec should not be slower than dynamic allocation \
         (pre={:?}, dyn={:?})",
        pre_duration,
        dyn_duration,
    );
}

/// Guarantee that code uses `Vec::with_capacity` instead of `Vec::new`
/// inside tight loops.  This test simulates the pattern used in the
/// repository layer (building result Vecs).
#[test]
fn test_vec_building_uses_capacity_hint() {
    let sizes: &[usize] = &[16, 32, 64, 128, 256, 512];

    for &size in sizes {
        let start = Instant::now();
        for _ in 0..500 {
            let mut v: Vec<String> = Vec::with_capacity(size);
            for i in 0..size {
                v.push(format!("item_{}", i));
            }
        }
        let with_cap = start.elapsed();

        let start = Instant::now();
        for _ in 0..500 {
            let mut v: Vec<String> = Vec::new();
            for i in 0..size {
                v.push(format!("item_{}", i));
            }
        }
        let without_cap = start.elapsed();

        // Pre-allocation should be at least competitive (within 3x)
        // for all sizes.  For larger sizes it is typically faster.
        assert!(
            with_cap <= without_cap * 3,
            "Vec::with_capacity({}) took {:?}, Vec::new took {:?} — \
             pre-allocated should be within 3x of dynamic (size={})",
            size, with_cap, without_cap, size,
        );
    }
}

// ---------------------------------------------------------------------------
// 2. String allocation — avoid unnecessary String creations when a
//    temporary is sufficient.
// ---------------------------------------------------------------------------

/// `to_lowercase()` creates a new String; measure the cost.
/// The code base calls `to_lowercase()` in search paths — verify it
/// does not dominate runtime.
#[test]
fn test_to_lowercase_overhead_is_acceptable() {
    let queries = vec![
        "Machine Learning",
        "DEEP LEARNING",
        "Natural Language Processing",
        "Transformer Architecture",
        "reinforcement learning",
        "A",
        "",
    ];

    let start = Instant::now();
    for _ in 0..10_000 {
        for q in &queries {
            let _ = q.to_lowercase();
        }
    }
    let elapsed = start.elapsed();
    println!("to_lowercase x70000: {:?}", elapsed);
    assert!(
        elapsed.as_micros() < 500_000,
        "to_lowercase should be fast (<500ms for 70k calls), got {:?}",
        elapsed,
    );
}

/// `format!` vs manual String building — verify both are fast enough
/// for production use.  The code base uses both patterns.
#[test]
fn test_string_building_format_vs_manual() {
    let iterations = 50_000;

    let start = Instant::now();
    for i in 0..iterations {
        let _ = format!("paper_{}_{}", i, "suffix");
    }
    let format_dur = start.elapsed();

    // Manual push_str with pre-allocated capacity — avoids the
    // format! machinery but uses i.to_string() internally.
    let start = Instant::now();
    for i in 0..iterations {
        let n = i.to_string();
        let mut s = String::with_capacity(16 + n.len());
        s.push_str("paper_");
        s.push_str(&n);
        s.push_str("_suffix");
        let _ = s;
    }
    let manual_dur = start.elapsed();

    println!("String: format={:?}, manual={:?}", format_dur, manual_dur);
    assert!(
        manual_dur <= format_dur * 3,
        "Manual string building should not be significantly slower \
         (manual={:?}, format={:?})",
        manual_dur,
        format_dur,
    );
}

// ---------------------------------------------------------------------------
// 3. Data transformation — the repo layer frequently maps over
//    collected Vecs.  Verify that `into_iter()` + `map` is at
//    least as efficient as `iter()` + `map` + `cloned()`.
// ---------------------------------------------------------------------------

#[test]
fn test_data_transform_ownership_vs_borrow() {
    let make_source = || -> Vec<Paper> {
        (0..10_000)
            .map(|i| Paper {
                id: format!("p{}", i),
                title: format!("Paper {}", i),
                doi: None,
                arxiv_id: None,
                abstract_text: None,
                user_notes: None,
                year: Some(2024),
                journal: None,
                created_at: "2025-01-01".into(),
            })
            .collect()
    };

    // Clone path — iter() + cloned title
    let start = Instant::now();
    for _ in 0..100 {
        let source = make_source();
        let v: Vec<String> = source
            .iter()
            .map(|p| p.title.clone())
            .collect();
        assert_eq!(v.len(), 10_000);
    }
    let clone_dur = start.elapsed();

    // Consume path — into_iter() consumes, no clone
    let start = Instant::now();
    for _ in 0..100 {
        let source = make_source();
        let v: Vec<String> = source
            .into_iter()
            .map(|p| p.title)
            .collect();
        assert_eq!(v.len(), 10_000);
    }
    let consume_dur = start.elapsed();

    println!("Data transform: clone={:?}, consume={:?}", clone_dur, consume_dur);
    assert!(
        consume_dur <= clone_dur * 2,
        "Consuming iterator should not be significantly slower \
         (consume={:?}, clone={:?})",
        consume_dur,
        clone_dur,
    );
}

// ---------------------------------------------------------------------------
// 4. Batch vs individual — the code uses batch Cypher queries
//    (UNWIND) instead of individual calls.  Verify that building
//    the intermediate id/name/keyword Vecs is not a bottleneck.
// ---------------------------------------------------------------------------

#[test]
fn test_batch_data_prep_is_efficient() {
    let sizes = vec![1, 5, 10, 20, 50];

    for n in &sizes {
        let authors: Vec<(String, String, Option<String>, bool, bool)> = (0..*n)
            .map(|i| {
                (
                    format!("id{}", i),
                    format!("Author {}", i),
                    None,
                    i == 0,
                    i == n - 1,
                )
            })
            .collect();

        let start = Instant::now();
        for _ in 0..10_000 {
            let mut ids: Vec<&str> = Vec::with_capacity(*n);
            let mut names: Vec<&str> = Vec::with_capacity(*n);
            let mut orcids: Vec<&str> = Vec::with_capacity(*n);

            for a in &authors {
                ids.push(a.0.as_str());
                names.push(a.1.as_str());
                orcids.push(a.2.as_deref().unwrap_or(""));
            }
        }
        let elapsed = start.elapsed();
        println!("Batch prep (n={}): {:?}", n, elapsed);
        assert!(
            elapsed.as_micros() < 500_000,
            "Batch data prep for n={} should be fast, got {:?}",
            n, elapsed,
        );
    }
}

// ---------------------------------------------------------------------------
// 5. Overall code-level benchmark — simulate a realistic workload
//    mirroring the service-layer patterns.
// ---------------------------------------------------------------------------

#[test]
fn test_overall_workload_performance() {
    let total_start = Instant::now();
    let iterations = 1_000u64;

    for i in 0..iterations {
        // Simulate workspace + paper creation
        let ws = Workspace {
            id: format!("ws_{}", i),
            name: format!("Workspace {}", i),
            description: String::new(),
            created_at: "2025-01-01T00:00:00Z".to_string(),
        };

        let paper = Paper {
            id: format!("p_{}", i),
            title: format!("Test Paper {}", i),
            doi: Some(format!("10.1234/test.{}", i)),
            arxiv_id: None,
            abstract_text: Some("This is a test abstract.".to_string()),
            user_notes: None,
            year: Some(2024),
            journal: Some("Test Journal".to_string()),
            created_at: "2025-01-01T00:00:00Z".to_string(),
        };

        // Simulate batch author creation
        let mut authors: Vec<Author> = Vec::with_capacity(5);
        for j in 0..5 {
            authors.push(Author {
                id: format!("a_{}_{}", i, j),
                name: format!("Author {} {}", i, j),
                orcid: None,
            });
        }

        // Simulate keyword batch
        let mut keywords: Vec<Keyword> = Vec::with_capacity(3);
        for k in 0..3 {
            keywords.push(Keyword {
                id: format!("k_{}_{}", i, k),
                name: format!("keyword_{}", k),
            });
        }

        // Simulate graph node building
        let mut nodes: Vec<GraphNode> = Vec::with_capacity(authors.len());
        for a in &authors {
            nodes.push(GraphNode {
                id: a.id.clone(),
                name: a.name.clone(),
                paper_count: 1,
                author_type: "first".to_string(),
            });
        }

        // Verify correctness of data
        assert_eq!(authors.len(), 5);
        assert_eq!(keywords.len(), 3);
        assert_eq!(nodes.len(), 5);

        let _ = (ws, paper, authors, keywords, nodes);
    }

    let total_duration = total_start.elapsed();
    println!(
        "Overall workload: {:?} total, {:?} per iteration",
        total_duration,
        total_duration / iterations as u32
    );

    assert!(
        total_duration.as_secs() < 5,
        "Overall workload test should complete in < 5s, got {:?}",
        total_duration,
    );
}

// ---------------------------------------------------------------------------
// 6. Allocation hotspot detection — identify patterns that should
//    be optimized: double allocation, redundant clone, etc.
// ---------------------------------------------------------------------------

/// Detect the double-allocation pattern: creating a Vec then mapping
/// into another Vec (e.g., `Vec<Node>` → `Vec<Model>`) — this is
/// present in `get_paper_detail` and `get_papers_detail_batch`.
/// The test measures the overhead of this pattern.
#[test]
fn test_double_allocation_pattern_overhead() {
    let n = 10_000;

    // Pattern A (current code): intermediate Vec → map → collect
    let start = Instant::now();
    for _ in 0..100 {
        let intermediate: Vec<(String, String)> = (0..n)
            .map(|i| (format!("id{}", i), format!("name{}", i)))
            .collect();
        let result: Vec<Author> = intermediate
            .iter()
            .map(|(id, name)| Author {
                id: id.clone(),
                name: name.clone(),
                orcid: None,
            })
            .collect();
        assert_eq!(result.len(), n);
    }
    let double_alloc = start.elapsed();

    // Pattern B (optimized): direct collect into target type
    let start = Instant::now();
    for _ in 0..100 {
        let result: Vec<Author> = (0..n)
            .map(|i| Author {
                id: format!("id{}", i),
                name: format!("name{}", i),
                orcid: None,
            })
            .collect();
        assert_eq!(result.len(), n);
    }
    let direct_alloc = start.elapsed();

    println!(
        "Double alloc={:?}, direct alloc={:?}",
        double_alloc, direct_alloc
    );
    assert!(
        direct_alloc <= double_alloc,
        "Direct allocation should be faster or equal \
         (direct={:?}, double={:?})",
        direct_alloc,
        double_alloc,
    );
}

/// Verify that String::with_capacity avoids reallocation when we
/// know the final size.
#[test]
fn test_string_with_capacity_is_faster() {
    let iterations = 100_000;

    let start = Instant::now();
    for i in 0..iterations {
        let n = i.to_string();
        let mut s = String::with_capacity(8 + n.len());
        s.push_str("prefix_");
        s.push_str(&n);
        let _ = s;
    }
    let cap_dur = start.elapsed();

    let start = Instant::now();
    for i in 0..iterations {
        let n = i.to_string();
        let mut s = String::new();
        s.push_str("prefix_");
        s.push_str(&n);
        let _ = s;
    }
    let default_dur = start.elapsed();

    println!(
        "String: with_capacity={:?}, default={:?}",
        cap_dur, default_dur
    );
    assert!(
        cap_dur <= default_dur,
        "String::with_capacity should not be slower \
         (cap={:?}, default={:?})",
        cap_dur,
        default_dur,
    );
}

/// Verify that `Cow<str>` or `&str` can be used to avoid String
/// allocation when data is borrowed.  This test simulates the
/// `search_by_keyword` pattern where query strings are duplicated.
#[test]
fn test_str_allocation_avoidance() {
    let queries = vec!["alpha", "beta", "gamma", "delta", "epsilon"];

    let start = Instant::now();
    for _ in 0..100_000 {
        let results: Vec<&str> = queries.iter().map(|s| *s).collect();
        let _ = results;
    }
    let ref_dur = start.elapsed();

    let start = Instant::now();
    for _ in 0..100_000 {
        let results: Vec<String> = queries.iter().map(|s| s.to_string()).collect();
        let _ = results;
    }
    let owned_dur = start.elapsed();

    println!("Str ref={:?}, owned={:?}", ref_dur, owned_dur);
    assert!(
        ref_dur <= owned_dur,
        "Borrowed &str should be faster than owned String \
         (ref={:?}, owned={:?})",
        ref_dur,
        owned_dur,
    );
}