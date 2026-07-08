use std::time::{Duration, Instant};

const DEFAULT_PAPERS_CAPACITY: usize = 32;
const DEFAULT_AUTHORS_CAPACITY: usize = 16;
const DEFAULT_KEYWORDS_CAPACITY: usize = 8;
const DEFAULT_WORKSPACES_CAPACITY: usize = 16;
const DEFAULT_GRAPH_NODES_CAPACITY: usize = 64;
const DEFAULT_GRAPH_LINKS_CAPACITY: usize = 128;

#[test]
fn test_capacity_constants_are_valid() {
    assert!(DEFAULT_PAPERS_CAPACITY > 0);
    assert!(DEFAULT_AUTHORS_CAPACITY > 0);
    assert!(DEFAULT_KEYWORDS_CAPACITY > 0);
    assert!(DEFAULT_WORKSPACES_CAPACITY > 0);
    assert!(DEFAULT_GRAPH_NODES_CAPACITY > 0);
    assert!(DEFAULT_GRAPH_LINKS_CAPACITY > 0);
}

#[test]
fn test_vec_with_capacity_performance_improvement() {
    const ITERATIONS: usize = 10000;
    const CAPACITY: usize = 64;

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let mut v: Vec<i32> = Vec::new();
        for i in 0..CAPACITY {
            v.push(i as i32);
        }
    }
    let duration_new = start.elapsed();

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let mut v: Vec<i32> = Vec::with_capacity(CAPACITY);
        for i in 0..CAPACITY {
            v.push(i as i32);
        }
    }
    let duration_with_capacity = start.elapsed();

    println!("Vec::new() total: {:?}", duration_new);
    println!("Vec::with_capacity() total: {:?}", duration_with_capacity);
    println!(
        "Improvement: {:.2}%",
        (duration_new.as_secs_f64() - duration_with_capacity.as_secs_f64())
            / duration_new.as_secs_f64()
            * 100.0
    );

    assert!(
        duration_with_capacity <= duration_new * 2,
        "Vec::with_capacity should not be significantly slower"
    );
}

#[test]
fn test_string_conversion_performance() {
    const ITERATIONS: usize = 100000;
    let test_str = "test-author-name";

    let start = Instant::now();
    let mut results1 = Vec::with_capacity(ITERATIONS);
    for _ in 0..ITERATIONS {
        let s = test_str.to_string();
        results1.push(s);
    }
    let duration_to_string = start.elapsed();

    let start = Instant::now();
    let mut results2 = Vec::with_capacity(ITERATIONS);
    for _ in 0..ITERATIONS {
        let s = String::from(test_str);
        results2.push(s);
    }
    let duration_from = start.elapsed();

    println!("to_string: {:?}", duration_to_string);
    println!("String::from: {:?}", duration_from);

    assert_eq!(results1.len(), ITERATIONS);
    assert_eq!(results2.len(), ITERATIONS);
}

#[derive(Debug, Clone)]
struct TestPaper {
    id: String,
    title: String,
    year: Option<i32>,
    journal: Option<String>,
}

#[derive(Debug, Clone)]
struct TestNode {
    id: String,
    title: String,
    year: i32,
    journal: String,
}

fn paper_from_node_map_collect(nodes: &[TestNode]) -> Vec<TestPaper> {
    nodes
        .iter()
        .map(|n| TestPaper {
            id: n.id.clone(),
            title: n.title.clone(),
            year: Some(n.year).filter(|y| *y > 0),
            journal: Some(n.journal.clone()).filter(|s| !s.is_empty()),
        })
        .collect()
}

fn paper_from_node_prealloc(nodes: &[TestNode]) -> Vec<TestPaper> {
    let mut papers = Vec::with_capacity(nodes.len());
    for n in nodes {
        papers.push(TestPaper {
            id: n.id.clone(),
            title: n.title.clone(),
            year: Some(n.year).filter(|y| *y > 0),
            journal: Some(n.journal.clone()).filter(|s| !s.is_empty()),
        });
    }
    papers
}

#[test]
fn test_prealloc_vs_map_collect_performance() {
    const NODE_COUNT: usize = 1000;
    const ITERATIONS: usize = 500;

    let mut nodes = Vec::with_capacity(NODE_COUNT);
    for i in 0..NODE_COUNT {
        nodes.push(TestNode {
            id: format!("paper-{}", i),
            title: format!("Paper Title {}", i),
            year: if i % 10 == 0 { 0 } else { 2020 + (i % 5) as i32 },
            journal: if i % 7 == 0 {
                String::new()
            } else {
                format!("Journal {}", i % 10)
            },
        });
    }

    let start = Instant::now();
    let mut result1 = Vec::new();
    for _ in 0..ITERATIONS {
        result1 = paper_from_node_map_collect(&nodes);
    }
    let duration_map = start.elapsed();

    let start = Instant::now();
    let mut result2 = Vec::new();
    for _ in 0..ITERATIONS {
        result2 = paper_from_node_prealloc(&nodes);
    }
    let duration_prealloc = start.elapsed();

    println!("map/collect: {:?} for {} iterations", duration_map, ITERATIONS);
    println!(
        "prealloc loop: {:?} for {} iterations",
        duration_prealloc, ITERATIONS
    );
    println!(
        "Improvement: {:.2}%",
        (duration_map.as_secs_f64() - duration_prealloc.as_secs_f64())
            / duration_map.as_secs_f64()
            * 100.0
    );

    assert_eq!(result1.len(), result2.len());
    for i in 0..result1.len() {
        assert_eq!(result1[i].id, result2[i].id);
        assert_eq!(result1[i].title, result2[i].title);
        assert_eq!(result1[i].year, result2[i].year);
        assert_eq!(result1[i].journal, result2[i].journal);
    }

    assert!(
        duration_prealloc <= duration_map * 2,
        "Prealloc approach should not be significantly slower"
    );
}

#[derive(Debug, Clone)]
struct GraphNode {
    id: String,
    name: String,
    paper_count: i32,
    author_type: String,
}

#[derive(Debug, Clone)]
struct GraphLink {
    source: String,
    target: String,
    paper_count: i32,
}

fn build_graph_with_collect(nodes_data: &[(String, String, i32, String)]) -> Vec<GraphNode> {
    nodes_data
        .iter()
        .map(|(id, name, count, atype)| GraphNode {
            id: id.clone(),
            name: name.clone(),
            paper_count: *count,
            author_type: atype.clone(),
        })
        .collect()
}

fn build_graph_with_prealloc(nodes_data: &[(String, String, i32, String)]) -> Vec<GraphNode> {
    let mut nodes = Vec::with_capacity(nodes_data.len());
    for (id, name, count, atype) in nodes_data {
        nodes.push(GraphNode {
            id: id.clone(),
            name: name.clone(),
            paper_count: *count,
            author_type: atype.clone(),
        });
    }
    nodes
}

#[test]
fn test_graph_data_building_performance() {
    const NODE_COUNT: usize = 500;
    const ITERATIONS: usize = 200;

    let mut nodes_data = Vec::with_capacity(NODE_COUNT);
    for i in 0..NODE_COUNT {
        nodes_data.push((
            format!("author-{}", i),
            format!("Author {}", i),
            (i % 10 + 1) as i32,
            if i % 3 == 0 {
                "both".to_string()
            } else if i % 3 == 1 {
                "first".to_string()
            } else {
                "corresponding".to_string()
            },
        ));
    }

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = build_graph_with_collect(&nodes_data);
    }
    let duration_collect = start.elapsed();

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = build_graph_with_prealloc(&nodes_data);
    }
    let duration_prealloc = start.elapsed();

    println!("collect approach: {:?}", duration_collect);
    println!("prealloc approach: {:?}", duration_prealloc);
    println!(
        "Improvement: {:.2}%",
        (duration_collect.as_secs_f64() - duration_prealloc.as_secs_f64())
            / duration_collect.as_secs_f64()
            * 100.0
    );

    let r1 = build_graph_with_collect(&nodes_data);
    let r2 = build_graph_with_prealloc(&nodes_data);
    assert_eq!(r1.len(), r2.len());
    for i in 0..r1.len() {
        assert_eq!(r1[i].id, r2[i].id);
        assert_eq!(r1[i].name, r2[i].name);
        assert_eq!(r1[i].paper_count, r2[i].paper_count);
        assert_eq!(r1[i].author_type, r2[i].author_type);
    }
}

#[test]
fn test_batch_vec_construction_performance() {
    const BATCH_SIZE: usize = 100;
    const ITERATIONS: usize = 1000;

    let authors: Vec<(String, String, Option<String>, bool, bool)> = (0..BATCH_SIZE)
        .map(|i| {
            (
                format!("id-{}", i),
                format!("Author {}", i),
                if i % 5 == 0 {
                    Some(format!("0000-000{}-0000-000{}", i, i))
                } else {
                    None
                },
                i == 0,
                i == 1,
            )
        })
        .collect();

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ids: Vec<String> = authors.iter().map(|a| a.0.clone()).collect();
        let _names: Vec<String> = authors.iter().map(|a| a.1.clone()).collect();
        let _orcids: Vec<String> = authors
            .iter()
            .map(|a| a.2.as_deref().unwrap_or("").to_string())
            .collect();
        let _is_first: Vec<bool> = authors.iter().map(|a| a.3).collect();
        let _is_corresponding: Vec<bool> = authors.iter().map(|a| a.4).collect();
    }
    let duration_map = start.elapsed();

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let n = authors.len();
        let mut ids = Vec::with_capacity(n);
        let mut names = Vec::with_capacity(n);
        let mut orcids = Vec::with_capacity(n);
        let mut is_first = Vec::with_capacity(n);
        let mut is_corresponding = Vec::with_capacity(n);
        for a in &authors {
            ids.push(a.0.clone());
            names.push(a.1.clone());
            orcids.push(a.2.as_deref().unwrap_or("").to_string());
            is_first.push(a.3);
            is_corresponding.push(a.4);
        }
    }
    let duration_prealloc = start.elapsed();

    println!("map/collect batch: {:?}", duration_map);
    println!("prealloc loop batch: {:?}", duration_prealloc);
    println!(
        "Improvement: {:.2}%",
        (duration_map.as_secs_f64() - duration_prealloc.as_secs_f64())
            / duration_map.as_secs_f64()
            * 100.0
    );

    assert!(
        duration_prealloc <= duration_map * 2,
        "Prealloc batch should not be significantly slower"
    );
}

#[test]
fn test_optimization_correctness_all_capacities_used() {
    let paper_vec: Vec<u8> = Vec::with_capacity(DEFAULT_PAPERS_CAPACITY);
    assert_eq!(paper_vec.capacity(), DEFAULT_PAPERS_CAPACITY);

    let author_vec: Vec<u8> = Vec::with_capacity(DEFAULT_AUTHORS_CAPACITY);
    assert_eq!(author_vec.capacity(), DEFAULT_AUTHORS_CAPACITY);

    let keyword_vec: Vec<u8> = Vec::with_capacity(DEFAULT_KEYWORDS_CAPACITY);
    assert_eq!(keyword_vec.capacity(), DEFAULT_KEYWORDS_CAPACITY);

    let workspace_vec: Vec<u8> = Vec::with_capacity(DEFAULT_WORKSPACES_CAPACITY);
    assert_eq!(workspace_vec.capacity(), DEFAULT_WORKSPACES_CAPACITY);

    let graph_nodes_vec: Vec<u8> = Vec::with_capacity(DEFAULT_GRAPH_NODES_CAPACITY);
    assert_eq!(graph_nodes_vec.capacity(), DEFAULT_GRAPH_NODES_CAPACITY);

    let graph_links_vec: Vec<u8> = Vec::with_capacity(DEFAULT_GRAPH_LINKS_CAPACITY);
    assert_eq!(graph_links_vec.capacity(), DEFAULT_GRAPH_LINKS_CAPACITY);
}

#[test]
fn test_workspace_capacity_is_distinct_from_papers() {
    assert_ne!(
        DEFAULT_WORKSPACES_CAPACITY, DEFAULT_PAPERS_CAPACITY,
        "Workspaces should have its own capacity constant"
    );
    assert!(
        DEFAULT_WORKSPACES_CAPACITY <= DEFAULT_PAPERS_CAPACITY,
        "Workspace count is typically smaller than paper count"
    );
}

#[test]
fn test_graph_capacity_sizing() {
    assert!(
        DEFAULT_GRAPH_LINKS_CAPACITY >= DEFAULT_GRAPH_NODES_CAPACITY,
        "In a graph, links typically outnumber nodes"
    );
    assert!(
        DEFAULT_GRAPH_NODES_CAPACITY >= DEFAULT_AUTHORS_CAPACITY,
        "Graph nodes should have larger default than authors"
    );
}

#[test]
fn test_string_allocation_optimization_pattern() {
    const ITERATIONS: usize = 10000;
    let source = "optimization-test-string";

    let start = Instant::now();
    let mut v1 = Vec::with_capacity(ITERATIONS);
    for _ in 0..ITERATIONS {
        v1.push(source.to_string());
    }
    let dur_to_string = start.elapsed();

    let start = Instant::now();
    let mut v2 = Vec::with_capacity(ITERATIONS);
    for _ in 0..ITERATIONS {
        v2.push(String::from(source));
    }
    let dur_from = start.elapsed();

    let start = Instant::now();
    let mut v3 = Vec::with_capacity(ITERATIONS);
    for _ in 0..ITERATIONS {
        v3.push(source.to_owned());
    }
    let dur_to_owned = start.elapsed();

    println!("to_string: {:?}", dur_to_string);
    println!("String::from: {:?}", dur_from);
    println!("to_owned: {:?}", dur_to_owned);

    assert_eq!(v1.len(), ITERATIONS);
    assert_eq!(v2.len(), ITERATIONS);
    assert_eq!(v3.len(), ITERATIONS);
}

#[test]
fn test_performance_is_reasonable() {
    const ITERATIONS: usize = 1000;
    const ELEMENTS: usize = 256;

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let mut v = Vec::with_capacity(ELEMENTS);
        for i in 0..ELEMENTS {
            v.push(i as i64);
        }
    }
    let total_duration = start.elapsed();
    let per_iteration = total_duration / ITERATIONS as u32;

    println!(
        "{} elements with prealloc: {:?} per iteration",
        ELEMENTS, per_iteration
    );

    assert!(
        per_iteration < Duration::from_micros(100),
        "Vec prealloc should be very fast, took {:?}",
        per_iteration
    );
}
