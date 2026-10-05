use assertion_providers::{Collected, ProviderContext, collect};
use serde_json::json;
use std::collections::BTreeMap;
#[test]
fn absent_file_is_known_false_and_document_malformed_is_error() {
    let root = tempfile::tempdir().unwrap();
    let context = ProviderContext::new(root.path());
    let args = BTreeMap::from([("path".into(), json!("missing"))]);
    assert_eq!(
        collect("fs.file.exists", &args, &context).unwrap(),
        Collected::Known(json!(false))
    );
    std::fs::write(root.path().join("bad.json"), "{").unwrap();
    let args = BTreeMap::from([("path".into(), json!("bad.json"))]);
    assert!(collect("json.read", &args, &context).is_err());
}
#[test]
fn empty_test_run_never_certifies_and_generated_extra_file_is_detected() {
    let root = tempfile::tempdir().unwrap();
    let context = ProviderContext::new(root.path());
    std::fs::write(
        root.path().join("tests.json"),
        r#"{"format":"engineering-tests/1","inventory":[],"passed":0,"failed":0,"skipped":0}"#,
    )
    .unwrap();
    let args = BTreeMap::from([("path".into(), json!("tests.json"))]);
    assert!(collect("tests.report", &args, &context).is_err());
    std::fs::create_dir(root.path().join("expected")).unwrap();
    std::fs::create_dir(root.path().join("actual")).unwrap();
    std::fs::write(root.path().join("actual/extra"), "extra").unwrap();
    let args = BTreeMap::from([
        ("expected".into(), json!("expected")),
        ("actual".into(), json!("actual")),
    ]);
    assert_eq!(
        collect("generated.matches", &args, &context).unwrap(),
        Collected::Known(json!(false))
    );
}
