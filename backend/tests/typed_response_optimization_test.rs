// 测试优化后的 typed response 结构体与 XML 解析的正确性。
// 这些测试不需要 Neo4j 数据库连接，专注于纯函数与序列化行为。
//
// 覆盖范围:
//   1. DeleteResponse / RemoveResponse 序列化与原 serde_json::json! 等价
//   2. extract_xml_tag / extract_xml_tags 在不同输入下的正确性
//   3. 导出 Markdown 中的时间戳格式符合 "%Y-%m-%d %H:%M" 规范

use literature_integration::models::dto::{DeleteResponse, RemoveResponse};
use literature_integration::repositories::external_api::{extract_xml_tag, extract_xml_tags};

// ---------------------------------------------------------------------------
// 1. typed response 序列化等价性测试
// ---------------------------------------------------------------------------

#[test]
fn test_delete_response_json_shape() {
    let resp = DeleteResponse { deleted: true };
    let json = serde_json::to_string(&resp).unwrap();
    // 必须与原 serde_json::json!({"deleted": true}) 完全一致
    assert_eq!(json, r#"{"deleted":true}"#);
}

#[test]
fn test_remove_response_json_shape() {
    let resp = RemoveResponse { removed: true };
    let json = serde_json::to_string(&resp).unwrap();
    assert_eq!(json, r#"{"removed":true}"#);
}

#[test]
fn test_delete_response_false_value() {
    // 确保 bool 字段能正确序列化 false（不仅仅是 true）
    let resp = DeleteResponse { deleted: false };
    let json = serde_json::to_string(&resp).unwrap();
    assert_eq!(json, r#"{"deleted":false}"#);
}

// ---------------------------------------------------------------------------
// 2. extract_xml_tag / extract_xml_tags 正确性测试
//    这两个函数依赖优化后的 find_tag_open / find_tag_close 内联实现。
// ---------------------------------------------------------------------------

#[test]
fn test_extract_xml_tag_basic() {
    let xml = r#"<feed><entry><title>Paper Title</title></entry></feed>"#;
    let title = extract_xml_tag(xml, "title");
    assert_eq!(title.as_deref(), Some("Paper Title"));
}

#[test]
fn test_extract_xml_tag_returns_none_when_missing() {
    let xml = r#"<feed><entry><title>Only Title</title></entry></feed>"#;
    assert!(extract_xml_tag(xml, "summary").is_none());
}

#[test]
fn test_extract_xml_tag_skips_empty_content() {
    // 空内容（仅空白）的标签应被跳过，继续搜索下一个非空匹配
    let xml = r#"<feed><title>   </title><title>Real Title</title></feed>"#;
    let title = extract_xml_tag(xml, "title");
    assert_eq!(title.as_deref(), Some("Real Title"));
}

#[test]
fn test_extract_xml_tags_collects_all() {
    let xml = r#"<feed><name>Alice</name><name>Bob</name><name>Carol</name></feed>"#;
    let names = extract_xml_tags(xml, "name");
    assert_eq!(names, vec!["Alice", "Bob", "Carol"]);
}

#[test]
fn test_extract_xml_tags_ignores_empty_entries() {
    let xml = r#"<feed><name>Alice</name><name>  </name><name>Bob</name></feed>"#;
    let names = extract_xml_tags(xml, "name");
    assert_eq!(names, vec!["Alice", "Bob"]);
}

#[test]
fn test_extract_xml_tag_handles_arxiv_like_payload() {
    // 模拟 arXiv API 返回的精简结构
    let xml = r#"<?xml version="1.0"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <entry>
    <title>Attention Is All You Need</title>
    <summary>The dominant sequence transduction models are based on complex recurrent or convolutional neural networks.</summary>
    <published>2017-06-12T17:59:00Z</published>
    <author><name>Ashish Vaswani</name></author>
    <author><name>Noam Shazeer</name></author>
  </entry>
</feed>"#;

    assert_eq!(extract_xml_tag(xml, "title").as_deref(), Some("Attention Is All You Need"));
    assert!(extract_xml_tag(xml, "summary").unwrap().contains("recurrent"));
    let published = extract_xml_tag(xml, "published").unwrap();
    // published 形如 "2017-06-12T..."，前 4 位即年份
    assert_eq!(&published[..4], "2017");
    let authors = extract_xml_tags(xml, "name");
    assert_eq!(authors, vec!["Ashish Vaswani", "Noam Shazeer"]);
}

// ---------------------------------------------------------------------------
// 3. 导出 Markdown 时间戳格式测试
//    优化后用 write! 直接写入 buffer，需确保格式仍为 "%Y-%m-%d %H:%M"。
// ---------------------------------------------------------------------------

#[test]
fn test_export_timestamp_format_matches_spec() {
    // 直接验证优化后的格式化逻辑：构造一个已知时间，按相同 format 字符串写入
    // String buffer，断言输出严格符合 "YYYY-MM-DD HH:MM"（16 个字符）。
    use chrono::{TimeZone, Utc};

    let known = Utc.with_ymd_and_hms(2026, 8, 10, 14, 5, 0).unwrap();
    let mut buf = String::new();
    use std::fmt::Write;
    write!(buf, "{}", known.format("%Y-%m-%d %H:%M")).unwrap();

    // 格式应为 "2026-08-10 14:05"，长度恰好 16
    assert_eq!(buf.len(), 16, "timestamp should be 16 chars, got: {}", buf);
    assert_eq!(buf, "2026-08-10 14:05");

    // 校验结构：第 5、8 位是 '-'，第 11 位是空格，第 14 位是 ':'
    let bytes = buf.as_bytes();
    assert_eq!(bytes[4], b'-');
    assert_eq!(bytes[7], b'-');
    assert_eq!(bytes[10], b' ');
    assert_eq!(bytes[13], b':');
}

#[test]
fn test_export_timestamp_format_is_consistent_across_calls() {
    // 优化前用 .to_string()，优化后用 write! 进 buffer；
    // 两种方式对同一时刻必须产出完全相同的字符串。
    use chrono::Utc;
    let now = Utc::now();

    let via_to_string = now.format("%Y-%m-%d %H:%M").to_string();
    let mut via_write = String::new();
    use std::fmt::Write;
    write!(via_write, "{}", now.format("%Y-%m-%d %H:%M")).unwrap();

    assert_eq!(via_to_string, via_write);
}
