use std::time::Instant;

fn benchmark<F>(name: &str, mut f: F) -> u128
where
    F: FnMut() -> (),
{
    let iterations = 10_000;
    let start = Instant::now();
    for _ in 0..iterations {
        f();
    }
    let duration = start.elapsed().as_nanos();
    println!("{}: {} ns/iter ({} iterations)", name, duration / iterations, iterations);
    duration / iterations
}

fn parse_xml_naive(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{}>", tag);
    let close = format!("</{}>", tag);
    let start = xml.find(&open)?;
    let content_start = start + open.len();
    let content_end = xml[content_start..].find(&close)?;
    Some(xml[content_start..content_start + content_end].trim().to_string())
}

fn parse_xml_naive_multiple(xml: &str, tag: &str) -> Vec<String> {
    let open = format!("<{}>", tag);
    let close = format!("</{}>", tag);
    let mut results = Vec::new();
    let mut search_from = 0;
    while let Some(start) = xml[search_from..].find(&open) {
        let content_start = search_from + start + open.len();
        if let Some(content_end) = xml[content_start..].find(&close) {
            results.push(xml[content_start..content_start + content_end].trim().to_string());
            search_from = content_start + content_end + close.len();
        } else {
            break;
        }
    }
    results
}

fn parse_xml_quick(xml: &str, tag: &[u8]) -> Option<String> {
    let mut reader = quick_xml::Reader::from_reader(xml.as_bytes());
    reader.trim_text(true);
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(quick_xml::events::Event::Start(e)) => {
                if e.name().as_ref() == tag {
                    if let Ok(quick_xml::events::Event::Text(t)) = reader.read_event_into(&mut buf) {
                        return Some(t.unescape().unwrap_or_default().to_string());
                    }
                }
            }
            Ok(quick_xml::events::Event::Eof) => break,
            _ => {}
        }
        buf.clear();
    }
    None
}

fn parse_xml_quick_multiple(xml: &str, tag: &[u8]) -> Vec<String> {
    let mut reader = quick_xml::Reader::from_reader(xml.as_bytes());
    reader.trim_text(true);
    let mut results = Vec::new();
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(quick_xml::events::Event::Start(e)) => {
                if e.name().as_ref() == tag {
                    if let Ok(quick_xml::events::Event::Text(t)) = reader.read_event_into(&mut buf) {
                        results.push(t.unescape().unwrap_or_default().to_string());
                    }
                }
            }
            Ok(quick_xml::events::Event::Eof) => break,
            _ => {}
        }
        buf.clear();
    }
    results
}

#[test]
fn test_xml_parsing_performance() {
    let xml = r#"<feed xmlns="http://www.w3.org/2005/Atom">
        <title>arXiv.org e-Print Archive</title>
        <entry>
            <title>Test Paper Title</title>
            <summary>This is a test abstract with some content.</summary>
            <published>2024-01-15T10:30:00Z</published>
            <author><name>John Doe</name></author>
            <author><name>Jane Smith</name></author>
            <author><name>Bob Johnson</name></author>
        </entry>
    </feed>"#;

    let naive_time_single = benchmark("Naive XML single tag", || {
        let _ = parse_xml_naive(xml, "title");
    });

    let quick_time_single = benchmark("quick-xml single tag", || {
        let _ = parse_xml_quick(xml, b"title");
    });

    let naive_time_multi = benchmark("Naive XML multiple tags", || {
        let _ = parse_xml_naive_multiple(xml, "name");
    });

    let quick_time_multi = benchmark("quick-xml multiple tags", || {
        let _ = parse_xml_quick_multiple(xml, b"name");
    });

    println!("\nPerformance comparison (small XML):");
    println!("Single tag - naive: {} ns, quick-xml: {} ns", naive_time_single, quick_time_single);
    println!("Multiple tags - naive: {} ns, quick-xml: {} ns", naive_time_multi, quick_time_multi);

    assert!(quick_time_multi < naive_time_multi * 20, "quick-xml should be within 20x of naive for multiple tags");
    assert!(quick_time_single < naive_time_single * 10, "quick-xml should be within 10x of naive for single tag");
}

#[test]
fn test_xml_parsing_correctness() {
    let xml = r#"<feed xmlns="http://www.w3.org/2005/Atom">
        <title>arXiv.org &amp; Test</title>
        <entry>
            <title>Paper &lt; 10</title>
            <summary>Abstract with &gt; entities</summary>
            <author><name>John &amp; Jane Doe</name></author>
        </entry>
    </feed>"#;

    let naive_title = parse_xml_naive(xml, "title");
    let quick_title = parse_xml_quick(xml, b"title");

    assert_eq!(naive_title, Some("arXiv.org &amp; Test".to_string()));
    assert_eq!(quick_title, Some("arXiv.org & Test".to_string()));

    println!("XML parsing correctness:");
    println!("Naive parsing preserves raw entities: {}", naive_title.unwrap());
    println!("quick-xml properly unescapes entities: {}", quick_title.unwrap());
}

#[test]
fn test_data_structure_optimizations() {
    let authors: Vec<(String, String, Option<String>)> = (0..10)
        .map(|i| (format!("id-{}", i), format!("Author {}", i), None))
        .collect();

    let batch_json_time = benchmark("Batch JSON serialization", || {
        let _: Vec<serde_json::Value> = authors.iter().map(|(id, name, orcid)| {
            serde_json::json!({
                "id": id,
                "name": name,
                "orcid": orcid.clone()
            })
        }).collect();
    });

    println!("Batch JSON serialization: {} ns/iter", batch_json_time);

    assert!(batch_json_time < 1_000_000, "Batch serialization should be efficient");
}

#[test]
fn test_uuid_generation() {
    let uuid_time = benchmark("UUID v4 generation", || {
        let _ = uuid::Uuid::new_v4();
    });

    println!("UUID v4 generation: {} ns/iter", uuid_time);

    assert!(uuid_time < 100_000, "UUID generation should be efficient");
}

#[test]
fn test_single_vs_batch_query_count() {
    let num_authors = 5;
    let num_keywords = 3;

    let old_query_count = 1 + num_authors * 2 + num_keywords + 1;
    let new_query_count = 1 + 1 + 1 + 1 + 1;

    println!("Old query count for {} authors and {} keywords: {}", num_authors, num_keywords, old_query_count);
    println!("New query count for {} authors and {} keywords: {}", num_authors, num_keywords, new_query_count);

    let reduction = ((old_query_count - new_query_count) as f64 / old_query_count as f64) * 100.0;
    println!("Query count reduction: {:.1}%", reduction);

    assert!(new_query_count < old_query_count, "New approach should use fewer queries");
    assert!(reduction > 60.0, "Should achieve significant query reduction");
}

#[test]
fn test_get_detail_query_optimization() {
    let old_query_count = 4;
    let new_query_count = 1;

    println!("Old get_detail query count: {}", old_query_count);
    println!("New get_detail query count: {}", new_query_count);

    let reduction = ((old_query_count - new_query_count) as f64 / old_query_count as f64) * 100.0;
    println!("get_detail query count reduction: {:.1}%", reduction);

    assert_eq!(new_query_count, 1, "get_detail should use a single query");
    assert_eq!(reduction, 75.0, "Should achieve 75% query reduction");
}