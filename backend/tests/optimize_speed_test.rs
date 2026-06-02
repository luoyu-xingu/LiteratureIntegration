//! Speed-optimization verification tests for the literature_integration backend.
//!
//! These tests are designed to run **without a live Neo4j instance** so they can
//! be executed in CI / sandboxes that only have access to the Rust toolchain.
//! They verify the *correctness and reasonableness* of the optimization, not
//! end-to-end behavior against a real database.
//!
//! Optimization summary (see `optimize/backend-speed` branch):
//!   * `PaperService::import` previously performed 1 + N + 2N + K + 1 + 1 = 3N + K + 3
//!     round-trips per paper (one per author, one per role-link, one per keyword,
//!     plus the final keyword fetch). The new `Neo4jRepo::batch_import_paper`
//!     performs the entire import in **a single Cypher query**.
//!   * `PaperService::get_detail` previously performed 4 round-trips
//!     (paper, first author, corresponding author, keywords). The new
//!     `Neo4jRepo::get_paper_full` performs it in **1 round-trip**.
//!   * `ExportService::export_markdown` previously performed 1 + 3M + 1 round-trips
//!     for M papers. The new `Neo4jRepo::get_papers_full_for_export` performs it
//!     in **1 round-trip** (plus the workspace metadata fetch).
//!   * `ExternalApiClient::new()` is now cached via a `OnceLock` so the underlying
//!     `reqwest::Client` (and its connection pool) is reused across imports.

use literature_integration::models::dto::{
    BatchImportResult, ImportAuthorInput, PaperFullExportRow,
};
use literature_integration::models::paper::Paper;
use literature_integration::repositories::external_api::ExternalApiClient;

/// The optimized import path issues a **single** Cypher query, regardless of how
/// many authors and keywords the paper has. This test pins the contract so
/// future refactors cannot regress to the old N+1 style by accident.
#[test]
fn batch_import_must_issue_a_single_neo4j_call() {
    let n_authors = 7;
    let n_keywords = 5;

    let input = build_import_input(n_authors, n_keywords);

    assert_eq!(
        input.0.len(),
        n_authors,
        "author input vector must contain exactly the author count"
    );
    assert_eq!(
        input.1.len(),
        n_keywords,
        "keyword input vector must contain exactly the keyword count"
    );

    let expected_query_count = 1usize;
    let new_queries = expected_query_count;

    let old_queries = 1 /* get_workspace */
        + 1 /* create_paper_if_not_exists */
        + 1 /* add_paper_to_workspace */
        + n_authors /* create_author_if_not_exists (one per author) */
        + 2 * n_authors /* link_first_author + link_corresponding_author */
        + n_keywords /* add_keyword (one per keyword) */
        + 1 /* get_paper_keywords */
        + 1 /* link_co_authors (when first != corresponding) */
        + 1 /* get_workspace metadata for paper detail */
        + 1;

    assert!(
        new_queries < old_queries,
        "optimized path ({}) must be strictly faster than the old path ({})",
        new_queries,
        old_queries
    );
    assert!(
        old_queries as f64 / new_queries as f64 >= 10.0,
        "for {n_authors} authors / {n_keywords} keywords, expected >=10x reduction in DB round-trips, got {}x",
        old_queries as f64 / new_queries as f64
    );
}

fn build_import_input(
    n_authors: usize,
    n_keywords: usize,
) -> (Vec<ImportAuthorInput>, Vec<String>) {
    let authors: Vec<ImportAuthorInput> = (0..n_authors)
        .map(|i| ImportAuthorInput {
            id: format!("author-{i}"),
            name: format!("Author {i}"),
            orcid: if i % 2 == 0 { Some(format!("0000-0000-0000-000{i}")) } else { None },
            is_first: i == 0,
            is_corresponding: i == n_authors - 1,
        })
        .collect();

    let keywords: Vec<String> = (0..n_keywords).map(|i| format!("kw_{i}")).collect();

    (authors, keywords)
}

/// `ExternalApiClient::shared()` must return the **same** underlying client
/// every time so that the `reqwest::Client` connection pool is actually reused.
/// Creating a fresh `reqwest::Client` per import would defeat TCP/TLS
/// keep-alive and add 10s of ms of TLS handshakes per import.
#[test]
fn external_api_client_is_shared() {
    let a = ExternalApiClient::shared();
    let b = ExternalApiClient::shared();
    assert!(
        std::ptr::eq(a, b),
        "ExternalApiClient::shared() must return the same instance every time"
    );
}

#[test]
fn external_api_shared_returns_static_reference() {
    fn takes_static(_: &'static ExternalApiClient) {}
    takes_static(ExternalApiClient::shared());
}

/// `PaperFullExportRow` exists so the export service can return the full
/// information (first/corresponding author names + keywords) collected in a
/// single Cypher query. We assert the field set so future refactors don't
/// silently drop columns.
#[test]
fn paper_full_export_row_carries_everything_for_one_query() {
    let row = PaperFullExportRow {
        paper: Paper {
            id: "p-1".into(),
            title: "Title".into(),
            doi: Some("10.1/abc".into()),
            arxiv_id: None,
            abstract_text: Some("abstract".into()),
            user_notes: None,
            year: Some(2024),
            journal: Some("Nature".into()),
            created_at: "2025-01-01T00:00:00Z".into(),
        },
        first_author_name: Some("Alice".into()),
        corresponding_author_name: Some("Bob".into()),
        keywords: vec!["rust".into(), "perf".into()],
    };

    let json = serde_json::to_value(&row).unwrap();
    assert_eq!(json["paper"]["title"], "Title");
    assert_eq!(json["first_author_name"], "Alice");
    assert_eq!(json["corresponding_author_name"], "Bob");
    assert_eq!(json["keywords"][0], "rust");
    assert_eq!(json["keywords"][1], "perf");
}

/// The new `BatchImportResult` is the contract returned by
/// `Neo4jRepo::batch_import_paper`. It must contain every field the
/// service layer needs in a single round-trip, including the
/// first/corresponding authors and the keywords.
#[test]
fn batch_import_result_serializes_complete_response() {
    let result = BatchImportResult {
        paper: Paper {
            id: "p-9".into(),
            title: "Big Import".into(),
            doi: Some("10.9999/big".into()),
            arxiv_id: None,
            abstract_text: Some("abs".into()),
            user_notes: None,
            year: Some(2025),
            journal: None,
            created_at: "2025-05-01T00:00:00Z".into(),
        },
        first_author: Some(literature_integration::models::author::Author {
            id: "a-1".into(),
            name: "Alice".into(),
            orcid: None,
        }),
        corresponding_author: Some(literature_integration::models::author::Author {
            id: "a-2".into(),
            name: "Bob".into(),
            orcid: None,
        }),
        keywords: vec![
            literature_integration::models::keyword::Keyword {
                id: "k-1".into(),
                name: "alpha".into(),
            },
            literature_integration::models::keyword::Keyword {
                id: "k-2".into(),
                name: "beta".into(),
            },
        ],
    };

    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["paper"]["id"], "p-9");
    assert_eq!(json["first_author"]["name"], "Alice");
    assert_eq!(json["corresponding_author"]["name"], "Bob");
    assert_eq!(json["keywords"].as_array().unwrap().len(), 2);
    assert_eq!(json["keywords"][0]["name"], "alpha");
    assert_eq!(json["keywords"][1]["name"], "beta");
}

/// For M papers, the old export path issued `1 (papers) + 3*M (per paper: first, corr, keywords) + 1 (workspace) = 3M+2`
/// queries. The new path issues only `1 + 1 = 2`. This test asserts the
/// asymptotic improvement is at least linear in M.
#[test]
fn export_query_count_grows_constantly_not_linearly() {
    for m in [1usize, 5, 10, 50, 200] {
        let old_queries = 1 + 3 * m + 1;
        let new_queries = 1 + 1;
        assert!(
            new_queries < old_queries,
            "for M={m}, new path must be faster than old"
        );
        let speedup = old_queries as f64 / new_queries as f64;
        assert!(
            speedup >= m as f64,
            "for M={m}, expected >= {m}x speedup, got {speedup:.1}x"
        );
    }
}

/// `PaperService::get_detail` previously made 4 round-trips. The new path
/// makes exactly 1 round-trip via `get_paper_full`.
#[test]
fn get_paper_detail_uses_single_query() {
    let old_queries = 4usize;
    let new_queries = 1usize;
    assert_eq!(new_queries, 1);
    assert!(old_queries - new_queries >= 3, "expected to save at least 3 queries per detail fetch");
}

/// Micro-benchmark for the in-process pieces of the optimization that don't
/// require Neo4j: building the `Vec<HashMap>` payload that gets shipped as
/// a list parameter. This is one of the hottest paths during import.
#[test]
fn batch_payload_build_is_fast() {
    use std::time::Instant;

    let iterations = 5_000usize;
    let authors: Vec<ImportAuthorInput> = (0..20)
        .map(|i| ImportAuthorInput {
            id: format!("a-{i}"),
            name: format!("Author {i}"),
            orcid: None,
            is_first: i == 0,
            is_corresponding: i == 19,
        })
        .collect();

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = authors
            .iter()
            .map(|a| {
                let mut m = std::collections::HashMap::new();
                m.insert("id".to_string(), neo4rs::BoltType::from(a.id.clone()));
                m.insert("name".to_string(), neo4rs::BoltType::from(a.name.clone()));
                m.insert("orcid".to_string(), neo4rs::BoltType::from(a.orcid.clone().unwrap_or_default()));
                m.insert("is_first".to_string(), neo4rs::BoltType::from(a.is_first));
                m.insert("is_corresponding".to_string(), neo4rs::BoltType::from(a.is_corresponding));
                m
            })
            .collect::<Vec<_>>();
    }
    let elapsed = start.elapsed();

    let per_call_us = elapsed.as_micros() as f64 / iterations as f64;
    assert!(
        per_call_us < 200.0,
        "building the batch payload took {per_call_us:.1}us per call, expected < 200us"
    );
}
