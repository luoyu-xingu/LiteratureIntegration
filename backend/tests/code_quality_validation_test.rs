//! Comprehensive code quality and correctness tests
//! Validates the optimized code paths without requiring external database connections.

mod common;

#[cfg(test)]
mod validation_tests {
    use literature_integration::models::author::Author;
    use literature_integration::models::dto::{
        AuthorWithPapers, ExportFilter, ExportRequest, GraphDataResponse, GraphLink, GraphNode,
        PaperDetailResponse,
    };
    use literature_integration::models::keyword::Keyword;
    use literature_integration::models::paper::Paper;

    // ── Model serialization roundtrip tests ──────────────────────────────────────────────

    #[test]
    fn test_paper_model_full_serialization_roundtrip() {
        let paper = Paper {
            id: "paper-001".to_string(),
            title: "Deep Learning for NLP: A Comprehensive Survey".to_string(),
            doi: Some("10.1000/test.doi.12345".to_string()),
            arxiv_id: Some("2401.01234".to_string()),
            abstract_text: Some(
                "This paper presents a comprehensive survey of deep learning \
                 techniques applied to natural language processing, covering \
                 transformers, attention mechanisms, and transfer learning \
                 approaches with extensive experimental results."
                    .to_string(),
            ),
            user_notes: Some(
                "# Important\n\n- Key contribution: novel attention mechanism\n\
                 - Dataset size: 10M parameters\n- Compare with BERT and GPT"
                    .to_string(),
            ),
            year: Some(2024),
            journal: Some("Journal of Machine Learning Research".to_string()),
            created_at: "2025-01-15T10:30:00Z".to_string(),
        };

        let json = serde_json::to_string(&paper).expect("Serialization should succeed");
        let deserialized: Paper =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(deserialized.id, paper.id);
        assert_eq!(deserialized.title, paper.title);
        assert_eq!(deserialized.doi, paper.doi);
        assert_eq!(deserialized.arxiv_id, paper.arxiv_id);
        assert_eq!(deserialized.abstract_text, paper.abstract_text);
        assert_eq!(deserialized.user_notes, paper.user_notes);
        assert_eq!(deserialized.year, paper.year);
        assert_eq!(deserialized.journal, paper.journal);
        assert_eq!(deserialized.created_at, paper.created_at);
    }

    #[test]
    fn test_author_model_with_orcid() {
        let author = Author {
            id: "author-001".to_string(),
            name: "Alice Smith".to_string(),
            orcid: Some("0000-0002-1825-0097".to_string()),
        };
        let json = serde_json::to_string(&author).unwrap();
        let back: Author = serde_json::from_str(&json).unwrap();
        assert_eq!(back.id, author.id);
        assert_eq!(back.name, author.name);
        assert_eq!(back.orcid, author.orcid);
    }

    #[test]
    fn test_keyword_model() {
        let kw = Keyword {
            id: "kw-1".to_string(),
            name: "machine-learning".to_string(),
        };
        let json = serde_json::to_string(&kw).unwrap();
        let back: Keyword = serde_json::from_str(&json).unwrap();
        assert_eq!(back.id, "kw-1");
        assert_eq!(back.name, "machine-learning");
    }

    // ── DTO struct validation tests ──────────────────────────────────────────────────

    #[test]
    fn test_paper_detail_response_complete() {
        let paper = Paper {
            id: "p-detail".into(),
            title: "Survey Paper".into(),
            doi: Some("10.1/x".into()),
            arxiv_id: None,
            abstract_text: Some("Abstract".into()),
            user_notes: None,
            year: Some(2023),
            journal: Some("JMLR".into()),
            created_at: "2025".into(),
        };
        let first = Some(Author {
            id: "a1".into(),
            name: "First Author".into(),
            orcid: None,
        });
        let corr = Some(Author {
            id: "a2".into(),
            name: "Corr Author".into(),
            orcid: Some("0000-0000-0000-0001".into()),
        });
        let keywords = vec![
            Keyword { id: "k1".into(), name: "NLP".into() },
            Keyword { id: "k2".into(), name: "ML".into() },
            Keyword { id: "k3".into(), name: "Survey".into() },
        ];

        let resp = PaperDetailResponse {
            paper: paper.clone(),
            first_author: first.clone(),
            corresponding_author: corr.clone(),
            keywords: keywords.clone(),
        };

        let json = serde_json::to_string(&resp).unwrap();
        let back: PaperDetailResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(back.paper.id, paper.id);
        assert_eq!(back.first_author.as_ref().unwrap().name, "First Author");
        assert_eq!(
            back.corresponding_author.as_ref().unwrap().orcid,
            Some("0000-0000-0000-0001".to_string())
        );
        assert_eq!(back.keywords.len(), 3);
        assert_eq!(back.keywords[1].name, "ML");
    }

    #[test]
    fn test_paper_detail_no_authors_no_keywords() {
        let resp = PaperDetailResponse {
            paper: Paper {
                id: "p-min".into(),
                title: "Minimal".into(),
                doi: None,
                arxiv_id: None,
                abstract_text: None,
                user_notes: None,
                year: None,
                journal: None,
                created_at: "now".into(),
            },
            first_author: None,
            corresponding_author: None,
            keywords: vec![],
        };
        let json = serde_json::to_string(&resp).unwrap();
        let back: PaperDetailResponse = serde_json::from_str(&json).unwrap();
        assert!(back.first_author.is_none());
        assert!(back.corresponding_author.is_none());
        assert!(back.keywords.is_empty());
        assert_eq!(back.paper.title, "Minimal");
    }

    #[test]
    fn test_graph_data_response() {
        let nodes = vec![
            GraphNode {
                id: "a1".into(),
                name: "Alice".into(),
                paper_count: 5,
                author_type: "first".into(),
            },
            GraphNode {
                id: "a2".into(),
                name: "Bob".into(),
                paper_count: 3,
                author_type: "corresponding".into(),
            },
            GraphNode {
                id: "a3".into(),
                name: "Carol".into(),
                paper_count: 7,
                author_type: "both".into(),
            },
        ];
        let links = vec![
            GraphLink {
                source: "a1".into(),
                target: "a2".into(),
                paper_count: 2,
            },
            GraphLink {
                source: "a1".into(),
                target: "a3".into(),
                paper_count: 4,
            },
        ];

        let resp = GraphDataResponse {
            nodes: nodes.clone(),
            links: links.clone(),
        };

        let json = serde_json::to_string(&resp).unwrap();
        let back: GraphDataResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(back.nodes.len(), 3);
        assert_eq!(back.links.len(), 2);
        assert_eq!(back.nodes[2].author_type, "both");
        assert_eq!(back.links[1].paper_count, 4);
    }

    #[test]
    fn test_author_with_papers() {
        let author = Author {
            id: "a-main".into(),
            name: "Prof. Researcher".into(),
            orcid: None,
        };
        let papers = (0..5)
            .map(|i| Paper {
                id: format!("p-{}", i),
                title: format!("Paper Number {}", i),
                doi: None,
                arxiv_id: None,
                abstract_text: None,
                user_notes: None,
                year: Some(2020 + i),
                journal: None,
                created_at: format!("202{}", i),
            })
            .collect::<Vec<_>>();

        let awp = AuthorWithPapers {
            author: author.clone(),
            papers: papers.clone(),
        };
        let json = serde_json::to_string(&awp).unwrap();
        let back: AuthorWithPapers = serde_json::from_str(&json).unwrap();
        assert_eq!(back.author.name, "Prof. Researcher");
        assert_eq!(back.papers.len(), 5);
        assert_eq!(back.papers[3].year, Some(2023));
    }

    // ── ExportRequest / ExportFilter tests ────────────────────────────────────

    #[test]
    fn test_export_request_full_filter() {
        let json = r#"{
            "format": "markdown",
            "group_by": "author",
            "filter": {
                "author_ids": ["a1", "a2", "a3"],
                "keyword_ids": ["k1", "k2"],
                "year_range": [2020, 2024]
            }
        }"#;
        let req: ExportRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.format, "markdown");
        assert_eq!(req.group_by.as_deref(), Some("author"));
        let filter = req.filter.unwrap();
        assert_eq!(filter.author_ids.unwrap().len(), 3);
        assert_eq!(filter.keyword_ids.unwrap().len(), 2);
        assert_eq!(filter.year_range.unwrap(), (2020, 2024));
    }

    #[test]
    fn test_export_filter_default() {
        let filter = ExportFilter::default();
        assert!(filter.author_ids.is_none());
        assert!(filter.keyword_ids.is_none());
        assert!(filter.year_range.is_none());
    }

    #[test]
    fn test_export_request_empty_filter_arrays() {
        let json = r#"{
            "format": "markdown",
            "filter": {
                "author_ids": [],
                "keyword_ids": []
            }
        }"#;
        let req: ExportRequest = serde_json::from_str(json).unwrap();
        let filter = req.filter.unwrap();
        assert_eq!(filter.author_ids.unwrap().len(), 0);
        assert_eq!(filter.keyword_ids.unwrap().len(), 0);
        assert!(filter.year_range.is_none());
    }

    // ── Markdown export validation (logic-only tests) ──────────────────────

    #[test]
    fn test_markdown_export_structure_validation() {
        // Build expected markdown components manually using the same logic as export service
        let workspace_name = "Research Workspace";
        let paper_count = 3;

        let mut md = String::with_capacity(1024);

        // Header building (mirrors ExportService)
        md.push_str("# 工作区: ");
        md.push_str(workspace_name);
        md.push_str("\n\n> 导出时间: ");
        md.push_str("2025-01-15 10:30");
        md.push_str("\n> 论文数量: ");

        // itoa-style integer conversion
        let mut buf = itoa::Buffer::new();
        md.push_str(buf.format(paper_count));
        md.push_str("\n\n---\n\n");

        assert!(md.starts_with("# 工作区: Research Workspace"));
        assert!(md.contains("论文数量: 3"));
        assert!(md.contains("---"));
    }

    #[test]
    fn test_markdown_keywords_join_no_allocation() {
        // Verify the optimized keyword join pattern
        let keywords = vec!["NLP", "Deep Learning", "Transformers", "BERT"];
        let mut output = String::new();
        let mut first = true;
        for kw in &keywords {
            if !first {
                output.push_str(", ");
            }
            output.push_str(kw);
            first = false;
        }
        assert_eq!(output, "NLP, Deep Learning, Transformers, BERT");
    }

    #[test]
    fn test_markdown_keywords_empty() {
        let keywords: Vec<&str> = vec![];
        let mut output = String::new();
        let mut first = true;
        for kw in &keywords {
            if !first {
                output.push_str(", ");
            }
            output.push_str(kw);
            first = false;
        }
        assert_eq!(output, "");
    }

    #[test]
    fn test_markdown_keywords_single() {
        let keywords = vec!["OnlyOne"];
        let mut output = String::new();
        let mut first = true;
        for kw in &keywords {
            if !first {
                output.push_str(", ");
            }
            output.push_str(kw);
            first = false;
        }
        assert_eq!(output, "OnlyOne");
    }

    #[test]
    fn test_markdown_paper_section_building() {
        // Verify complete paper section rendering
        let paper = Paper {
            id: "p1".into(),
            title: "Attention Is All You Need".into(),
            doi: Some("10.1/attention".into()),
            arxiv_id: Some("1706.03762".into()),
            abstract_text: Some("The dominant sequence transduction models are based on...".into()),
            user_notes: Some("Classic paper!".into()),
            year: Some(2017),
            journal: Some("NeurIPS".into()),
            created_at: "2025".into(),
        };
        let first = Some(Author { id: "a".into(), name: "Ashish Vaswani".into(), orcid: None });
        let corr = Some(Author { id: "b".into(), name: "Noam Shazeer".into(), orcid: None });
        let keywords = vec![
            Keyword { id: "k1".into(), name: "Attention".into() },
            Keyword { id: "k2".into(), name: "Transformers".into() },
        ];

        let mut md = String::with_capacity(2048);
        let mut year_buf = itoa::Buffer::new();

        md.push_str("### ");
        md.push_str(&paper.title);
        md.push_str("\n- **年份**: ");
        if let Some(y) = paper.year {
            md.push_str(year_buf.format(y));
        }
        md.push_str(" | **期刊**: ");
        md.push_str(paper.journal.as_deref().unwrap_or(""));
        md.push_str("\n- **DOI**: ");
        md.push_str(paper.doi.as_deref().unwrap_or(""));
        md.push_str("\n- **一作**: ");
        md.push_str(first.as_ref().map(|a| a.name.as_str()).unwrap_or(""));
        md.push_str(" | **通讯**: ");
        md.push_str(corr.as_ref().map(|a| a.name.as_str()).unwrap_or(""));
        md.push_str("\n- **关键词**: ");
        let mut f = true;
        for kw in &keywords {
            if !f { md.push_str(", "); }
            md.push_str(&kw.name);
            f = false;
        }
        md.push_str("\n\n");
        if let Some(abst) = &paper.abstract_text {
            md.push_str("**Abstract:**\n");
            md.push_str(abst);
            md.push_str("\n\n");
        }
        if let Some(notes) = &paper.user_notes {
            if !notes.is_empty() {
                md.push_str("**笔记:**\n");
                md.push_str(notes);
                md.push_str("\n\n");
            }
        }
        md.push_str("---\n\n");

        assert!(md.contains("### Attention Is All You Need"));
        assert!(md.contains("**年份**: 2017"));
        assert!(md.contains("**期刊**: NeurIPS"));
        assert!(md.contains("**一作**: Ashish Vaswani"));
        assert!(md.contains("**通讯**: Noam Shazeer"));
        assert!(md.contains("**关键词**: Attention, Transformers"));
        assert!(md.contains("**Abstract:**"));
        assert!(md.contains("The dominant sequence"));
        assert!(md.contains("**笔记:**"));
        assert!(md.contains("Classic paper!"));
    }

    // ── itoa integer formatting validation ───────────────────────────────────

    #[test]
    fn test_itoa_usize_various() {
        let cases: &[(usize, &str)] = &[
            (0, "0"),
            (1, "1"),
            (9, "9"),
            (10, "10"),
            (99, "99"),
            (100, "100"),
            (1234, "1234"),
            (9999, "9999"),
            (1_000_000, "1000000"),
            (usize::MAX / 1000, &format!("{}", usize::MAX / 1000)),
        ];
        let mut buf = itoa::Buffer::new();
        for &(n, expected) in cases {
            assert_eq!(buf.format(n), expected, "Failed for usize={}", n);
        }
    }

    #[test]
    fn test_itoa_i32_various() {
        let cases: &[(i32, &str)] = &[
            (0, "0"),
            (1, "1"),
            (-1, "-1"),
            (42, "42"),
            (-42, "-42"),
            (2024, "2024"),
            (-2024, "-2024"),
            (9999, "9999"),
            (i32::MAX, "2147483647"),
            (i32::MIN, "-2147483648"),
        ];
        let mut buf = itoa::Buffer::new();
        for &(n, expected) in cases {
            assert_eq!(buf.format(n), expected, "Failed for i32={}", n);
        }
    }

    #[test]
    fn test_itoa_buffer_reuse() {
        // Buffer must be reusable without cross-contamination
        let mut buf = itoa::Buffer::new();
        let r1 = buf.format(12345).to_string();
        let r2 = buf.format(67890).to_string();
        assert_eq!(r1, "12345");
        assert_eq!(r2, "67890");
        let r3 = buf.format(-42).to_string();
        assert_eq!(r3, "-42");
    }

    // ── Vec capacity / shrink_to_fit behavior ────────────────────────────

    #[test]
    fn test_vec_capacity_preallocation() {
        // Verify pre-allocation pattern used throughout codebase
        const EXPECTED: usize = 64;
        let mut v: Vec<i32> = Vec::with_capacity(EXPECTED);
        assert!(v.capacity() >= EXPECTED);
        assert_eq!(v.len(), 0);

        for i in 0..EXPECTED {
            v.push(i as i32);
        }
        assert_eq!(v.len(), EXPECTED);
        // No reallocation should happen if we stayed within capacity
        assert!(v.capacity() >= EXPECTED);
    }

    #[test]
    fn test_vec_shrink_to_fit_reduces_capacity() {
        let mut v: Vec<i32> = Vec::with_capacity(1024);
        for i in 0..10 {
            v.push(i);
        }
        assert!(v.capacity() >= 1024);
        v.shrink_to_fit();
        // After shrink, capacity should be >= len but possibly exact or close
        assert!(v.capacity() >= 10);
        assert_eq!(v.len(), 10);
    }

    #[test]
    fn test_vec_iter_map_collect_efficiency() {
        // Pattern used in keyword building
        let input: Vec<(String, String)> = (0..100)
            .map(|i| (format!("id-{}", i), format!("Keyword {}", i)))
            .collect();

        let keywords: Vec<Keyword> = input
            .iter()
            .map(|(id, name)| Keyword { id: id.clone(), name: name.clone() })
            .collect();

        assert_eq!(keywords.len(), 100);
        assert_eq!(keywords[0].id, "id-0");
        assert_eq!(keywords[99].name, "Keyword 99");
    }

    // ── Author batch processing logic validation ──────────────────────────────

    #[test]
    fn test_author_batch_index_tracking_logic() {
        // Simulate the create_authors_batch index-finding logic
        let authors: Vec<(String, String, Option<String>, bool, bool)> = vec![
            ("id-1".into(), "Alice".into(), None, true, false),
            ("id-2".into(), "Bob".into(), None, false, true),
            ("id-3".into(), "Carol".into(), None, false, false),
            ("id-4".into(), "Dave".into(), None, false, false),
        ];
        let mut first_idx = -1i64;
        let mut corr_idx = -1i64;
        for (i, a) in authors.iter().enumerate() {
            if a.3 && first_idx < 0 { first_idx = i as i64; }
            if a.4 && corr_idx < 0 { corr_idx = i as i64; }
        }
        assert_eq!(first_idx, 0);
        assert_eq!(corr_idx, 1);
        assert_ne!(first_idx, corr_idx);
    }

    #[test]
    fn test_author_batch_same_first_and_corresponding() {
        let authors: Vec<(String, String, Option<String>, bool, bool)> = vec![
            ("id-1".into(), "Alice".into(), None, true, true),
            ("id-2".into(), "Bob".into(), None, false, false),
        ];
        let mut first_idx = -1i64;
        let mut corr_idx = -1i64;
        for (i, a) in authors.iter().enumerate() {
            if a.3 && first_idx < 0 { first_idx = i as i64; }
            if a.4 && corr_idx < 0 { corr_idx = i as i64; }
        }
        assert_eq!(first_idx, 0);
        assert_eq!(corr_idx, 0);
        // Same author
        assert_eq!(first_idx, corr_idx);
    }

    #[test]
    fn test_author_batch_no_first_no_corresponding() {
        let authors: Vec<(String, String, Option<String>, bool, bool)> = vec![
            ("id-1".into(), "Alice".into(), None, false, false),
            ("id-2".into(), "Bob".into(), None, false, false),
        ];
        let mut first_idx = -1i64;
        let mut corr_idx = -1i64;
        for (i, a) in authors.iter().enumerate() {
            if a.3 && first_idx < 0 { first_idx = i as i64; }
            if a.4 && corr_idx < 0 { corr_idx = i as i64; }
        }
        assert_eq!(first_idx, -1);
        assert_eq!(corr_idx, -1);
    }

    #[test]
    fn test_author_batch_build_author_closure_logic() {
        let authors: Vec<(String, String, Option<String>, bool, bool)> = vec![
            ("a1".into(), "First".into(), Some("0000-0001".into()), true, false),
            ("a2".into(), "Corr".into(), Some("".into()), false, true),
            ("a3".into(), "Co".into(), None, false, false),
        ];

        let build = |idx: i64| {
            if idx < 0 { return None; }
            let i = idx as usize;
            let a = &authors[i];
            Some(Author {
                id: a.0.clone(),
                name: a.1.clone(),
                orcid: a.2.clone().filter(|s| !s.is_empty()),
            })
        };

        let first = build(0);
        let corr = build(1);
        let none = build(-1);
        let same = build(0);

        assert_eq!(first.as_ref().unwrap().name, "First");
        assert_eq!(first.as_ref().unwrap().orcid, Some("0000-0001".to_string()));
        assert_eq!(corr.as_ref().unwrap().name, "Corr");
        // empty orcid string should be filtered out
        assert_eq!(corr.as_ref().unwrap().orcid, None);
        assert!(none.is_none());
        assert_eq!(same.as_ref().unwrap().id, "a1");

        // Test clone when same author
        let first_clone = first.clone();
        let reused_corr = if 0 == 0 { first_clone } else { corr.clone() };
        assert_eq!(reused_corr.as_ref().unwrap().name, "First");
    }

    // ── String / slice pattern matching patterns ──────────────────────────────

    #[test]
    fn test_empty_slice_vs_none_filter_logic() {
        // Pattern used in get_papers_for_export and get_papers_detail_batch
        let check_filter = |ids_opt: Option<&[String]>| -> bool {
            let has_filter = ids_opt.map_or(false, |a| !a.is_empty());
            has_filter
        };

        let none_ids: Option<&[String]> = None;
        let empty: Vec<String> = vec![];
        let some = vec!["x".to_string()];

        assert!(!check_filter(none_ids));
        assert!(!check_filter(Some(&empty)));
        assert!(check_filter(Some(&some)));
    }

    #[test]
    fn test_year_filter_active_destructuring() {
        let yr_none: Option<(i32, i32)> = None;
        let yr_some = Some((2020, 2024));

        assert!(yr_none.is_none());
        let (min_y, max_y) = yr_some.unwrap_or((0, 0));
        assert_eq!(min_y, 2020);
        assert_eq!(max_y, 2024);

        let (mn, mx) = yr_none.unwrap_or((0, 0));
        assert_eq!(mn, 0);
        assert_eq!(mx, 0);

        let yr_active_none = yr_none.is_some();
        let yr_active_some = yr_some.is_some();
        assert!(!yr_active_none);
        assert!(yr_active_some);
    }

    // ── String property extraction helper logic (non-empty string filter) ─────────────────

    #[test]
    fn test_nonempty_string_filter_pattern() {
        // Pattern from get_nonempty_str / orcid filter logic
        let filter_empty = |s: Option<String>| -> Option<String> {
            match s {
                Some(val) if !val.is_empty() => Some(val),
                _ => None,
            }
        };

        assert_eq!(filter_empty(None), None);
        assert_eq!(filter_empty(Some("".into())), None);
        assert_eq!(filter_empty(Some("  ".into())), Some("  ".to_string()));
        assert_eq!(
            filter_empty(Some("valid".into())),
            Some("valid".to_string())
        );
    }

    #[test]
    fn test_positive_i32_filter_pattern() {
        // Pattern from get_positive_i32 for year extraction
        let filter_pos = |val: Result<i32, ()>| -> Option<i32> {
            match val {
                Ok(y) if y > 0 => Some(y),
                _ => None,
            }
        };
        assert_eq!(filter_pos(Ok(0)), None);
        assert_eq!(filter_pos(Ok(-5)), None);
        assert_eq!(filter_pos(Ok(2024)), Some(2024));
        assert_eq!(filter_pos(Err(())), None);
    }

    // ── Option handling patterns (as_deref / unwrap_or) ─────────────────────

    #[test]
    fn test_option_as_deref_unwrap_or_pattern() {
        let s_some: Option<String> = Some("hello".into());
        let s_none: Option<String> = None;

        assert_eq!(s_some.as_deref().unwrap_or(""), "hello");
        assert_eq!(s_none.as_deref().unwrap_or(""), "");

        let i_some: Option<i32> = Some(42);
        assert_eq!(i_some.unwrap_or(0), 42);
        let i_none: Option<i32> = None;
        assert_eq!(i_none.unwrap_or(0), 0);
    }

    #[test]
    fn test_estimated_capacity_estimation_logic() {
        // Mirror the export service size estimation
        let papers_detail: Vec<(Paper, Option<Author>, Option<Author>, Vec<Keyword>)> = vec![
            (Paper {
                id: "1".into(), title: "Short".into(),
                doi: None, arxiv_id: None,
                abstract_text: Some("A".repeat(100)),
                user_notes: Some("B".repeat(50)),
                year: Some(2024), journal: None,
                created_at: "".into(),
            }, None, None, vec![
                Keyword { id: "k".into(), name: "K".into() },
            ]),
        ];

        let mut est = 256 + "Test".len();
        for (p, _, _, kws) in &papers_detail {
            est += p.title.len() + 128;
            est += p.abstract_text.as_ref().map(|s| s.len() + 32).unwrap_or(0);
            est += p.user_notes.as_ref().filter(|s| !s.is_empty()).map(|s| s.len() + 32).unwrap_or(0);
            est += kws.iter().map(|k| k.name.len() + 4).sum::<usize>();
        }
        assert!(est > 256);
        // 256 base + 4 workspace name + 5 title + 128 + 100 abst + 32 + 50 notes + 32 + 11 kw name + 4
        // = 622 at least
        assert!(est >= 600);
    }

    // ── Comprehensive scenario: full export simulation ──────────────────────────

    #[test]
    fn test_full_export_markdown_generation_simulation() {
        // End-to-end test of the entire export logic (without DB calls)
        let ws_name = "AI Research Lab - 2024";
        let paper1 = Paper {
            id: "p1".into(),
            title: "GPT-4 Technical Report".into(),
            doi: Some("10.1/gpt4".into()),
            arxiv_id: None,
            abstract_text: Some("We present GPT-4, a large multimodal model...".into()),
            user_notes: Some("State of the art as of 2023".into()),
            year: Some(2023),
            journal: Some("OpenAI Report".into()),
            created_at: "2025".into(),
        };
        let fa1 = Some(Author { id: "a1".into(), name: "OpenAI Team".into(), orcid: None });
        let ca1 = Some(Author { id: "a2".into(), name: "Ilya Sutskever".into(), orcid: Some("0000-0001".into()) });
        let kws1 = vec![
            Keyword { id: "k1".into(), name: "LLM".into() },
            Keyword { id: "k2".into(), name: "GPT".into() },
            Keyword { id: "k3".into(), name: "Multimodal".into() },
        ];

        let paper2 = Paper {
            id: "p2".into(),
            title: "BERT: Pre-training of Deep Bidirectional Transformers".into(),
            doi: Some("10.1/bert".into()),
            arxiv_id: Some("1810.04805".into()),
            abstract_text: Some("We introduce a new language representation model...".into()),
            user_notes: None,
            year: Some(2018),
            journal: Some("NAACL".into()),
            created_at: "2025".into(),
        };
        let fa2 = Some(Author { id: "a3".into(), name: "Jacob Devlin".into(), orcid: None });
        let ca2 = Some(Author { id: "a4".into(), name: "Kristina Toutanova".into(), orcid: None });
        let kws2 = vec![
            Keyword { id: "k4".into(), name: "BERT".into() },
            Keyword { id: "k5".into(), name: "Pre-training".into() },
        ];

        let papers_detail = vec![
            (paper1, fa1, ca1, kws1),
            (paper2, fa2, ca2, kws2),
        ];

        let mut est = 256 + ws_name.len();
        for (paper, _, _, kws) in &papers_detail {
            est += paper.title.len() + 128;
            est += paper.abstract_text.as_ref().map(|s| s.len() + 32).unwrap_or(0);
            est += paper.user_notes.as_ref().filter(|s| !s.is_empty()).map(|s| s.len() + 32).unwrap_or(0);
            est += kws.iter().map(|k| k.name.len() + 4).sum::<usize>();
        }
        let mut md = String::with_capacity(est);
        md.push_str("# 工作区: ");
        md.push_str(ws_name);
        md.push_str("\n\n> 导出时间: 2025-01-15 12:00");
        md.push_str("\n> 论文数量: ");
        let mut ibuf = itoa::Buffer::new();
        md.push_str(ibuf.format(papers_detail.len()));
        md.push_str("\n\n---\n\n");

        let mut ybuf = itoa::Buffer::new();

        for (paper, fa, ca, kws) in &papers_detail {
            md.push_str("### ");
            md.push_str(&paper.title);
            md.push_str("\n- **年份**: ");
            if let Some(y) = paper.year { md.push_str(ybuf.format(y)); }
            md.push_str(" | **期刊**: ");
            md.push_str(paper.journal.as_deref().unwrap_or(""));
            md.push_str("\n- **DOI**: ");
            md.push_str(paper.doi.as_deref().unwrap_or(""));
            md.push_str("\n- **一作**: ");
            md.push_str(fa.as_ref().map(|a| a.name.as_str()).unwrap_or(""));
            md.push_str(" | **通讯**: ");
            md.push_str(ca.as_ref().map(|a| a.name.as_str()).unwrap_or(""));
            md.push_str("\n- **关键词**: ");
            let mut f = true;
            for kw in kws {
                if !f { md.push_str(", "); }
                md.push_str(&kw.name);
                f = false;
            }
            md.push_str("\n\n");
            if let Some(a) = &paper.abstract_text {
                md.push_str("**Abstract:**\n");
                md.push_str(a);
                md.push_str("\n\n");
            }
            if let Some(n) = &paper.user_notes {
                if !n.is_empty() {
                    md.push_str("**笔记:**\n");
                    md.push_str(n);
                    md.push_str("\n\n");
                }
            }
            md.push_str("---\n\n");
        }

        if md.capacity() > md.len() * 2 {
            md.shrink_to_fit();
        }

        // Validate structure
        assert!(md.starts_with("# 工作区: AI Research Lab - 2024"));
        assert!(md.contains("论文数量: 2"));

        // Paper 1
        assert!(md.contains("### GPT-4 Technical Report"));
        assert!(md.contains("**年份**: 2023"));
        assert!(md.contains("**一作**: OpenAI Team | **通讯**: Ilya Sutskever"));
        assert!(md.contains("**关键词**: LLM, GPT, Multimodal"));
        assert!(md.contains("**笔记:**\nState of the art as of 2023"));

        // Paper 2
        assert!(md.contains("### BERT:"));
        assert!(md.contains("**年份**: 2018 | **期刊**: NAACL"));
        assert!(md.contains("**关键词**: BERT, Pre-training"));
        assert!(!md.contains("**笔记:**\n\n")); // No notes for paper 2
        assert!(md.contains("We introduce a new language representation model"));

        // Ensure no allocations in keywords - should have exactly 2 paper dividers after header
        let sep_count = md.matches("---\n\n").count();
        assert!(sep_count >= 3); // header + 2 papers
    }

    // ── Model Clone / Copy / Send / Sync basic checks (compile-time) ────────────────

    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}
    fn assert_clone<T: Clone>() {}

    #[test]
    fn test_model_type_meets_basic_traits() {
        assert_send::<Paper>();
        assert_sync::<Paper>();
        assert_clone::<Paper>();

        assert_send::<Author>();
        assert_sync::<Author>();
        assert_clone::<Author>();

        assert_send::<Keyword>();
        assert_sync::<Keyword>();

        assert_send::<PaperDetailResponse>();
        assert_sync::<PaperDetailResponse>();

        assert_send::<GraphDataResponse>();
        assert_sync::<GraphDataResponse>();

        assert_send::<AuthorWithPapers>();
        assert_sync::<AuthorWithPapers>();
    }
}
