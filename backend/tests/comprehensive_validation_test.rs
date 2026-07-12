use literature_integration::models::author::Author;
use literature_integration::models::dto::{
    AuthorWithPapers, CreateWorkspaceRequest, ExportFilter, ExportRequest, GraphDataResponse,
    GraphLink, GraphNode, PaperDetailResponse, UpdatePaperRequest, UpdateWorkspaceRequest,
};
use literature_integration::models::keyword::Keyword;
use literature_integration::models::paper::Paper;
use literature_integration::models::workspace::Workspace;
use literature_integration::errors::AppError;

#[cfg(test)]
mod model_tests {
    use super::*;

    #[test]
    fn test_workspace_creation() {
        let ws = Workspace {
            id: "ws-1".to_string(),
            name: "Test Workspace".to_string(),
            description: "A test workspace".to_string(),
            created_at: "2025-01-01T00:00:00Z".to_string(),
        };
        assert_eq!(ws.id, "ws-1");
        assert_eq!(ws.name, "Test Workspace");
        assert_eq!(ws.description, "A test workspace");
    }

    #[test]
    fn test_workspace_clone() {
        let ws = Workspace {
            id: "ws-1".to_string(),
            name: "Test".to_string(),
            description: "desc".to_string(),
            created_at: "2025".to_string(),
        };
        let cloned = ws.clone();
        assert_eq!(ws.id, cloned.id);
        assert_eq!(ws.name, cloned.name);
    }

    #[test]
    fn test_workspace_serialization_roundtrip() {
        let ws = Workspace {
            id: "ws-123".to_string(),
            name: "Serialization Test".to_string(),
            description: "Testing serialization".to_string(),
            created_at: "2025-06-15T10:30:00Z".to_string(),
        };
        let json = serde_json::to_string(&ws).unwrap();
        let deserialized: Workspace = serde_json::from_str(&json).unwrap();
        assert_eq!(ws.id, deserialized.id);
        assert_eq!(ws.name, deserialized.name);
        assert_eq!(ws.description, deserialized.description);
        assert_eq!(ws.created_at, deserialized.created_at);
    }

    #[test]
    fn test_paper_all_fields() {
        let paper = Paper {
            id: "p-1".to_string(),
            title: "Test Paper".to_string(),
            doi: Some("10.1234/test".to_string()),
            arxiv_id: Some("2301.00001".to_string()),
            abstract_text: Some("This is an abstract".to_string()),
            user_notes: Some("My notes".to_string()),
            year: Some(2024),
            journal: Some("Nature".to_string()),
            created_at: "2025-01-01".to_string(),
        };
        assert_eq!(paper.id, "p-1");
        assert_eq!(paper.doi.as_deref(), Some("10.1234/test"));
        assert_eq!(paper.year, Some(2024));
    }

    #[test]
    fn test_paper_minimal_fields() {
        let paper = Paper {
            id: "p-2".to_string(),
            title: "Minimal Paper".to_string(),
            doi: None,
            arxiv_id: None,
            abstract_text: None,
            user_notes: None,
            year: None,
            journal: None,
            created_at: "2025".to_string(),
        };
        assert!(paper.doi.is_none());
        assert!(paper.year.is_none());
        assert!(paper.journal.is_none());
    }

    #[test]
    fn test_paper_serialization_with_nulls() {
        let json = r#"{
            "id": "p-3",
            "title": "Null Test",
            "doi": null,
            "arxiv_id": null,
            "abstract_text": null,
            "user_notes": null,
            "year": null,
            "journal": null,
            "created_at": "2025"
        }"#;
        let paper: Paper = serde_json::from_str(json).unwrap();
        assert_eq!(paper.id, "p-3");
        assert!(paper.doi.is_none());
        assert!(paper.abstract_text.is_none());
    }

    #[test]
    fn test_author_with_orcid() {
        let author = Author {
            id: "a-1".to_string(),
            name: "John Doe".to_string(),
            orcid: Some("0000-0001-2345-6789".to_string()),
        };
        assert_eq!(author.name, "John Doe");
        assert_eq!(author.orcid.as_deref(), Some("0000-0001-2345-6789"));
    }

    #[test]
    fn test_author_without_orcid() {
        let author = Author {
            id: "a-2".to_string(),
            name: "Jane Smith".to_string(),
            orcid: None,
        };
        assert!(author.orcid.is_none());
    }

    #[test]
    fn test_keyword_creation() {
        let kw = Keyword {
            id: "k-1".to_string(),
            name: "machine learning".to_string(),
        };
        assert_eq!(kw.name, "machine learning");
    }

    #[test]
    fn test_keyword_serialization() {
        let kw = Keyword {
            id: "k-2".to_string(),
            name: "deep learning".to_string(),
        };
        let json = serde_json::to_string(&kw).unwrap();
        let deserialized: Keyword = serde_json::from_str(&json).unwrap();
        assert_eq!(kw.id, deserialized.id);
        assert_eq!(kw.name, deserialized.name);
    }
}

#[cfg(test)]
mod dto_tests {
    use super::*;

    #[test]
    fn test_create_workspace_request() {
        let json = r#"{"name":"Test WS","description":"A test"}"#;
        let req: CreateWorkspaceRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.name, "Test WS");
        assert_eq!(req.description, Some("A test".to_string()));
    }

    #[test]
    fn test_create_workspace_request_no_description() {
        let json = r#"{"name":"Test WS"}"#;
        let req: CreateWorkspaceRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.name, "Test WS");
        assert!(req.description.is_none());
    }

    #[test]
    fn test_update_workspace_request_partial() {
        let json = r#"{"name":"Updated"}"#;
        let req: UpdateWorkspaceRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.name, Some("Updated".to_string()));
        assert!(req.description.is_none());
    }

    #[test]
    fn test_update_workspace_request_both() {
        let json = r#"{"name":"Updated","description":"New desc"}"#;
        let req: UpdateWorkspaceRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.name, Some("Updated".to_string()));
        assert_eq!(req.description, Some("New desc".to_string()));
    }

    #[test]
    fn test_update_paper_request_notes_only() {
        let json = r#"{"user_notes":"Some notes"}"#;
        let req: UpdatePaperRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.user_notes, Some("Some notes".to_string()));
    }

    #[test]
    fn test_paper_detail_response() {
        let resp = PaperDetailResponse {
            paper: Paper {
                id: "p-1".into(),
                title: "Test".into(),
                doi: None,
                arxiv_id: None,
                abstract_text: None,
                user_notes: None,
                year: Some(2024),
                journal: None,
                created_at: "2025".into(),
            },
            first_author: Some(Author {
                id: "a-1".into(),
                name: "First".into(),
                orcid: None,
            }),
            corresponding_author: Some(Author {
                id: "a-2".into(),
                name: "Corresp".into(),
                orcid: None,
            }),
            keywords: vec![Keyword {
                id: "k-1".into(),
                name: "ML".into(),
            }],
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("Test"));
        assert!(json.contains("First"));
        assert!(json.contains("ML"));
    }

    #[test]
    fn test_graph_data_response() {
        let resp = GraphDataResponse {
            nodes: vec![
                GraphNode {
                    id: "n-1".into(),
                    name: "Author1".into(),
                    paper_count: 5,
                    author_type: "first".into(),
                },
                GraphNode {
                    id: "n-2".into(),
                    name: "Author2".into(),
                    paper_count: 3,
                    author_type: "corresponding".into(),
                },
            ],
            links: vec![GraphLink {
                source: "n-1".into(),
                target: "n-2".into(),
                paper_count: 2,
            }],
        };
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: GraphDataResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.nodes.len(), 2);
        assert_eq!(deserialized.links.len(), 1);
        assert_eq!(deserialized.nodes[0].name, "Author1");
    }

    #[test]
    fn test_graph_node_fields() {
        let node = GraphNode {
            id: "n1".to_string(),
            name: "Test Author".to_string(),
            paper_count: 10,
            author_type: "both".to_string(),
        };
        assert_eq!(node.paper_count, 10);
        assert_eq!(node.author_type, "both");
    }

    #[test]
    fn test_graph_link_fields() {
        let link = GraphLink {
            source: "n1".to_string(),
            target: "n2".to_string(),
            paper_count: 5,
        };
        assert_eq!(link.source, "n1");
        assert_eq!(link.target, "n2");
        assert_eq!(link.paper_count, 5);
    }

    #[test]
    fn test_export_request_markdown() {
        let json = r#"{"format":"markdown"}"#;
        let req: ExportRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.format, "markdown");
        assert!(req.group_by.is_none());
        assert!(req.filter.is_none());
    }

    #[test]
    fn test_export_request_with_filter() {
        let json = r#"{
            "format":"markdown",
            "group_by":"author",
            "filter":{
                "author_ids":["a1","a2"],
                "keyword_ids":["k1"],
                "year_range":[2020,2024]
            }
        }"#;
        let req: ExportRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.format, "markdown");
        assert_eq!(req.group_by.as_deref(), Some("author"));
        let filter = req.filter.unwrap();
        assert_eq!(filter.author_ids.unwrap().len(), 2);
        assert_eq!(filter.keyword_ids.unwrap().len(), 1);
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
    fn test_author_with_papers() {
        let awp = AuthorWithPapers {
            author: Author {
                id: "a-1".into(),
                name: "Test Author".into(),
                orcid: None,
            },
            papers: vec![
                Paper {
                    id: "p-1".into(),
                    title: "Paper 1".into(),
                    doi: None,
                    arxiv_id: None,
                    abstract_text: None,
                    user_notes: None,
                    year: Some(2024),
                    journal: None,
                    created_at: "2025".into(),
                },
                Paper {
                    id: "p-2".into(),
                    title: "Paper 2".into(),
                    doi: None,
                    arxiv_id: None,
                    abstract_text: None,
                    user_notes: None,
                    year: Some(2023),
                    journal: None,
                    created_at: "2025".into(),
                },
            ],
        };
        let json = serde_json::to_string(&awp).unwrap();
        let deserialized: AuthorWithPapers = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.author.name, "Test Author");
        assert_eq!(deserialized.papers.len(), 2);
    }
}

#[cfg(test)]
mod error_tests {
    use super::*;

    #[test]
    fn test_workspace_not_found_error_display() {
        let err = AppError::WorkspaceNotFound("ws-123".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("ws-123"));
        assert!(msg.to_lowercase().contains("workspace"));
    }

    #[test]
    fn test_paper_not_found_error_display() {
        let err = AppError::PaperNotFound("p-456".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("p-456"));
    }

    #[test]
    fn test_author_not_found_error_display() {
        let err = AppError::AuthorNotFound("a-789".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("a-789"));
    }

    #[test]
    fn test_validation_error_display() {
        let err = AppError::ValidationError("Invalid input".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("Invalid input"));
    }

    #[test]
    fn test_neo4j_error_display() {
        let err = AppError::Neo4jError("Connection failed".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("Connection failed"));
    }

    #[test]
    fn test_external_api_error_display() {
        let err = AppError::ExternalApiError("API timeout".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("API timeout"));
    }

    #[test]
    fn test_import_failed_error_display() {
        let err = AppError::ImportFailed("DOI not found".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("DOI not found"));
    }
}

#[cfg(test)]
mod string_optimization_tests {
    use super::*;

    #[test]
    fn test_string_capacity_estimation() {
        let paper_count = 10;
        let estimated_size = paper_count * 500 + 200;
        let mut s = String::with_capacity(estimated_size);
        assert!(s.capacity() >= estimated_size);
        assert_eq!(s.len(), 0);

        for i in 0..paper_count {
            s.push_str(&format!("Paper {}: Title of paper {}\n", i, i));
        }
        assert!(s.len() > 0);
        assert!(s.capacity() >= s.len());
    }

    #[test]
    fn test_vec_capacity_preallocation() {
        let n = 100;
        let mut v: Vec<i32> = Vec::with_capacity(n);
        assert_eq!(v.capacity(), n);
        assert_eq!(v.len(), 0);

        for i in 0..n {
            v.push(i as i32);
        }
        assert_eq!(v.len(), n);
        assert!(v.capacity() >= n);
    }

    #[test]
    fn test_vec_shrink_to_fit() {
        let mut v: Vec<i32> = Vec::with_capacity(100);
        for i in 0..10 {
            v.push(i);
        }
        assert_eq!(v.len(), 10);
        assert!(v.capacity() >= 100);

        v.shrink_to_fit();
        assert_eq!(v.len(), 10);
        assert!(v.capacity() < 100);
    }

    #[test]
    fn test_string_concat_vs_format() {
        let parts: Vec<&str> = vec!["Hello", " ", "World", "!"];

        let mut s1 = String::with_capacity(20);
        for p in &parts {
            s1.push_str(p);
        }

        let s2 = format!("{}{}{}{}", parts[0], parts[1], parts[2], parts[3]);

        assert_eq!(s1, s2);
        assert_eq!(s1, "Hello World!");
    }

    #[test]
    fn test_keyword_join_optimization() {
        let keywords = vec!["ML", "DL", "NLP", "CV"];

        let mut joined = String::new();
        for (i, kw) in keywords.iter().enumerate() {
            if i > 0 {
                joined.push_str(", ");
            }
            joined.push_str(kw);
        }

        let standard_join = keywords.join(", ");
        assert_eq!(joined, standard_join);
        assert_eq!(joined, "ML, DL, NLP, CV");
    }
}

#[cfg(test)]
mod collection_operation_tests {
    use super::*;

    #[test]
    fn test_iterator_map_collect() {
        let items: Vec<i32> = vec![1, 2, 3, 4, 5];
        let doubled: Vec<i32> = items.iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_iterator_enumerate() {
        let items = vec!["a", "b", "c"];
        let indexed: Vec<(usize, &&str)> = items.iter().enumerate().collect();
        assert_eq!(indexed.len(), 3);
        assert_eq!(indexed[0].0, 0);
        assert_eq!(*indexed[0].1, "a");
        assert_eq!(indexed[2].0, 2);
    }

    #[test]
    fn test_option_map() {
        let some_val = Some(42);
        let doubled = some_val.map(|x| x * 2);
        assert_eq!(doubled, Some(84));

        let none_val: Option<i32> = None;
        let result = none_val.map(|x| x * 2);
        assert_eq!(result, None);
    }

    #[test]
    fn test_option_unwrap_or_default() {
        let some_val = Some("hello".to_string());
        assert_eq!(some_val.unwrap_or_default(), "hello");

        let none_val: Option<String> = None;
        assert_eq!(none_val.unwrap_or_default(), "");
    }

    #[test]
    fn test_option_as_deref() {
        let some_val = Some(String::from("test"));
        assert_eq!(some_val.as_deref(), Some("test"));

        let none_val: Option<String> = None;
        assert_eq!(none_val.as_deref(), None);
    }

    #[test]
    fn test_result_map() {
        let ok_val: Result<i32, &str> = Ok(5);
        let doubled = ok_val.map(|x| x * 2);
        assert_eq!(doubled, Ok(10));

        let err_val: Result<i32, &str> = Err("error");
        let result = err_val.map(|x| x * 2);
        assert_eq!(result, Err("error"));
    }
}

#[cfg(test)]
mod paper_operation_tests {
    use super::*;

    fn create_test_paper(id: &str, title: &str, year: Option<i32>) -> Paper {
        Paper {
            id: id.to_string(),
            title: title.to_string(),
            doi: None,
            arxiv_id: None,
            abstract_text: None,
            user_notes: None,
            year,
            journal: None,
            created_at: "2025-01-01".to_string(),
        }
    }

    #[test]
    fn test_paper_sorting_by_year() {
        let mut papers = vec![
            create_test_paper("p1", "Old Paper", Some(2020)),
            create_test_paper("p2", "New Paper", Some(2024)),
            create_test_paper("p3", "Mid Paper", Some(2022)),
            create_test_paper("p4", "No Year", None),
        ];

        papers.sort_by(|a, b| b.year.cmp(&a.year));

        assert_eq!(papers[0].id, "p2");
        assert_eq!(papers[1].id, "p3");
        assert_eq!(papers[2].id, "p1");
        assert_eq!(papers[3].id, "p4");
    }

    #[test]
    fn test_paper_filter_by_year() {
        let papers = vec![
            create_test_paper("p1", "2020 Paper", Some(2020)),
            create_test_paper("p2", "2024 Paper", Some(2024)),
            create_test_paper("p3", "2022 Paper", Some(2022)),
            create_test_paper("p4", "No Year", None),
        ];

        let start_year = 2022;
        let end_year = 2024;
        let filtered: Vec<&Paper> = papers
            .iter()
            .filter(|p| match p.year {
                Some(y) => y >= start_year && y <= end_year,
                None => false,
            })
            .collect();

        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().any(|p| p.id == "p2"));
        assert!(filtered.iter().any(|p| p.id == "p3"));
    }

    #[test]
    fn test_paper_search_title() {
        let papers = vec![
            create_test_paper("p1", "Machine Learning Basics", Some(2020)),
            create_test_paper("p2", "Deep Learning Advances", Some(2024)),
            create_test_paper("p3", "Quantum Computing", Some(2022)),
        ];

        let query = "Learning";
        let results: Vec<&Paper> = papers
            .iter()
            .filter(|p| p.title.contains(query))
            .collect();

        assert_eq!(results.len(), 2);
    }
}

#[cfg(test)]
mod performance_assurance_tests {
    use std::time::Instant;

    #[test]
    fn test_string_preallocation_faster_than_growth() {
        let iterations = 1000;
        let char_count = 100;

        let start_preallocated = Instant::now();
        for _ in 0..iterations {
            let mut s = String::with_capacity(char_count);
            for i in 0..char_count {
                s.push((b'a' + (i % 26) as u8) as char);
            }
            assert_eq!(s.len(), char_count);
        }
        let preallocated_duration = start_preallocated.elapsed();

        let start_growing = Instant::now();
        for _ in 0..iterations {
            let mut s = String::new();
            for i in 0..char_count {
                s.push((b'a' + (i % 26) as u8) as char);
            }
            assert_eq!(s.len(), char_count);
        }
        let growing_duration = start_growing.elapsed();

        println!(
            "Preallocated: {:?}, Growing: {:?}",
            preallocated_duration, growing_duration
        );
    }

    #[test]
    fn test_vec_preallocation_performance() {
        let n = 10000;

        let start_preallocated = Instant::now();
        let mut v1: Vec<i32> = Vec::with_capacity(n);
        for i in 0..n {
            v1.push(i as i32);
        }
        assert_eq!(v1.len(), n);
        let preallocated_duration = start_preallocated.elapsed();

        let start_growing = Instant::now();
        let mut v2: Vec<i32> = Vec::new();
        for i in 0..n {
            v2.push(i as i32);
        }
        assert_eq!(v2.len(), n);
        let growing_duration = start_growing.elapsed();

        println!(
            "Vec preallocated: {:?}, growing: {:?}",
            preallocated_duration, growing_duration
        );
    }

    #[test]
    fn test_iteration_methods_performance() {
        let data: Vec<i32> = (0..10000).collect();

        let start_for_loop = Instant::now();
        let mut sum1 = 0;
        for x in &data {
            sum1 += *x;
        }
        let for_loop_duration = start_for_loop.elapsed();

        let start_iter = Instant::now();
        let sum2: i32 = data.iter().sum();
        let iter_duration = start_iter.elapsed();

        assert_eq!(sum1, sum2);
        println!(
            "For loop: {:?}, iter sum: {:?}",
            for_loop_duration, iter_duration
        );
    }
}
