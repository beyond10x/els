use std::process::Command;

#[test]
fn validates_assertions_without_collecting_evidence() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_canon-engineering"))
        .current_dir(&root)
        .args([
            "gates",
            "validate",
            "--file",
            "fixtures/assertions/gates.yaml",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["valid"], true);
    assert_eq!(value["assertions"], 2);
    assert_eq!(value["requests"], 2);
}
