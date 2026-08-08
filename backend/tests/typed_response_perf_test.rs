//! 类型化响应结构体优化验证测试
//!
//! 验证内容:
//! 1. 正确性: 类型化结构体 (RemovedResponse / DeletedResponse / ErrorResponse)
//!    序列化后的字节与原 `serde_json::json!` 实现完全一致 (保证 API 输出无回归)。
//! 2. 性能要求: 类型化序列化不慢于 `serde_json::json!` 路径 (优化应真实生效)。
//!
//! 优化原理: `serde_json::json!` 会先构造中间 `serde_json::Value` 树
//! (默认 BTreeMap + String 键), 再二次序列化; 类型化结构体直接写入输出缓冲区,
//! 跳过中间分配与二次遍历。
//!
//! 本测试不需要 Neo4j 数据库连接。

use literature_integration::models::dto::{DeletedResponse, ErrorBody, ErrorResponse, RemovedResponse};
use std::time::Instant;

const ITERS: usize = 200_000;

// ==================== 正确性测试 ====================

#[test]
fn test_removed_response_byte_identical_to_json_macro() {
    let typed = serde_json::to_string(&RemovedResponse { removed: true }).unwrap();
    let json_macro = serde_json::to_string(&serde_json::json!({"removed": true})).unwrap();
    assert_eq!(typed, json_macro, "类型化输出应与 json! 完全一致");
    assert_eq!(typed, r#"{"removed":true}"#);
}

#[test]
fn test_removed_response_false_value() {
    let typed = serde_json::to_string(&RemovedResponse { removed: false }).unwrap();
    assert_eq!(typed, r#"{"removed":false}"#);
}

#[test]
fn test_deleted_response_byte_identical_to_json_macro() {
    let typed = serde_json::to_string(&DeletedResponse { deleted: true }).unwrap();
    let json_macro = serde_json::to_string(&serde_json::json!({"deleted": true})).unwrap();
    assert_eq!(typed, json_macro, "类型化输出应与 json! 完全一致");
    assert_eq!(typed, r#"{"deleted":true}"#);
}

#[test]
fn test_error_response_byte_identical_to_json_macro() {
    let code = "WORKSPACE_NOT_FOUND";
    let message = "Workspace not found: ws-123".to_string();

    let typed = serde_json::to_string(&ErrorResponse {
        error: ErrorBody { code, message: message.clone() },
    })
    .unwrap();

    let json_macro = serde_json::to_string(&serde_json::json!({
        "error": { "code": code, "message": message }
    }))
    .unwrap();

    assert_eq!(typed, json_macro, "类型化错误响应应与 json! 完全一致");
    // 字段顺序: code 在 message 之前 (与原实现一致)
    assert_eq!(
        typed,
        r#"{"error":{"code":"WORKSPACE_NOT_FOUND","message":"Workspace not found: ws-123"}}"#
    );
}

#[test]
fn test_error_response_all_variants_byte_identical() {
    // 覆盖 errors.rs 中所有错误变体的 code 值, 确保每个都字节一致
    let cases: &[(&str, &str)] = &[
        ("WORKSPACE_NOT_FOUND", "Workspace not found: w"),
        ("PAPER_NOT_FOUND", "Paper not found: p"),
        ("AUTHOR_NOT_FOUND", "Author not found: a"),
        ("IMPORT_FAILED", "Import failed: x"),
        ("NEO4J_ERROR", "Neo4j error: db"),
        ("VALIDATION_ERROR", "Validation error: bad"),
        ("EXTERNAL_API_ERROR", "External API error: timeout"),
        ("INTERNAL_ERROR", "Internal server error"),
    ];

    for (code, message) in cases {
        let message = message.to_string();
        let typed = serde_json::to_string(&ErrorResponse {
            error: ErrorBody { code, message: message.clone() },
        })
        .unwrap();
        let json_macro = serde_json::to_string(&serde_json::json!({
            "error": { "code": code, "message": message }
        }))
        .unwrap();
        assert_eq!(typed, json_macro, "code={} 不一致", code);
    }
}

// ==================== 性能要求测试 ====================
// 合理要求: 类型化路径不应慢于 serde_json::json! 路径。
// 由于类型化路径可证明地执行更少工作 (无中间 Value/BTreeMap 分配),
// 在足够迭代次数下应稳定地更快或相当。

#[test]
fn test_typed_removed_response_not_slower_than_json_macro() {
    // 预热, 消除首次分配 / 指令缓存冷启动影响
    for _ in 0..2000 {
        let _ = serde_json::to_string(&RemovedResponse { removed: true }).unwrap();
        let _ = serde_json::to_string(&serde_json::json!({"removed": true})).unwrap();
    }

    let start = Instant::now();
    for _ in 0..ITERS {
        let s = serde_json::to_string(&RemovedResponse { removed: true }).unwrap();
        std::hint::black_box(s);
    }
    let typed_total = start.elapsed();

    let start = Instant::now();
    for _ in 0..ITERS {
        let s = serde_json::to_string(&serde_json::json!({"removed": true})).unwrap();
        std::hint::black_box(s);
    }
    let json_total = start.elapsed();

    let ratio = typed_total.as_nanos() as f64 / json_total.as_nanos() as f64;
    eprintln!(
        "removed: typed={:?} json!={:?} (typed/json! ratio={:.3})",
        typed_total, json_total, ratio
    );

    assert!(
        typed_total <= json_total,
        "类型化序列化 ({:?}) 慢于 serde_json::json! ({:?}), 优化未生效",
        typed_total,
        json_total
    );
}

#[test]
fn test_typed_error_response_not_slower_than_json_macro() {
    let code = "PAPER_NOT_FOUND";
    let message = "Paper not found: paper-abc-123".to_string();

    // 预热
    for _ in 0..2000 {
        let _ = serde_json::to_string(&ErrorResponse {
            error: ErrorBody { code, message: message.clone() },
        })
        .unwrap();
        let _ = serde_json::to_string(&serde_json::json!({
            "error": { "code": code, "message": message.clone() }
        }))
        .unwrap();
    }

    let start = Instant::now();
    for _ in 0..ITERS {
        let s = serde_json::to_string(&ErrorResponse {
            error: ErrorBody { code, message: message.clone() },
        })
        .unwrap();
        std::hint::black_box(s);
    }
    let typed_total = start.elapsed();

    let start = Instant::now();
    for _ in 0..ITERS {
        let s = serde_json::to_string(&serde_json::json!({
            "error": { "code": code, "message": message.clone() }
        }))
        .unwrap();
        std::hint::black_box(s);
    }
    let json_total = start.elapsed();

    let ratio = typed_total.as_nanos() as f64 / json_total.as_nanos() as f64;
    eprintln!(
        "error: typed={:?} json!={:?} (typed/json! ratio={:.3})",
        typed_total, json_total, ratio
    );

    assert!(
        typed_total <= json_total,
        "类型化错误响应序列化 ({:?}) 慢于 serde_json::json! ({:?}), 优化未生效",
        typed_total,
        json_total
    );
}
