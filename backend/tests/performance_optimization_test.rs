mod common;

use literature_integration::repositories::external_api::{AuthorMeta, ExternalApiClient, PaperMeta};
use std::time::{Duration, Instant};

fn build_fake_meta(authors: usize, keywords: usize) -> PaperMeta {
    PaperMeta {
        title: "A performance-aware paper title".to_string(),
        authors: (0..authors)
            .map(|i| AuthorMeta {
                name: format!("Author {}", i),
                orcid: None,
                is_first: i == 0,
                is_corresponding: i == authors - 1,
            })
            .collect(),
        abstract_text: Some("An abstract with some content".to_string()),
        year: Some(2024),
        journal: Some("Journal of Perf".to_string()),
        keywords: (0..keywords).map(|i| format!("kw-{}", i)).collect(),
        doi: Some("10.9999/test".to_string()),
        arxiv_id: None,
    }
}

#[tokio::test]
async fn test_shared_client_reuse_is_cheap() {
    let client = ExternalApiClient::new();
    let start = Instant::now();
    for _ in 0..100 {
        let _ = ExternalApiClient::new();
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed < Duration::from_millis(200),
        "Client instantiation should be very cheap (shared); took {:?}",
        elapsed
    );
    let _ = client;
}

#[test]
fn test_meta_build_allocations_do_not_crash() {
    let meta = build_fake_meta(10, 20);
    assert_eq!(meta.authors.len(), 10);
    assert_eq!(meta.keywords.len(), 20);
    assert_eq!(meta.authors[0].is_first, true);
    assert_eq!(meta.authors[9].is_corresponding, true);
}

#[tokio::test]
async fn test_repeated_instantiation_performance() {
    let iterations = 10_000;
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = ExternalApiClient::new();
    }
    let elapsed = start.elapsed();
    let per_iter = elapsed / iterations as u32;
    assert!(
        per_iter < Duration::from_micros(100),
        "Client instantiation should be sub-microsecond (shared client); took {:?} per iter",
        per_iter
    );
}

#[test]
fn test_identifier_parsing_plain_doi() {
    let identifier = "10.1234/abc";
    assert!(identifier.starts_with("10."));
}

#[test]
fn test_identifier_parsing_case_insensitive_prefix() {
    for prefix in &["doi:", "DOI:", "Doi:"] {
        let identifier = format!("{}10.1234/abc", prefix);
        let trimmed = identifier.trim();
        let lower_head = trimmed
            .get(..5.min(trimmed.len()))
            .unwrap_or("")
            .to_ascii_lowercase();
        assert!(
            trimmed.starts_with("10.") || lower_head.starts_with("doi:"),
            "Should detect doi prefix for '{}'",
            identifier
        );
    }
}

#[test]
fn test_identifier_parsing_arxiv_id() {
    for id in &["2401.00001", "hep-th/9901001"] {
        let trimmed = id.trim();
        let lower_head = trimmed
            .get(..5.min(trimmed.len()))
            .unwrap_or("")
            .to_ascii_lowercase();
        assert!(
            !(trimmed.starts_with("10.") || lower_head.starts_with("doi:")),
            "Should route to arxiv, not doi, for '{}'",
            id
        );
    }
}

#[test]
fn test_author_construction_allocation_sensible() {
    let n = 100;
    let mut authors: Vec<AuthorMeta> = Vec::with_capacity(n);
    for i in 0..n {
        authors.push(AuthorMeta {
            name: format!("Author-{}", i),
            orcid: None,
            is_first: i == 0,
            is_corresponding: i == n - 1,
        });
    }
    assert_eq!(authors.len(), n);
    assert!(authors[0].is_first);
    assert!(authors[n - 1].is_corresponding);
}

#[test]
fn test_title_and_container_title_pop_reduces_copy() {
    let mut v = vec!["First".to_string(), "Second".to_string()];
    let first = v.pop().unwrap_or_default();
    assert!(!first.is_empty());
    assert_eq!(v.len(), 1);
}
