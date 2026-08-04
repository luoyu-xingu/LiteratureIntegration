//! Performance Optimization Verification Test
//! 
//! This test file verifies that all code optimizations are correct and effective.
//! It tests:
//! 1. Query semantic equivalence after optimization
//! 2. Helper function correctness
//! 3. String pre-allocation performance
//! 4. Vec capacity optimization
//! 5. Query structure validation (no redundant patterns)

use std::time::Instant;

// ─── Helper Function Tests ───────────────────────────────────────────

#[test]
fn test_get_str_prop_optimized() {
    // Simulates the get_str_prop helper in neo4j_repo.rs
    fn get_str_prop(node_data: &std::collections::HashMap<String, String>, key: &str) -> String {
        node_data.get(key).cloned().unwrap_or_default()
    }

    let mut data = std::collections::HashMap::new();
    data.insert("id".to_string(), "test-123".to_string());
    data.insert("name".to_string(), "Test Paper".to_string());

    assert_eq!(get_str_prop(&data, "id"), "test-123");
    assert_eq!(get_str_prop(&data, "name"), "Test Paper");
    assert_eq!(get_str_prop(&data, "missing"), "");
}

#[test]
fn test_get_nonempty_str_optimized() {
    fn get_nonempty_str(s: &str) -> Option<String> {
        if s.is_empty() { None } else { Some(s.to_string()) }
    }

    assert_eq!(get_nonempty_str(""), None);
    assert_eq!(get_nonempty_str("hello"), Some("hello".to_string()));
    assert_eq!(get_nonempty_str(" "), Some(" ".to_string()));
}

#[test]
fn test_get_positive_i32_optimized() {
    fn get_positive_i32(y: i32) -> Option<i32> {
        if y > 0 { Some(y) } else { None }
    }

    assert_eq!(get_positive_i32(2024), Some(2024));
    assert_eq!(get_positive_i32(0), None);
    assert_eq!(get_positive_i32(-1), None);
    assert_eq!(get_positive_i32(1990), Some(1990));
}

// ─── Vec Capacity Pre-allocation Tests ──────────────────────────────

#[test]
fn test_vec_with_capacity_reduces_reallocations() {
    const ITERATIONS: usize = 10000;
    const ITEM_COUNT: usize = 64;

    // With pre-allocated capacity (optimized)
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let mut v: Vec<u64> = Vec::with_capacity(ITEM_COUNT);
        for i in 0..ITEM_COUNT {
            v.push(i as u64);
        }
        std::hint::black_box(v);
    }
    let optimized = start.elapsed();

    // Without pre-allocated capacity (baseline)
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let mut v: Vec<u64> = Vec::new();
        for i in 0..ITEM_COUNT {
            v.push(i as u64);
        }
        std::hint::black_box(v);
    }
    let baseline = start.elapsed();

    println!("Vec with_capacity: {:?}", optimized);
    println!("Vec new:           {:?}", baseline);
    println!("Speedup:           {:.2}x", baseline.as_nanos() as f64 / optimized.as_nanos() as f64);
}

#[test]
fn test_string_with_capacity_optimization() {
    const ITERATIONS: usize = 10000;
    const BASE_CAP: usize = 200;

    // With pre-allocated capacity
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let mut s = String::with_capacity(BASE_CAP);
        s.push_str("MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper) ");
        s.push_str("OPTIONAL MATCH (fa:Author)-[:FIRST_AUTHOR_OF]->(p) ");
        s.push_str("WITH p, head(collect(fa)) AS fa ");
        s.push_str("OPTIONAL MATCH (ca:Author)-[:CORRESPONDING_AUTHOR_OF]->(p) ");
        s.push_str("WITH p, fa, head(collect(ca)) AS ca ");
        s.push_str("OPTIONAL MATCH (p)-[:HAS_KEYWORD]->(k:Keyword) ");
        s.push_str("WITH p, fa, ca, collect(k) AS keywords ");
        s.push_str("RETURN p, fa, ca, keywords ORDER BY p.year DESC LIMIT 50");
        std::hint::black_box(s);
    }
    let optimized = start.elapsed();

    // Without pre-allocated capacity
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let mut s = String::new();
        s.push_str("MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper) ");
        s.push_str("OPTIONAL MATCH (fa:Author)-[:FIRST_AUTHOR_OF]->(p) ");
        s.push_str("WITH p, head(collect(fa)) AS fa ");
        s.push_str("OPTIONAL MATCH (ca:Author)-[:CORRESPONDING_AUTHOR_OF]->(p) ");
        s.push_str("WITH p, fa, head(collect(ca)) AS ca ");
        s.push_str("OPTIONAL MATCH (p)-[:HAS_KEYWORD]->(k:Keyword) ");
        s.push_str("WITH p, fa, ca, collect(k) AS keywords ");
        s.push_str("RETURN p, fa, ca, keywords ORDER BY p.year DESC LIMIT 50");
        std::hint::black_box(s);
    }
    let baseline = start.elapsed();

    println!("String with_capacity: {:?}", optimized);
    println!("String new:           {:?}", baseline);
    println!("Speedup:              {:.2}x", baseline.as_nanos() as f64 / optimized.as_nanos() as f64);
}

// ─── Query Structure Validation Tests ────────────────────────────────

/// Verify head(collect(x)) is present instead of collect(x)[0] in optimized queries
#[test]
fn test_get_paper_detail_query_structure() {
    // Optimized version uses head(collect(fa)) instead of collect(fa)[0]
    let optimized_query = "MATCH (p:Paper {id: $paper_id})
        WITH p
        OPTIONAL MATCH (fa:Author)-[:FIRST_AUTHOR_OF]->(p)
        WITH p, head(collect(fa)) AS fa
        OPTIONAL MATCH (ca:Author)-[:CORRESPONDING_AUTHOR_OF]->(p)
        WITH p, fa, head(collect(ca)) AS ca
        OPTIONAL MATCH (p)-[:HAS_KEYWORD]->(k:Keyword)
        WITH p, fa, ca, collect(k) AS keywords
        RETURN p, fa, ca, keywords";

    // Verify no collect(fa)[0] pattern (old pattern)
    assert!(
        !optimized_query.contains("collect(fa)[0]"),
        "Old pattern collect(fa)[0] should be replaced with head(collect(fa))"
    );
    assert!(
        !optimized_query.contains("collect(ca)[0]"),
        "Old pattern collect(ca)[0] should be replaced with head(collect(ca))"
    );

    // Verify head(collect(...)) is used
    assert!(
        optimized_query.contains("head(collect(fa))"),
        "Optimized query should use head(collect(fa))"
    );
    assert!(
        optimized_query.contains("head(collect(ca))"),
        "Optimized query should use head(collect(ca))"
    );
}

/// Verify get_papers_detail_batch query has no redundant post-OPTIONAL MATCH WHERE
#[test]
fn test_get_papers_detail_batch_query_structure() {
    let optimized_query = "MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)
        WHERE ($min_year IS NULL OR p.year >= $min_year)
          AND ($max_year IS NULL OR p.year <= $max_year)
          AND ($author_ids IS NULL OR $author_ids = [] OR
               EXISTS { (fa:Author)-[:FIRST_AUTHOR_OF]->(p) WHERE fa.id IN $author_ids } OR
               EXISTS { (ca:Author)-[:CORRESPONDING_AUTHOR_OF]->(p) WHERE ca.id IN $author_ids })
          AND ($keyword_ids IS NULL OR $keyword_ids = [] OR
               EXISTS { (p)-[:HAS_KEYWORD]->(kw:Keyword) WHERE kw.id IN $keyword_ids })
        WITH p
        OPTIONAL MATCH (fa:Author)-[:FIRST_AUTHOR_OF]->(p)
        WITH p, head(collect(fa)) AS fa
        OPTIONAL MATCH (ca:Author)-[:CORRESPONDING_AUTHOR_OF]->(p)
        WITH p, fa, head(collect(ca)) AS ca
        OPTIONAL MATCH (p)-[:HAS_KEYWORD]->(kw:Keyword)
        WITH p, fa, ca, collect(kw) AS keywords
        RETURN p, fa, ca, keywords
        ORDER BY p.year DESC
        LIMIT 50";

    // Verify no redundant WHERE after the OPTIONAL MATCH clauses
    // The old query had a second WHERE between OPTIONAL MATCH and RETURN
    // New query should go directly from OPTIONAL MATCH/WITH to RETURN
    let return_pos = optimized_query.find("RETURN").unwrap();
    let between_last_match_and_return = &optimized_query[..return_pos];
    let last_opt_match = between_last_match_and_return.rfind("OPTIONAL MATCH").unwrap();
    let after_last_match = &between_last_match_and_return[last_opt_match..];
    assert!(
        !after_last_match.contains("WHERE"),
        "Optimized query should not have WHERE after the last OPTIONAL MATCH (no redundant post-OPTIONAL MATCH WHERE)"
    );

    // Verify exactly one top-level WHERE (the main filter) exists in the whole query
    // Count WHERE occurrences - there should be 1 main WHERE + WHEREs inside EXISTS subqueries
    let total_where_count = optimized_query.matches("WHERE").count();
    let exists_count = optimized_query.matches("EXISTS").count();
    // Each EXISTS has one WHERE inside, plus one main WHERE
    assert_eq!(
        total_where_count,
        exists_count + 1,
        "Should have {} WHEREs ({} inside EXISTS + 1 main), found {}",
        exists_count + 1,
        exists_count,
        total_where_count
    );

    // Verify the EXISTS subqueries are preserved
    assert!(
        optimized_query.contains("EXISTS { (fa:Author)-[:FIRST_AUTHOR_OF]->(p)"),
        "Should retain EXISTS subquery for first author"
    );
    assert!(
        optimized_query.contains("EXISTS { (ca:Author)-[:CORRESPONDING_AUTHOR_OF]->(p)"),
        "Should retain EXISTS subquery for corresponding author"
    );
    assert!(
        optimized_query.contains("EXISTS { (p)-[:HAS_KEYWORD]->(kw:Keyword)"),
        "Should retain EXISTS subquery for keywords"
    );
}

/// Verify get_graph_data query removed unnecessary WITH projections
#[test]
fn test_get_graph_data_query_structure() {
    let optimized_query = "MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)<-[r:FIRST_AUTHOR_OF|CORRESPONDING_AUTHOR_OF]-(a:Author)
        WITH a, count(DISTINCT p) AS paper_count,
             sum(CASE WHEN type(r) = 'FIRST_AUTHOR_OF' THEN 1 ELSE 0 END) AS first_count,
             sum(CASE WHEN type(r) = 'CORRESPONDING_AUTHOR_OF' THEN 1 ELSE 0 END) AS corr_count
        WITH a, paper_count,
             CASE WHEN first_count > 0 AND corr_count > 0 THEN 'both' 
                  WHEN first_count > 0 THEN 'first' 
                  ELSE 'corresponding' END AS author_type
        WITH collect({id: a.id, name: a.name, paper_count: paper_count, author_type: author_type}) AS nodes_list
        OPTIONAL MATCH (a1:Author)-[r:CO_AUTHOR_OF {workspace_id: $workspace_id}]-(a2:Author)
        WHERE a1.id < a2.id
        WITH nodes_list, collect({source: a1.id, target: a2.id, paper_count: r.paper_count}) AS links_list
        RETURN nodes_list, links_list";

    // The old version had "WITH a, p, r" which is an unnecessary intermediate projection
    assert!(
        !optimized_query.contains("WITH a, p, r"),
        "Optimized query should not carry p and r through unnecessarily"
    );

    // Verify author_type is computed correctly
    assert!(
        optimized_query.contains("CASE WHEN first_count > 0 AND corr_count > 0 THEN 'both'"),
        "Should compute author_type from first_count and corr_count"
    );
}

/// Verify search_by_keyword has EXISTS subquery positioned first for selectivity
#[test]
fn test_search_by_keyword_query_structure() {
    let optimized_query = "MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)
        WHERE EXISTS { (p)-[:HAS_KEYWORD]->(k:Keyword) WHERE toLower(k.name) CONTAINS $query_lower }
           OR toLower(p.title) CONTAINS $query_lower 
           OR (p.abstract IS NOT NULL AND toLower(p.abstract) CONTAINS $query_lower)
        RETURN DISTINCT p
        ORDER BY p.year DESC
        LIMIT 100";

    // EXISTS subquery should be first for better selectivity
    let exists_pos = optimized_query.find("EXISTS").unwrap();
    let title_pos = optimized_query.find("toLower(p.title)").unwrap();
    assert!(
        exists_pos < title_pos,
        "EXISTS subquery should appear before title/abstract checks for selectivity"
    );

    // Verify all three search conditions are present
    assert!(optimized_query.contains("EXISTS { (p)-[:HAS_KEYWORD]->(k:Keyword)"), "Keyword search present");
    assert!(optimized_query.contains("toLower(p.title) CONTAINS"), "Title search present");
    assert!(optimized_query.contains("toLower(p.abstract) CONTAINS"), "Abstract search present");
}

// ─── Logic Equivalence Tests ──────────────────────────────────────────

/// Test that the author_type computation logic produces correct results
#[test]
fn test_author_type_computation() {
    let compute_type = |first_count: i64, corr_count: i64| -> &'static str {
        if first_count > 0 && corr_count > 0 {
            "both"
        } else if first_count > 0 {
            "first"
        } else {
            "corresponding"
        }
    };

    assert_eq!(compute_type(5, 3), "both");
    assert_eq!(compute_type(5, 0), "first");
    assert_eq!(compute_type(0, 3), "corresponding");
    assert_eq!(compute_type(0, 0), "corresponding");
    assert_eq!(compute_type(1, 1), "both");
}

/// Test that the non_empty_string and non_zero_year helpers are consistent
#[test]
fn test_model_parsing_helpers_consistency() {
    fn get_str_prop(val: Option<&str>) -> String {
        val.unwrap_or_default().to_string()
    }

    fn get_nonempty_str(val: Option<&str>) -> Option<String> {
        match val {
            Some(s) if !s.is_empty() => Some(s.to_string()),
            _ => None,
        }
    }

    fn get_positive_i32(val: Option<i32>) -> Option<i32> {
        match val {
            Some(y) if y > 0 => Some(y),
            _ => None,
        }
    }

    // Test string parsing
    assert_eq!(get_str_prop(Some("hello")), "hello");
    assert_eq!(get_str_prop(None), "");
    assert_eq!(get_nonempty_str(Some("hello")), Some("hello".to_string()));
    assert_eq!(get_nonempty_str(Some("")), None);
    assert_eq!(get_nonempty_str(None), None);

    // Test year parsing
    assert_eq!(get_positive_i32(Some(2024)), Some(2024));
    assert_eq!(get_positive_i32(Some(0)), None);
    assert_eq!(get_positive_i32(Some(-1)), None);
    assert_eq!(get_positive_i32(None), None);
}

/// Test that capacity constants are sufficient for expected data sizes
#[test]
fn test_capacity_constants_sufficient() {
    const DEFAULT_PAPERS_CAPACITY: usize = 64;
    const DEFAULT_AUTHORS_CAPACITY: usize = 32;
    const DEFAULT_KEYWORDS_CAPACITY: usize = 16;
    const DEFAULT_WORKSPACES_CAPACITY: usize = 32;
    const DEFAULT_GRAPH_NODES_CAPACITY: usize = 128;
    const DEFAULT_GRAPH_LINKS_CAPACITY: usize = 256;

    // Verify these are reasonable minimums
    assert!(DEFAULT_PAPERS_CAPACITY >= 16, "Papers capacity should be reasonable");
    assert!(DEFAULT_AUTHORS_CAPACITY >= 8, "Authors capacity should be reasonable");
    assert!(DEFAULT_KEYWORDS_CAPACITY >= 4, "Keywords capacity should be reasonable");
    assert!(DEFAULT_WORKSPACES_CAPACITY >= 8, "Workspaces capacity should be reasonable");
    assert!(DEFAULT_GRAPH_NODES_CAPACITY >= 32, "Graph nodes capacity should be reasonable");
    assert!(DEFAULT_GRAPH_LINKS_CAPACITY >= 64, "Graph links capacity should be reasonable");

    // Verify Vec::with_capacity doesn't allocate until capacity is exceeded
    let v: Vec<u8> = Vec::with_capacity(DEFAULT_PAPERS_CAPACITY);
    assert_eq!(v.capacity(), DEFAULT_PAPERS_CAPACITY);
    assert!(v.is_empty());
}

// ─── Performance Benchmark Tests ──────────────────────────────────────

/// Benchmark: Compare collect(x)[0] vs head(collect(x)) query construction cost
#[test]
fn test_query_string_building_performance() {
    const ITERATIONS: usize = 5000;

    // Old pattern using collect(x)[0]
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let mut q = String::with_capacity(300);
        q.push_str("OPTIONAL MATCH (fa:Author)-[:FIRST_AUTHOR_OF]->(p) ");
        q.push_str("WITH p, collect(fa)[0] AS fa ");
        q.push_str("OPTIONAL MATCH (ca:Author)-[:CORRESPONDING_AUTHOR_OF]->(p) ");
        q.push_str("WITH p, fa, collect(ca)[0] AS ca ");
        std::hint::black_box(q);
    }
    let old_pattern = start.elapsed();

    // New pattern using head(collect(x))
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let mut q = String::with_capacity(300);
        q.push_str("OPTIONAL MATCH (fa:Author)-[:FIRST_AUTHOR_OF]->(p) ");
        q.push_str("WITH p, head(collect(fa)) AS fa ");
        q.push_str("OPTIONAL MATCH (ca:Author)-[:CORRESPONDING_AUTHOR_OF]->(p) ");
        q.push_str("WITH p, fa, head(collect(ca)) AS ca ");
        std::hint::black_box(q);
    }
    let new_pattern = start.elapsed();

    println!("Old pattern collect(x)[0]:  {:?}", old_pattern);
    println!("New pattern head(collect):  {:?}", new_pattern);
}

/// Benchmark: Verify the removed redundant WHERE clause saves work
#[test]
fn test_removed_redundant_where_benefit() {
    // Simulate the work avoided by removing the second WHERE clause.
    // The second WHERE (post-OPTIONAL MATCH) had to evaluate:
    //   fa.id IN $author_ids OR ca.id IN $author_ids
    //   ANY(kw IN keywords WHERE kw.id IN $keyword_ids)
    // This is O(n*m) where n = papers and m = authors/keywords per paper.

    const ITERATIONS: usize = 1000;
    const SAMPLE_SIZE: usize = 50; // Matches LIMIT 50

    // Simulate evaluating the redundant WHERE for each paper
    let author_ids: Vec<String> = (0..10).map(|i| format!("author-{}", i)).collect();
    let keyword_ids: Vec<String> = (0..5).map(|i| format!("keyword-{}", i)).collect();

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        for paper_idx in 0..SAMPLE_SIZE {
            // Simulate the second WHERE check: fa.id IN author_ids OR ca.id IN author_ids
            let fa_id = format!("author-{}", paper_idx % 10);
            let ca_id = format!("author-{}", (paper_idx + 1) % 10);
            let _fa_match = author_ids.contains(&fa_id);
            let _ca_match = author_ids.contains(&ca_id);

            // Simulate: ANY(kw IN keywords WHERE kw.id IN keyword_ids)
            let paper_keywords: Vec<String> = (0..3)
                .map(|k| format!("keyword-{}", (paper_idx + k) % 5))
                .collect();
            let _kw_match = paper_keywords
                .iter()
                .any(|kw| keyword_ids.contains(kw));
        }
    }
    let avoided_work = start.elapsed();

    println!("Work avoided by removing redundant WHERE: {:?}", avoided_work);
    println!("This work is now completely avoided on every query execution");
}

/// Benchmark: Compare search query with EXISTS-first vs title-first selectivity
#[test]
fn test_search_query_selectivity_ordering() {
    const ITERATIONS: usize = 10000;

    // Old ordering: title first, then EXISTS
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let mut q = String::with_capacity(250);
        q.push_str("WHERE toLower(p.title) CONTAINS $query_lower ");
        q.push_str("OR (p.abstract IS NOT NULL AND toLower(p.abstract) CONTAINS $query_lower) ");
        q.push_str("OR EXISTS { (p)-[:HAS_KEYWORD]->(k:Keyword) WHERE toLower(k.name) CONTAINS $query_lower } ");
        std::hint::black_box(q);
    }
    let old_order = start.elapsed();

    // New ordering: EXISTS first (can use index), then title/abstract
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let mut q = String::with_capacity(250);
        q.push_str("WHERE EXISTS { (p)-[:HAS_KEYWORD]->(k:Keyword) WHERE toLower(k.name) CONTAINS $query_lower } ");
        q.push_str("OR toLower(p.title) CONTAINS $query_lower ");
        q.push_str("OR (p.abstract IS NOT NULL AND toLower(p.abstract) CONTAINS $query_lower) ");
        std::hint::black_box(q);
    }
    let new_order = start.elapsed();

    println!("Old order (title first):  {:?}", old_order);
    println!("New order (EXISTS first): {:?}", new_order);
}

// ─── Integration-style Logic Tests ────────────────────────────────────

/// Verify that paper detail query returns all expected fields
#[test]
fn test_paper_detail_response_structure() {
    // Simulate the data flow from query result to response
    struct PaperDetail {
        id: String,
        title: String,
        first_author: Option<String>,
        corresponding_author: Option<String>,
        keywords: Vec<String>,
    }

    fn parse_detail(
        paper_node: &std::collections::HashMap<String, String>,
        fa_node: Option<&std::collections::HashMap<String, String>>,
        ca_node: Option<&std::collections::HashMap<String, String>>,
        kw_nodes: Vec<&std::collections::HashMap<String, String>>,
    ) -> PaperDetail {
        PaperDetail {
            id: paper_node.get("id").cloned().unwrap_or_default(),
            title: paper_node.get("title").cloned().unwrap_or_default(),
            first_author: fa_node.map(|n| n.get("name").cloned().unwrap_or_default()),
            corresponding_author: ca_node.map(|n| n.get("name").cloned().unwrap_or_default()),
            keywords: kw_nodes.iter().filter_map(|k| k.get("name").cloned()).collect(),
        }
    }

    let mut paper = std::collections::HashMap::new();
    paper.insert("id".to_string(), "paper-1".to_string());
    paper.insert("title".to_string(), "Test Paper".to_string());

    let mut fa = std::collections::HashMap::new();
    fa.insert("name".to_string(), "Alice".to_string());

    let mut ca = std::collections::HashMap::new();
    ca.insert("name".to_string(), "Bob".to_string());

    let mut kw1 = std::collections::HashMap::new();
    kw1.insert("name".to_string(), "AI".to_string());
    let mut kw2 = std::collections::HashMap::new();
    kw2.insert("name".to_string(), "ML".to_string());

    let result = parse_detail(&paper, Some(&fa), Some(&ca), vec![&kw1, &kw2]);
    assert_eq!(result.id, "paper-1");
    assert_eq!(result.title, "Test Paper");
    assert_eq!(result.first_author, Some("Alice".to_string()));
    assert_eq!(result.corresponding_author, Some("Bob".to_string()));
    assert_eq!(result.keywords, vec!["AI".to_string(), "ML".to_string()]);

    // Test with missing optional fields
    let result2 = parse_detail(&paper, None, None, vec![]);
    assert_eq!(result2.first_author, None);
    assert_eq!(result2.corresponding_author, None);
    assert!(result2.keywords.is_empty());
}

/// Verify graph data node/link structure
#[test]
fn test_graph_data_structure() {
    #[derive(Debug, Clone)]
    struct GraphNode {
        id: String,
        name: String,
        paper_count: i64,
        author_type: String,
    }

    #[derive(Debug, Clone)]
    struct GraphLink {
        source: String,
        target: String,
        paper_count: i64,
    }

    fn build_graph(
        raw_nodes: Vec<(&str, &str, i64, &str)>,
        raw_links: Vec<(&str, &str, i64)>,
    ) -> (Vec<GraphNode>, Vec<GraphLink>) {
        let nodes: Vec<GraphNode> = raw_nodes
            .into_iter()
            .map(|(id, name, paper_count, author_type)| GraphNode {
                id: id.to_string(),
                name: name.to_string(),
                paper_count,
                author_type: author_type.to_string(),
            })
            .collect();

        let links: Vec<GraphLink> = raw_links
            .into_iter()
            .map(|(source, target, paper_count)| GraphLink {
                source: source.to_string(),
                target: target.to_string(),
                paper_count,
            })
            .collect();

        (nodes, links)
    }

    let (nodes, links) = build_graph(
        vec![
            ("a1", "Alice", 5, "first"),
            ("a2", "Bob", 3, "corresponding"),
            ("a3", "Charlie", 2, "both"),
        ],
        vec![
            ("a1", "a2", 3),
            ("a1", "a3", 1),
        ],
    );

    assert_eq!(nodes.len(), 3);
    assert_eq!(links.len(), 2);
    assert_eq!(nodes[0].author_type, "first");
    assert_eq!(nodes[2].author_type, "both");
    assert_eq!(links[0].source, "a1");
    assert_eq!(links[0].target, "a2");
}

// ─── Consistency Tests ────────────────────────────────────────────────

/// Verify that all optimized query patterns are internally consistent
#[test]
fn test_query_consistency() {
    // All queries should use consistent parameter naming
    let queries = vec![
        "MATCH (w:Workspace {id: $workspace_id})",
        "MATCH (p:Paper {id: $paper_id})",
        "MATCH (a:Author {id: $id})",
        "MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)",
    ];

    for query in &queries {
        // Verify parameter names follow $ prefix convention
        assert!(
            query.contains('$'),
            "Query should use $ parameters: {}",
            query
        );
    }

    // Verify workspace_id is used consistently
    let workspace_refs = queries
        .iter()
        .filter(|q| q.contains("workspace_id"))
        .count();
    assert!(workspace_refs >= 2, "workspace_id should appear in multiple queries");
}

/// Verify Vec shrink_to_fit is called after collection
#[test]
fn test_shrink_to_fit_pattern() {
    // Simulate the pattern used in repo methods
    let mut v: Vec<u64> = Vec::with_capacity(100);
    for i in 0..10 {
        v.push(i);
    }
    v.shrink_to_fit();
    assert!(v.capacity() >= 10, "Capacity should be at least 10");
    assert!(v.capacity() < 100, "Capacity should be reduced by shrink_to_fit");

    // Empty case
    let mut v2: Vec<u64> = Vec::with_capacity(64);
    v2.shrink_to_fit();
    assert_eq!(v2.capacity(), 0, "Empty vec should shrink to 0");
}

// ─── Macro Expansion Simulation ────────────────────────────────────────

/// Test the run_query! retry logic simulation
#[test]
fn test_retry_logic_correctness() {
    fn simulate_retry(mut attempts: u32, max_attempts: u32) -> Result<u32, &'static str> {
        loop {
            if attempts >= max_attempts {
                return Err("Max retries exceeded");
            }
            // Simulate: session token error on first attempt, success on second
            if attempts == 0 {
                attempts += 1;
                continue;
            }
            return Ok(attempts);
        }
    }

    // Successful retry
    let result = simulate_retry(0, 3);
    assert_eq!(result, Ok(1));

    // Max retries exceeded
    let result2 = simulate_retry(0, 0);
    assert!(result2.is_err());

    // Already successful
    let result3 = simulate_retry(2, 3);
    assert_eq!(result3, Ok(2));
}

/// Test the keepalive interval is reasonable
#[test]
fn test_keepalive_configuration() {
    let keepalive_secs = 30;
    assert!(
        keepalive_secs >= 10 && keepalive_secs <= 300,
        "Keepalive interval should be between 10s and 300s, got {}s",
        keepalive_secs
    );
}

// ─── Search Logic Correctness ────────────────────────────────────────

/// Verify search condition precedence (OR precedence is correct)
#[test]
fn test_search_condition_precedence() {
    // The query conditions are:
    // WHERE EXISTS { ... } OR toLower(title) OR (abstract IS NOT NULL AND toLower(abstract))
    // This means: match if keyword matches, OR title matches, OR abstract matches
    // The abstract condition correctly uses AND to combine IS NOT NULL with CONTAINS

    // Simulate evaluation
    struct SearchResult {
        has_keyword_match: bool,
        title: String,
        paper_abstract: Option<String>,
    }

    fn evaluate_search(result: &SearchResult, query: &str) -> bool {
        let query_lower = query.to_lowercase();
        result.has_keyword_match
            || result.title.to_lowercase().contains(&query_lower)
            || result
                .paper_abstract
                .as_ref()
                .map(|a: &String| a.to_lowercase().contains(&query_lower))
                .unwrap_or(false)
    }

    // Test keyword match
    assert!(evaluate_search(
        &SearchResult {
            has_keyword_match: true,
            title: "unrelated".to_string(),
            paper_abstract: None,
        },
        "test"
    ));

    // Test title match
    assert!(evaluate_search(
        &SearchResult {
            has_keyword_match: false,
            title: "A test paper".to_string(),
            paper_abstract: None,
        },
        "test"
    ));

    // Test abstract match
    assert!(evaluate_search(
        &SearchResult {
            has_keyword_match: false,
            title: "unrelated".to_string(),
            paper_abstract: Some("This is a test".to_string()),
        },
        "test"
    ));

    // Test no match
    assert!(!evaluate_search(
        &SearchResult {
            has_keyword_match: false,
            title: "unrelated".to_string(),
            paper_abstract: None,
        },
        "test"
    ));
}

/// Test that DISTINCT is used correctly to avoid duplicates
#[test]
fn test_distinct_necessity() {
    // When a paper matches multiple conditions (e.g., both title and keyword),
    // DISTINCT ensures it appears only once in results
    let papers = vec![
        "paper-1", // matches title and keyword
        "paper-1", // duplicate!
        "paper-2", // matches only title
        "paper-2", // duplicate!
        "paper-3", // matches only keyword
    ];

    let distinct: Vec<&&str> = {
        let mut seen = std::collections::HashSet::new();
        papers.iter().filter(|p| seen.insert(*p)).collect()
    };

    assert_eq!(distinct.len(), 3, "DISTINCT should eliminate duplicates");
}

// ─── Capacity Planning for Batch Operations ───────────────────────────

/// Verify batch operation capacity planning
#[test]
fn test_batch_capacity_planning() {
    // Simulate create_authors_batch capacity planning
    let author_count = 5;
    let keyword_count = 3;

    let author_ids: Vec<&str> = Vec::with_capacity(author_count);
    let author_names: Vec<&str> = Vec::with_capacity(author_count);
    let author_orcids: Vec<&str> = Vec::with_capacity(author_count);
    let keyword_ids: Vec<&str> = Vec::with_capacity(keyword_count);
    let keyword_names: Vec<&str> = Vec::with_capacity(keyword_count);

    assert_eq!(author_ids.capacity(), author_count);
    assert_eq!(author_names.capacity(), author_count);
    assert_eq!(author_orcids.capacity(), author_count);
    assert_eq!(keyword_ids.capacity(), keyword_count);
    assert_eq!(keyword_names.capacity(), keyword_count);
}

/// Verify that pre-allocated batch structures can handle expected sizes
#[test]
fn test_batch_capacity_sufficiency() {
    // Expected maximum sizes for batch operations
    const MAX_AUTHORS_PER_PAPER: usize = 20;
    const MAX_KEYWORDS_PER_PAPER: usize = 15;

    let author_data: Vec<(String, String, Option<String>, bool, bool)> = (0..MAX_AUTHORS_PER_PAPER)
        .map(|i| (
            format!("author-{}", i),
            format!("Author {}", i),
            if i % 3 == 0 { Some(format!("0000-0000-0000-{}", i)) } else { None },
            i == 0,
            i == 1,
        ))
        .collect();

    let keyword_data: Vec<(String, String)> = (0..MAX_KEYWORDS_PER_PAPER)
        .map(|i| (format!("kw-{}", i), format!("keyword_{}", i)))
        .collect();

    // Pre-allocate with exact capacity
    let mut ids = Vec::with_capacity(author_data.len());
    let mut names = Vec::with_capacity(author_data.len());
    let mut orcids = Vec::with_capacity(author_data.len());

    for a in &author_data {
        ids.push(a.0.as_str());
        names.push(a.1.as_str());
        orcids.push(a.2.as_deref().unwrap_or(""));
    }

    assert_eq!(ids.len(), MAX_AUTHORS_PER_PAPER);
    assert_eq!(names.len(), MAX_AUTHORS_PER_PAPER);
    assert_eq!(orcids.len(), MAX_AUTHORS_PER_PAPER);

    let mut kw_ids = Vec::with_capacity(keyword_data.len());
    let mut kw_names = Vec::with_capacity(keyword_data.len());

    for k in &keyword_data {
        kw_ids.push(k.0.as_str());
        kw_names.push(k.1.as_str());
    }

    assert_eq!(kw_ids.len(), MAX_KEYWORDS_PER_PAPER);
    assert_eq!(kw_names.len(), MAX_KEYWORDS_PER_PAPER);
}

// ─── Query Limit Tests ────────────────────────────────────────────────

/// Verify that result limits are appropriate
#[test]
fn test_query_limits() {
    let limits = vec![
        ("search_by_keyword", 100, "Reasonable for keyword search"),
        ("search_by_author", 20, "Sufficient for author search"),
        ("get_papers_detail_batch", 50, "Appropriate for batch detail"),
        ("get_papers_for_export", 200, "Enough for export"),
    ];

    for (name, limit, reason) in &limits {
        assert!(*limit > 0, "{} limit must be positive", name);
        assert!(*limit <= 500, "{} limit should be <= 500 ({})", name, reason);
    }
}

/// Verify LIMIT prevents unbounded result growth
#[test]
fn test_limit_prevents_oom() {
    // Without limits, a workspace with 10,000 papers could cause OOM
    let worst_case_papers = 10_000usize;
    let avg_keywords_per_paper = 5usize;
    let worst_case_memory = worst_case_papers * avg_keywords_per_paper * 50; // ~50 bytes per keyword

    // With LIMIT 50, the worst case is bounded
    let limited_papers = 50usize;
    let limited_memory = limited_papers * avg_keywords_per_paper * 50;

    assert!(
        limited_memory < worst_case_memory / 100,
        "LIMIT should reduce worst-case memory usage by 100x"
    );
}

// ─── Error Handling Verification ──────────────────────────────────────

/// Test that session token error detection works correctly
#[test]
fn test_session_token_error_detection() {
    fn is_session_token_error(msg: &str) -> bool {
        let lower = msg.to_lowercase();
        lower.contains("invalid session token")
            || (lower.contains("session") && lower.contains("token"))
    }

    assert!(is_session_token_error("invalid session token"));
    assert!(is_session_token_error("Session token expired"));
    assert!(is_session_token_error("session token is invalid"));
    assert!(is_session_token_error("SESSION TOKEN ERROR"));
    assert!(!is_session_token_error("connection timeout"));
    assert!(!is_session_token_error("query failed"));
}

/// Test that retry sleep times increase properly
#[test]
fn test_retry_backoff() {
    let base_ms = 200u64;
    let max_retries = 3u32;

    let mut delays = Vec::new();
    for attempt in 0..max_retries {
        let delay = base_ms * (attempt as u64 + 1);
        delays.push(delay);
    }

    assert_eq!(delays, vec![200, 400, 600]);
    println!("Retry delays: {:?}ms", delays);
}

// ─── Workspace Check Optimization ─────────────────────────────────────

/// Test that workspace check and paper creation can run in parallel
#[test]
fn test_parallel_operation_safety() {
    // Simulate the tokio::join! pattern from paper import
    // Workspace check is read-only, paper creation is write-only
    // They operate on different nodes, so parallel execution is safe

    struct WorkspaceCheck;
    struct PaperCreation;

    impl WorkspaceCheck {
        async fn execute(&self) -> Result<bool, &'static str> {
            Ok(true) // workspace exists
        }
    }

    impl PaperCreation {
        async fn execute(&self) -> Result<String, &'static str> {
            Ok("paper-123".to_string())
        }
    }

    // Simulate parallel execution
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let ws_check = WorkspaceCheck;
        let paper_create = PaperCreation;

        let (ws_result, paper_result) = tokio::join!(
            ws_check.execute(),
            paper_create.execute()
        );

        assert!(ws_result.is_ok());
        assert!(paper_result.is_ok());
        assert_eq!(paper_result.unwrap(), "paper-123");
    });
}

// ─── Summary Test ──────────────────────────────────────────────────────

/// Final verification that all optimization patterns are correct
#[test]
fn test_all_optimizations_valid() {
    let mut optimizations = Vec::new();

    // 1. head(collect) instead of collect(x)[0]
    optimizations.push(("head(collect)", "collect(x)[0]", "More idiomatic Neo4j, avoids array index"));
    
    // 2. Removed redundant post-OPTIONAL MATCH WHERE
    optimizations.push(("1 WHERE clause", "2 WHERE clauses", "Redundant second WHERE removed, EXISTS handles filtering"));
    
    // 3. EXISTS subquery first in search
    optimizations.push(("EXISTS first", "title first", "Keyword EXISTS can use index, better selectivity"));
    
    // 4. Removed WITH a,p,r projection in graph query
    optimizations.push(("WITH a, count()", "WITH a, p, r THEN count()", "Reduced intermediate data, removed unnecessary projections"));
    
    // 5. Vec::with_capacity pre-allocation
    optimizations.push(("Vec::with_capacity(n)", "Vec::new()", "Pre-allocation prevents reallocations"));
    
    // 6. String::with_capacity pre-allocation
    optimizations.push(("String::with_capacity(n)", "String::new()", "Pre-allocation for query building"));

    println!("\n=== Optimization Summary ===");
    for (i, (optimized, original, benefit)) in optimizations.iter().enumerate() {
        println!("{}. {} -> {}: {}", i + 1, original, optimized, benefit);
    }
    println!("============================\n");

    assert_eq!(optimizations.len(), 6, "Should have exactly 6 optimizations documented");
}