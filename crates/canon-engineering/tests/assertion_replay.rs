use serde_json::{Value, json};
use std::{
    fs,
    path::Path,
    process::{Command, Output},
};

#[test]
fn documented_workflow_uses_default_paths() {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir(root.path().join(".engineering")).unwrap();
    fs::write(
        root.path().join(".engineering/gates.yaml"),
        include_str!("../../../fixtures/assertions/gates.yaml"),
    )
    .unwrap();
    fs::write(
        root.path().join("Cargo.toml"),
        "[package]\nname = \"example\"\n",
    )
    .unwrap();
    fs::write(root.path().join("README.md"), "Apache-2.0").unwrap();
    assert_eq!(
        good(&cli(root.path(), &["gates", "validate"]))["valid"],
        true
    );
    good(&cli(root.path(), &["gates", "explain"]));
    let run = cli(root.path(), &["gates", "run"]);
    good(&run);
    let replay = cli(
        root.path(),
        &[
            "gates",
            "evaluate",
            "--evidence",
            ".engineering/assertions/evidence.json",
        ],
    );
    good(&replay);
    assert_eq!(run.stdout, replay.stdout);
}
fn cli(root: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_canon-engineering"))
        .current_dir(root)
        .args(args)
        .output()
        .unwrap()
}
fn gates(root: &Path, expressions: &[&str]) {
    fs::write(root.join("gates.yaml"),serde_json::to_vec(&json!({"format":"engineering-gates/1","language":"canon-expr/1","gates":[{"id":"ready","text":"Ready to finish","assertions":expressions}]})).unwrap()).unwrap();
}
fn good(output: &Output) -> Value {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}
#[test]
fn retained_evidence_replays_without_reading_sources_and_expires() {
    let root = tempfile::tempdir().unwrap();
    fs::write(root.path().join("README.md"), "hello").unwrap();
    gates(
        root.path(),
        &[
            "file_exists(\"README.md\")",
            "text.contains(\"README.md\", \"hello\")",
        ],
    );
    let first = cli(
        root.path(),
        &["gates", "run", "--file", "gates.yaml", "--now", "1000"],
    );
    assert_eq!(good(&first)["truth"], "true");
    fs::remove_file(root.path().join("README.md")).unwrap();
    fs::remove_file(root.path().join("gates.yaml")).unwrap();
    let replay = cli(
        root.path(),
        &[
            "gates",
            "evaluate",
            "--evidence",
            ".engineering/assertions/evidence.json",
        ],
    );
    good(&replay);
    assert_eq!(first.stdout, replay.stdout);
    let expired = cli(
        root.path(),
        &[
            "gates",
            "evaluate",
            "--evidence",
            ".engineering/assertions/evidence.json",
            "--now",
            "1301",
        ],
    );
    assert_eq!(expired.status.code(), Some(3));
    assert_eq!(
        serde_json::from_slice::<Value>(&expired.stdout).unwrap()["truth"],
        "unknown"
    );
    let changed = cli(
        root.path(),
        &[
            "gates",
            "evaluate",
            "--evidence",
            ".engineering/assertions/evidence.json",
            "--source-identity",
            "different",
        ],
    );
    assert_eq!(changed.status.code(), Some(3));
}
#[test]
fn recipes_deduplicate_and_empty_or_invalid_assertions_never_certify() {
    let root = tempfile::tempdir().unwrap();
    gates(
        root.path(),
        &["file_exists(\"README.md\") and fs.file.exists(\"README.md\")"],
    );
    assert_eq!(
        good(&cli(
            root.path(),
            &["gates", "validate", "--file", "gates.yaml"]
        ))["requests"],
        1
    );
    for expression in [
        "fs.file.exists(42)",
        "fs.file.exists(\"x\").imaginary",
        "true or undefined()",
        "1 < 2 < 3",
    ] {
        gates(root.path(), &[expression]);
        let output = cli(root.path(), &["gates", "validate", "--file", "gates.yaml"]);
        assert_eq!(output.status.code(), Some(2), "{expression}");
    }
    gates(root.path(), &[]);
    assert_eq!(
        cli(root.path(), &["gates", "run", "--file", "gates.yaml"])
            .status
            .code(),
        Some(2)
    );
}
#[test]
fn known_negative_is_false_and_bad_evidence_is_a_hard_error() {
    let root = tempfile::tempdir().unwrap();
    gates(root.path(), &["fs.file.exists(\"missing\")"]);
    let output = cli(
        root.path(),
        &["gates", "run", "--file", "gates.yaml", "--now", "10"],
    );
    assert_eq!(output.status.code(), Some(1));
    let path = root.path().join(".engineering/assertions/evidence.json");
    let mut bundle: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    bundle["observations"][0]["outcome"] =
        json!({"kind":"known","value":{"kind":"string","value":"false"}});
    fs::write(path, serde_json::to_vec(&bundle).unwrap()).unwrap();
    assert_eq!(
        cli(
            root.path(),
            &[
                "gates",
                "evaluate",
                "--evidence",
                ".engineering/assertions/evidence.json"
            ]
        )
        .status
        .code(),
        Some(2)
    );
}
#[test]
fn gate_source_cannot_be_replaced_by_evidence_output() {
    let root = tempfile::tempdir().unwrap();
    gates(root.path(), &["true"]);
    let before = fs::read(root.path().join("gates.yaml")).unwrap();
    assert_eq!(
        cli(
            root.path(),
            &[
                "gates",
                "run",
                "--file",
                "gates.yaml",
                "--out",
                "./gates.yaml"
            ]
        )
        .status
        .code(),
        Some(2)
    );
    assert_eq!(before, fs::read(root.path().join("gates.yaml")).unwrap());
}
