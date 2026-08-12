// Validates the "serialize delete responses via typed structs" optimization.
//
// The optimization replaced `serde_json::json!({"removed": true})` /
// `serde_json::json!({"deleted": true})` in the delete handlers with the typed
// `PaperRemovedResponse` / `WorkspaceDeletedResponse` structs.
//
// Requirements covered:
//   1. Correctness: the typed struct must produce byte-identical JSON to the
//      old `serde_json::json!` output, preserving the frontend contract
//      (field names `removed` / `deleted`).
//   2. Performance: serializing the typed struct must be strictly faster than
//      building + serializing the equivalent `serde_json::Value` tree, because
//      the typed path skips the intermediate `Value` (Map + Bool) allocation
//      and the second serialization pass. Measured as the minimum wall-clock
//      over several trials of a large iteration count to suppress noise.

use std::hint::black_box;
use std::time::Instant;

use literature_integration::models::dto::{PaperRemovedResponse, WorkspaceDeletedResponse};

/// Number of serialization iterations per trial. Large enough that the absolute
/// difference (tens of ns/iter) dwarfs timer resolution noise.
const ITERS: usize = 1_000_000;
/// Number of trials; we keep the minimum (least noisy) per approach.
const TRIALS: usize = 5;

fn min_trial_duration<T, F: FnMut() -> T>(mut work: F) -> std::time::Duration {
    let mut best: Option<std::time::Duration> = None;
    for _ in 0..TRIALS {
        let start = Instant::now();
        for _ in 0..ITERS {
            black_box(work());
        }
        let dur = start.elapsed();
        best = Some(best.map_or(dur, |b: std::time::Duration| b.min(dur)));
    }
    best.unwrap()
}

#[test]
fn test_paper_removed_response_byte_identical_to_json_macro() {
    let typed = serde_json::to_string(&PaperRemovedResponse { removed: true }).unwrap();
    let legacy = serde_json::to_string(&serde_json::json!({ "removed": true })).unwrap();
    assert_eq!(typed, legacy, "typed struct must match legacy json! output");
    assert_eq!(typed, r#"{"removed":true}"#);
}

#[test]
fn test_workspace_deleted_response_byte_identical_to_json_macro() {
    let typed = serde_json::to_string(&WorkspaceDeletedResponse { deleted: true }).unwrap();
    let legacy = serde_json::to_string(&serde_json::json!({ "deleted": true })).unwrap();
    assert_eq!(typed, legacy, "typed struct must match legacy json! output");
    assert_eq!(typed, r#"{"deleted":true}"#);
}

#[test]
fn test_paper_removed_response_is_strictly_faster_than_value_tree() {
    // Warm up the allocator / caches so the first trial isn't penalized.
    for _ in 0..10_000 {
        let _ = serde_json::to_string(&PaperRemovedResponse { removed: true }).unwrap();
        let _ = serde_json::to_string(&serde_json::json!({ "removed": true })).unwrap();
    }

    let typed_dur = min_trial_duration(|| {
        serde_json::to_string(&PaperRemovedResponse { removed: true }).unwrap()
    });
    let legacy_dur = min_trial_duration(|| {
        serde_json::to_string(&serde_json::json!({ "removed": true })).unwrap()
    });

    println!(
        "paper removed: typed={:?} legacy={:?} (iters={}, trials={})",
        typed_dur, legacy_dur, ITERS, TRIALS
    );

    // The typed path does strictly less work (no intermediate Value tree, no
    // re-serialization), so its best-case time must be strictly lower.
    assert!(
        typed_dur < legacy_dur,
        "typed serialization ({:?}) must be strictly faster than json! ({:?})",
        typed_dur,
        legacy_dur
    );
}

#[test]
fn test_workspace_deleted_response_is_strictly_faster_than_value_tree() {
    for _ in 0..10_000 {
        let _ = serde_json::to_string(&WorkspaceDeletedResponse { deleted: true }).unwrap();
        let _ = serde_json::to_string(&serde_json::json!({ "deleted": true })).unwrap();
    }

    let typed_dur = min_trial_duration(|| {
        serde_json::to_string(&WorkspaceDeletedResponse { deleted: true }).unwrap()
    });
    let legacy_dur = min_trial_duration(|| {
        serde_json::to_string(&serde_json::json!({ "deleted": true })).unwrap()
    });

    println!(
        "workspace deleted: typed={:?} legacy={:?} (iters={}, trials={})",
        typed_dur, legacy_dur, ITERS, TRIALS
    );

    assert!(
        typed_dur < legacy_dur,
        "typed serialization ({:?}) must be strictly faster than json! ({:?})",
        typed_dur,
        legacy_dur
    );
}
