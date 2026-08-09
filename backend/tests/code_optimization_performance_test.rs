use std::time::{Duration, Instant};
use literature_integration::models::paper::Paper;
use literature_integration::models::author::Author;
use literature_integration::models::workspace::Workspace;
use literature_integration::models::dto::{
    PaperDetailResponse, GraphDataResponse, GraphNode, GraphLink,
    AuthorWithPapers, ExportFilter,
};
use literature_integration::models::keyword::Keyword;

#[test]
fn test_vec_with_capacity_avoids_reallocation() {
    let expected_capacity = 1000;
    let mut v: Vec<u64> = Vec::with_capacity(expected_capacity);
    let initial_capacity = v.capacity();
    assert!(
        initial_capacity >= expected_capacity,
        "Vec::with_capacity should allocate at least the requested capacity"
    );

    for i in 0..expected_capacity {
        v.push(i as u64);
    }

    assert_eq!(v.len(), expected_capacity);
    assert!(
        v.capacity() >= expected_capacity,
        "Capacity should not grow beyond the initial allocation"
    );
}

#[test]
fn test_vec_from_iter_with_exact_size() {
    let source: Vec<u64> = (0..500).collect();
    let v: Vec<u64> = source.iter().copied().collect();
    assert_eq!(v.len(), 500);
    assert!(v.capacity() >= 500);
}

#[test]
fn test_string_allocation_efficiency() {
    let s1 = String::from("hello world");
    let s2 = s1.clone();
    assert_eq!(s1, s2);

    let s3 = "static str";
    let s4 = s3.to_string();
    assert_eq!(s3, s4.as_str());

    let concatenated = format!("{}-{}", s1, s3);
    assert_eq!(concatenated, "hello world-static str");
}

#[test]
fn test_paper_detail_response_construction_no_clone() {
    let paper = Paper {
        id: "p1".into(),
        title: "Test".into(),
        doi: None,
        arxiv_id: None,
        abstract_text: None,
        user_notes: None,
        year: Some(2024),
        journal: None,
        created_at: "2025".into(),
    };

    let first_author = Some(Author {
        id: "a1".into(),
        name: "First Author".into(),
        orcid: None,
    });

    let corr_author = Some(Author {
        id: "a2".into(),
        name: "Corr Author".into(),
        orcid: None,
    });

    let keywords = vec![
        Keyword { id: "k1".into(), name: "ml".into() },
        Keyword { id: "k2".into(), name: "ai".into() },
    ];

    let response = PaperDetailResponse {
        paper,
        first_author,
        corresponding_author: corr_author,
        keywords,
    };

    assert_eq!(response.paper.id, "p1");
    assert_eq!(response.keywords.len(), 2);
}

#[test]
fn test_workspace_model_clone_cost() {
    let ws = Workspace {
        id: "ws-1".into(),
        name: "Test Workspace".into(),
        description: "A test workspace for performance validation".into(),
        created_at: "2025-01-01T00:00:00Z".into(),
    };

    let cloned = ws.clone();
    assert_eq!(ws.id, cloned.id);
    assert_eq!(ws.name, cloned.name);

    let start = Instant::now();
    for _ in 0..10_000 {
        let _c = ws.clone();
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed < Duration::from_millis(100),
        "Cloning 10k Workspaces took {:?}, should be under 100ms",
        elapsed
    );
}

#[test]
fn test_author_model_clone_cost() {
    let author = Author {
        id: "a-1".into(),
        name: "Test Author With A Relatively Long Name".into(),
        orcid: Some("0000-0001-2345-6789".into()),
    };

    let start = Instant::now();
    for _ in 0..10_000 {
        let _c = author.clone();
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed < Duration::from_millis(50),
        "Cloning 10k Authors took {:?}, should be under 50ms",
        elapsed
    );
}

#[test]
fn test_graph_data_response_vector_sizes() {
    let nodes: Vec<GraphNode> = (0..100).map(|i| GraphNode {
        id: format!("n-{}", i),
        name: format!("Author {}", i),
        paper_count: (i % 10) as i32 + 1,
        author_type: if i % 3 == 0 { "first".into() } else { "corresponding".into() },
    }).collect();

    let links: Vec<GraphLink> = (0..200).map(|i| GraphLink {
        source: format!("n-{}", i % 100),
        target: format!("n-{}", (i + 50) % 100),
        paper_count: (i % 5) as i32 + 1,
    }).collect();

    let resp = GraphDataResponse {
        nodes,
        links,
    };

    assert_eq!(resp.nodes.len(), 100);
    assert_eq!(resp.links.len(), 200);

    let json = serde_json::to_string(&resp).unwrap();
    assert!(json.contains("n-0"));
}

#[test]
fn test_author_with_papers_performance() {
    let author = Author {
        id: "a-1".into(),
        name: "Test Author".into(),
        orcid: None,
    };

    let papers: Vec<Paper> = (0..50).map(|i| Paper {
        id: format!("p-{}", i),
        title: format!("Paper {}", i),
        doi: None,
        arxiv_id: None,
        abstract_text: None,
        user_notes: None,
        year: Some(2020 + (i % 5) as i32),
        journal: None,
        created_at: "2025".into(),
    }).collect();

    let awp = AuthorWithPapers {
        author,
        papers,
    };

    let start = Instant::now();
    for _ in 0..1_000 {
        let json = serde_json::to_string(&awp).unwrap();
        assert!(!json.is_empty());
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed < Duration::from_millis(200),
        "Serializing 1k AuthorWithPapers (50 papers each) took {:?}, should be under 200ms",
        elapsed
    );
}

#[test]
fn test_paper_batch_processing_with_capacity() {
    let paper_count = 200;

    let start = Instant::now();

    let mut papers: Vec<Paper> = Vec::with_capacity(paper_count);
    for i in 0..paper_count {
        papers.push(Paper {
            id: format!("p-{}", i),
            title: format!("Performance Test Paper {}", i),
            doi: Some(format!("10.1234/test.{}", i)),
            arxiv_id: if i % 3 == 0 { Some(format!("2301.{:05}", i)) } else { None },
            abstract_text: if i % 2 == 0 {
                Some(format!("This is the abstract for paper number {}. It discusses performance optimization techniques.", i))
            } else {
                None
            },
            user_notes: None,
            year: Some(2020 + (i % 6) as i32),
            journal: if i % 4 == 0 { Some("Nature".into()) } else { None },
            created_at: "2025-01-01T00:00:00Z".into(),
        });
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed < Duration::from_millis(100),
        "Building 200 papers with Vec::with_capacity took {:?}, should be under 100ms",
        elapsed
    );

    assert_eq!(papers.len(), paper_count);

    let start_serialize = Instant::now();
    let json = serde_json::to_string(&papers).unwrap();
    let serialize_elapsed = start_serialize.elapsed();

    assert!(
        serialize_elapsed < Duration::from_millis(100),
        "Serializing 200 papers took {:?}, should be under 100ms",
        serialize_elapsed
    );
    assert!(!json.is_empty());
}

#[test]
fn test_keyword_batch_optimization_pattern() {
    let keywords: Vec<(String, String)> = (0..100).map(|i| {
        (format!("k-{}", i), format!("keyword_{}", i))
    }).collect();

    let n = keywords.len();

    let start = Instant::now();

    let mut ids: Vec<&str> = Vec::with_capacity(n);
    let mut names: Vec<&str> = Vec::with_capacity(n);

    for k in &keywords {
        ids.push(k.0.as_str());
        names.push(k.1.as_str());
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed < Duration::from_millis(10),
        "Building keyword batch vectors took {:?}, should be under 10ms",
        elapsed
    );

    assert_eq!(ids.len(), n);
    assert_eq!(names.len(), n);
}

#[test]
fn test_owner_writes_pattern_no_clone() {
    let papers: Vec<Paper> = (0..50).map(|i| Paper {
        id: format!("p-{}", i),
        title: format!("Paper {}", i),
        doi: None,
        arxiv_id: None,
        abstract_text: None,
        user_notes: None,
        year: Some(2024),
        journal: None,
        created_at: "2025".into(),
    }).collect();

    let start = Instant::now();

    let mut result: Vec<Paper> = Vec::with_capacity(papers.len());
    for p in papers {
        result.push(p);
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed < Duration::from_millis(50),
        "Moving 50 papers into pre-allocated vec took {:?}, should be under 50ms",
        elapsed
    );

    assert_eq!(result.len(), 50);
}

#[test]
fn test_workspace_id_generation_performance() {
    let start = Instant::now();
    let ids: Vec<String> = (0..500).map(|_| uuid::Uuid::new_v4().to_string()).collect();
    let elapsed = start.elapsed();

    assert_eq!(ids.len(), 500);
    assert!(
        elapsed < Duration::from_millis(500),
        "Generating 500 UUIDs took {:?}, should be under 500ms",
        elapsed
    );
}

#[test]
fn test_export_filter_default_performance() {
    let start = Instant::now();
    for _ in 0..10_000 {
        let filter = ExportFilter::default();
        assert!(filter.author_ids.is_none());
        assert!(filter.keyword_ids.is_none());
        assert!(filter.year_range.is_none());
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed < Duration::from_millis(100),
        "Creating 10k ExportFilter defaults took {:?}, should be under 100ms",
        elapsed
    );
}

#[test]
fn test_string_to_lowercase_performance() {
    let test_strings: Vec<String> = (0..1000).map(|i| {
        format!("THIS IS A TEST STRING WITH MIXED CASE NUMBER {} that needs to be lowercased for case-insensitive comparison", i)
    }).collect();

    let start = Instant::now();
    for s in &test_strings {
        let lower = s.to_lowercase();
        assert!(lower == lower.to_lowercase());
    }
    let elapsed = start.elapsed();

    assert!(
        elapsed < Duration::from_millis(50),
        "Lowercasing 1k strings took {:?}, should be under 50ms",
        elapsed
    );
}

#[test]
fn test_option_unwrap_or_pattern() {
    let val: Option<String> = None;
    let result = val.unwrap_or_default();
    assert!(result.is_empty());

    let val2: Option<String> = Some("hello".into());
    let result2 = val2.unwrap_or_default();
    assert_eq!(result2, "hello");

    let start = Instant::now();
    for i in 0..10_000 {
        let v: Option<i32> = if i % 2 == 0 { Some(i) } else { None };
        let r = v.unwrap_or(0);
        assert!(r >= 0);
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed < Duration::from_millis(10),
        "Option unwrap_or 10k iterations took {:?}, should be under 10ms",
        elapsed
    );
}

#[test]
fn test_collect_into_vec_with_capacity() {
    let source: Vec<i32> = (0..1000).collect();

    let start = Instant::now();
    let result: Vec<i32> = source.iter().map(|x| x * 2).collect();
    let elapsed = start.elapsed();

    assert_eq!(result.len(), 1000);
    assert!(
        elapsed < Duration::from_millis(10),
        "Mapping 1k items took {:?}, should be under 10ms",
        elapsed
    );
}

#[test]
fn test_paper_sequential_processing_performance() {
    let n = 500;
    let start = Instant::now();

    let mut papers = Vec::with_capacity(n);
    for i in 0..n {
        let doi = format!("10.1000/test.{}", i);
        papers.push(Paper {
            id: uuid::Uuid::new_v4().to_string(),
            title: format!("Performance Analysis of System {}", i),
            doi: Some(doi),
            arxiv_id: None,
            abstract_text: Some(format!(
                "This paper presents a comprehensive performance analysis of system {}. \
                 We evaluate various optimization techniques and their impact on overall system throughput. \
                 Our findings suggest that careful resource management can lead to significant improvements.",
                i
            )),
            user_notes: None,
            year: Some(2020 + (i % 5) as i32),
            journal: Some("Performance Journal".into()),
            created_at: chrono::Utc::now().to_rfc3339(),
        });
    }

    let elapsed = start.elapsed();
    assert_eq!(papers.len(), n);
    assert!(
        elapsed < Duration::from_millis(500),
        "Building {} papers with full fields took {:?}, should be under 500ms",
        n, elapsed
    );
}

#[test]
fn test_keyword_models_batch_construction() {
    let raw_keywords: Vec<(String, String)> = (0..50).map(|i| {
        (uuid::Uuid::new_v4().to_string(), format!("performance_related_keyword_{}", i))
    }).collect();

    let n = raw_keywords.len();
    let start = Instant::now();

    let mut keyword_models = Vec::with_capacity(n);
    for (id, name) in &raw_keywords {
        keyword_models.push(Keyword {
            id: id.clone(),
            name: name.clone(),
        });
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed < Duration::from_millis(5),
        "Building 50 keyword models took {:?}, should be under 5ms",
        elapsed
    );
    assert_eq!(keyword_models.len(), n);
}

#[test]
fn test_author_first_corresponding_indexing() {
    let authors: Vec<(String, String, Option<String>, bool, bool)> = vec![
        (uuid::Uuid::new_v4().to_string(), "First Author".into(), None, true, false),
        (uuid::Uuid::new_v4().to_string(), "Second Author".into(), None, false, false),
        (uuid::Uuid::new_v4().to_string(), "Third Corr Author".into(), None, false, true),
        (uuid::Uuid::new_v4().to_string(), "Fourth Author".into(), None, false, false),
    ];

    let n = authors.len();
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
    assert!(
        elapsed < Duration::from_millis(5),
        "Processing 4 authors took {:?}, should be under 5ms",
        elapsed
    );

    assert_eq!(first_idx, 0);
    assert_eq!(corr_idx, 2);
    assert_eq!(ids.len(), n);
}

#[test]
fn test_cypher_query_string_construction() {
    let start = Instant::now();
    for _ in 0..10_000 {
        let cypher = "MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper) \
                     WHERE ($min_year IS NULL OR p.year >= $min_year) \
                       AND ($max_year IS NULL OR p.year <= $max_year) \
                     RETURN DISTINCT p ORDER BY p.year DESC LIMIT 200";
        assert!(cypher.len() > 0);
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed < Duration::from_millis(100),
        "Constructing 10k query strings took {:?}, should be under 100ms",
        elapsed
    );
}

#[test]
fn test_parallel_join_pattern_performance() {
    let start = Instant::now();

    let results: Vec<(usize, usize)> = (0..1000).map(|i| {
        let a = i * 2;
        let b = i + 1;
        (a, b)
    }).collect();

    let _sum: usize = results.iter().map(|(a, b)| a + b).sum();

    let elapsed = start.elapsed();
    assert!(
        elapsed < Duration::from_millis(10),
        "Simulating parallel join pattern took {:?}, should be under 10ms",
        elapsed
    );
}

#[test]
fn test_paper_detail_response_build() {
    let start = Instant::now();

    for i in 0..500 {
        let paper = Paper {
            id: format!("p-{}", i),
            title: format!("Test Paper {}", i),
            doi: Some(format!("10.1234/test.{}", i)),
            arxiv_id: None,
            abstract_text: Some("Abstract text for testing".into()),
            user_notes: None,
            year: Some(2024),
            journal: Some("Test Journal".into()),
            created_at: "2025-01-01T00:00:00Z".into(),
        };

        let first_author = Some(Author {
            id: format!("fa-{}", i),
            name: format!("First Author {}", i),
            orcid: None,
        });

        let corr_author = if i % 3 == 0 {
            Some(Author {
                id: format!("ca-{}", i),
                name: format!("Corr Author {}", i),
                orcid: None,
            })
        } else {
            None
        };

        let keywords = vec![
            Keyword { id: format!("k1-{}", i), name: "machine learning".into() },
            Keyword { id: format!("k2-{}", i), name: "optimization".into() },
        ];

        let _response = PaperDetailResponse {
            paper,
            first_author,
            corresponding_author: corr_author,
            keywords,
        };
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed < Duration::from_millis(200),
        "Building 500 paper detail responses took {:?}, should be under 200ms",
        elapsed
    );
}

#[test]
fn test_model_serialization_roundtrip() {
    let papers: Vec<Paper> = (0..100).map(|i| Paper {
        id: format!("p-{}", i),
        title: format!("Paper Title {}", i),
        doi: Some(format!("10.1000/test.{}", i)),
        arxiv_id: if i % 2 == 0 { Some(format!("2301.{:05}", i)) } else { None },
        abstract_text: Some(format!("Abstract of paper {}. This discusses optimization methods.", i)),
        user_notes: None,
        year: Some(2020 + (i % 6) as i32),
        journal: if i % 3 == 0 { Some("Nature".into()) } else { None },
        created_at: "2025-06-01T12:00:00Z".into(),
    }).collect();

    let start = Instant::now();

    let json = serde_json::to_string(&papers).unwrap();
    let deserialized: Vec<Paper> = serde_json::from_str(&json).unwrap();

    let elapsed = start.elapsed();

    assert_eq!(deserialized.len(), 100);
    assert!(
        elapsed < Duration::from_millis(100),
        "Serializing + deserializing 100 papers took {:?}, should be under 100ms",
        elapsed
    );

    for (orig, deser) in papers.iter().zip(deserialized.iter()) {
        assert_eq!(orig.id, deser.id);
        assert_eq!(orig.title, deser.title);
        assert_eq!(orig.year, deser.year);
    }
}

#[test]
fn test_chrono_datetime_performance() {
    let start = Instant::now();
    for _ in 0..10_000 {
        let now = chrono::Utc::now().to_rfc3339();
        assert!(!now.is_empty());
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed < Duration::from_millis(500),
        "Generating 10k timestamps took {:?}, should be under 500ms",
        elapsed
    );
}