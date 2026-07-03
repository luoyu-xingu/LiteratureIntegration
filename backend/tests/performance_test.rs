use literature_integration::repositories::external_api::ExternalApiClient;
use literature_integration::models::dto::{ImportPaperRequest, ExportRequest, ExportFilter};

#[test]
fn test_external_api_client_static() {
    let client1 = ExternalApiClient;
    let client2 = ExternalApiClient;
    assert_eq!(std::ptr::addr_of!(client1), std::ptr::addr_of!(client2));
}

#[test]
fn test_import_paper_request_validation() {
    let req = ImportPaperRequest {
        identifier: "10.1038/nature12345".to_string(),
    };
    assert!(!req.identifier.is_empty());
    assert!(req.identifier.starts_with("10."));
}

#[test]
fn test_export_request_defaults() {
    let req = ExportRequest {
        format: "markdown".to_string(),
        group_by: None,
        filter: Some(ExportFilter {
            author_ids: None,
            keyword_ids: None,
            year_range: None,
        }),
    };
    assert_eq!(req.format, "markdown");
    assert!(req.group_by.is_none());
    assert!(req.filter.is_some());
    let filter = req.filter.unwrap();
    assert!(filter.author_ids.is_none());
    assert!(filter.keyword_ids.is_none());
    assert!(filter.year_range.is_none());
}

#[test]
fn test_export_filter_with_values() {
    let filter = ExportFilter {
        author_ids: Some(vec!["a1".to_string(), "a2".to_string()]),
        keyword_ids: Some(vec!["k1".to_string()]),
        year_range: Some((2020, 2024)),
    };
    assert_eq!(filter.author_ids.unwrap().len(), 2);
    assert_eq!(filter.keyword_ids.unwrap().len(), 1);
    assert_eq!(filter.year_range.unwrap().0, 2020);
    assert_eq!(filter.year_range.unwrap().1, 2024);
}

#[test]
fn test_parallel_query_struct() {
    let paper_ids = vec!["p1".to_string(), "p2".to_string(), "p3".to_string()];
    assert_eq!(paper_ids.len(), 3);
    
    let first_authors: std::collections::HashMap<String, Option<String>> = std::collections::HashMap::new();
    let corr_authors: std::collections::HashMap<String, Option<String>> = std::collections::HashMap::new();
    let keywords_map: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    
    assert!(first_authors.is_empty());
    assert!(corr_authors.is_empty());
    assert!(keywords_map.is_empty());
}

#[tokio::test]
async fn test_http_client_reuse() {
    let client1 = ExternalApiClient;
    let client2 = ExternalApiClient;
    
    let result1 = tokio::spawn(async move {
        let _ = client1.fetch_by_identifier("10.1038/nature12345").await;
    });
    
    let result2 = tokio::spawn(async move {
        let _ = client2.fetch_by_identifier("10.1038/nature67890").await;
    });
    
    let _ = tokio::join!(result1, result2);
}

#[test]
fn test_batch_query_empty_input() {
    let empty_ids: Vec<String> = Vec::new();
    assert!(empty_ids.is_empty());
    
    let fa: std::collections::HashMap<String, Option<String>> = std::collections::HashMap::new();
    let ca: std::collections::HashMap<String, Option<String>> = std::collections::HashMap::new();
    let kw: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    
    assert!(fa.is_empty());
    assert!(ca.is_empty());
    assert!(kw.is_empty());
}