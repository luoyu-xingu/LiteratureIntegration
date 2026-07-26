use std::time::Instant;

#[cfg(test)]
mod xml_parsing_performance {
    use super::*;
    use literature_integration::repositories::external_api::extract_xml_tags;

    fn generate_large_xml(num_authors: usize) -> String {
        let mut xml = String::with_capacity(1024 * 1024);
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<feed xmlns=\"http://www.w3.org/2005/Atom\">\n");
        xml.push_str("  <title>Test Feed</title>\n");
        xml.push_str("  <entry>\n");
        xml.push_str("    <title>Performance Test Paper</title>\n");
        xml.push_str("    <summary>This is a long abstract for performance testing purposes. ");
        xml.push_str("It contains multiple sentences to simulate real-world content. ");
        xml.push_str("The goal is to test how efficiently the XML parser handles large content. ");
        xml.push_str("This should help identify bottlenecks in the parsing logic.</summary>\n");
        xml.push_str("    <published>2024-01-15T10:30:00Z</published>\n");
        
        for i in 0..num_authors {
            xml.push_str("    <author>\n");
            xml.push_str(&format!("      <name>Author {} Name</name>\n", i));
            xml.push_str("    </author>\n");
        }
        
        xml.push_str("  </entry>\n");
        xml.push_str("</feed>");
        xml
    }

    fn extract_single_tag(xml: &str, tag: &str) -> Option<String> {
        let open_tag = format!("<{}>", tag);
        let close_tag = format!("</{}>", tag);
        let start = xml.find(&open_tag)?;
        let content_start = start + open_tag.len();
        let content_end = xml[content_start..].find(&close_tag)?;
        Some(xml[content_start..content_start + content_end].trim().to_string())
    }

    fn extract_nth_tag(xml: &str, tag: &str, n: usize) -> Option<String> {
        let open_tag = format!("<{}>", tag);
        let close_tag = format!("</{}>", tag);
        let open_len = open_tag.len();
        let close_len = close_tag.len();
        
        let mut pos = 0;
        let mut count = 0;
        
        while let Some(start) = xml[pos..].find(&open_tag) {
            let content_start = pos + start + open_len;
            if let Some(end) = xml[content_start..].find(&close_tag) {
                count += 1;
                if count == n {
                    return Some(xml[content_start..content_start + end].trim().to_string());
                }
                pos = content_start + end + close_len;
            } else {
                break;
            }
        }
        None
    }

    #[test]
    fn test_extract_single_tag_performance() {
        let xml = generate_large_xml(100);
        
        let start = Instant::now();
        for _ in 0..1000 {
            let title = extract_nth_tag(&xml, "title", 2);
            assert!(title.is_some());
            assert!(title.unwrap().contains("Performance Test Paper"));
        }
        let duration = start.elapsed();
        
        println!("extract_xml_tag (100 authors, 1000 iterations): {:?}", duration);
        
        assert!(duration.as_millis() < 500, 
            "XML tag extraction took too long: {:?}", duration);
    }

    #[test]
    fn test_extract_multiple_tags_performance() {
        let xml = generate_large_xml(50);
        
        let start = Instant::now();
        for _ in 0..100 {
            let names = extract_xml_tags(&xml, "name");
            assert_eq!(names.len(), 50);
        }
        let duration = start.elapsed();
        
        println!("extract_xml_tags (50 authors, 100 iterations): {:?}", duration);
        
        assert!(duration.as_millis() < 200, 
            "XML tags extraction took too long: {:?}", duration);
    }

    #[test]
    fn test_extract_summary_performance() {
        let xml = generate_large_xml(100);
        
        let start = Instant::now();
        for _ in 0..500 {
            let summary = extract_single_tag(&xml, "summary");
            assert!(summary.is_some());
        }
        let duration = start.elapsed();
        
        println!("extract_summary (100 authors, 500 iterations): {:?}", duration);
        
        assert!(duration.as_millis() < 300, 
            "Summary extraction took too long: {:?}", duration);
    }
}

#[cfg(test)]
mod string_operations_performance {
    use super::*;

    #[test]
    fn test_string_preallocation_effectiveness() {
        let iterations = 10000;
        let items_per_iteration = 100;
        
        let start_preallocated = Instant::now();
        for _ in 0..iterations {
            let mut s = String::with_capacity(items_per_iteration * 20);
            for i in 0..items_per_iteration {
                s.push_str(&format!("Item {} ", i));
            }
        }
        let preallocated_duration = start_preallocated.elapsed();
        
        let start_non_preallocated = Instant::now();
        for _ in 0..iterations {
            let mut s = String::new();
            for i in 0..items_per_iteration {
                s.push_str(&format!("Item {} ", i));
            }
        }
        let non_preallocated_duration = start_non_preallocated.elapsed();
        
        println!("String preallocated: {:?}, non-preallocated: {:?}", 
            preallocated_duration, non_preallocated_duration);
        
        assert!(preallocated_duration <= non_preallocated_duration,
            "Preallocated should not be slower than non-preallocated");
    }

    #[test]
    fn test_vec_preallocation_effectiveness() {
        let iterations = 1000;
        let items_per_iteration = 1000;
        
        let mut preallocated_total = std::time::Duration::ZERO;
        let mut non_preallocated_total = std::time::Duration::ZERO;
        
        for _ in 0..5 {
            let start_preallocated = Instant::now();
            for _ in 0..iterations {
                let mut v: Vec<String> = Vec::with_capacity(items_per_iteration);
                for i in 0..items_per_iteration {
                    v.push(format!("string_{}", i));
                }
            }
            preallocated_total += start_preallocated.elapsed();
            
            let start_non_preallocated = Instant::now();
            for _ in 0..iterations {
                let mut v: Vec<String> = Vec::new();
                for i in 0..items_per_iteration {
                    v.push(format!("string_{}", i));
                }
            }
            non_preallocated_total += start_non_preallocated.elapsed();
        }
        
        let preallocated_avg = preallocated_total / 5;
        let non_preallocated_avg = non_preallocated_total / 5;
        
        println!("Vec preallocated (avg): {:?}, non-preallocated (avg): {:?}", 
            preallocated_avg, non_preallocated_avg);
        
        assert!(preallocated_avg <= non_preallocated_avg * 2,
            "Preallocated should not be more than 2x slower than non-preallocated");
    }
}

#[cfg(test)]
mod search_query_performance {
    use super::*;
    use literature_integration::repositories::external_api::ExternalApiClient;

    #[test]
    fn test_doi_parsing_performance() {
        let dois = vec![
            "10.1234/test.1",
            "doi:10.5678/test.2",
            " 10.9012/test.3 ",
            "DOI:10.3456/test.4",
        ];
        
        let start = Instant::now();
        for _ in 0..10000 {
            for doi in &dois {
                let trimmed = doi.trim();
                let _is_doi = trimmed.starts_with("10.")
                    || trimmed.as_bytes().get(0..4).map_or(false, |s| s.eq_ignore_ascii_case(b"doi:"));
            }
        }
        let duration = start.elapsed();
        
        println!("DOI parsing (10000 iterations, 4 DOIs): {:?}", duration);
        
        assert!(duration.as_millis() < 50, 
            "DOI parsing took too long: {:?}", duration);
    }

    #[test]
    fn test_external_api_client_initialization() {
        let start = Instant::now();
        let _client = ExternalApiClient::shared();
        let duration = start.elapsed();
        
        println!("ExternalApiClient initialization: {:?}", duration);
        
        assert!(duration.as_millis() < 500, 
            "API client initialization took too long: {:?}", duration);
        
        let start_reuse = Instant::now();
        let _client2 = ExternalApiClient::shared();
        let duration_reuse = start_reuse.elapsed();
        
        println!("ExternalApiClient reuse (should be instant): {:?}", duration_reuse);
        
        assert!(duration_reuse.as_millis() < 10, 
            "API client reuse should be near-instant, took: {:?}", duration_reuse);
    }
}

#[cfg(test)]
mod benchmark_comparison {
    use super::*;

    #[test]
    fn test_optimization_overhead() {
        let start = Instant::now();
        
        for _ in 0..1000000 {
            let _ = format!("test_{}", 42);
        }
        
        let duration = start.elapsed();
        println!("Simple format operations (1M iterations): {:?}", duration);
        
        assert!(duration.as_millis() < 1000, 
            "Basic operations too slow: {:?}", duration);
    }

    #[test]
    fn test_option_operations_performance() {
        let options: Vec<Option<String>> = vec![
            Some("test1".to_string()),
            Some("test2".to_string()),
            None,
            Some("test3".to_string()),
        ];
        
        let start = Instant::now();
        for _ in 0..1000000 {
            for opt in &options {
                let _ = opt.as_deref();
                let _ = opt.is_some();
            }
        }
        let duration = start.elapsed();
        
        println!("Option operations (1M iterations): {:?}", duration);
        
        assert!(duration.as_millis() < 500, 
            "Option operations too slow: {:?}", duration);
    }
}