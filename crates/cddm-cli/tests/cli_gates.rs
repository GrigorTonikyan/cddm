#![forbid(unsafe_code)]

use std::process::Command;

fn cddm_bin() -> &'static str {
    env!("CARGO_BIN_EXE_cddm")
}

#[test]
fn test_dead_code_cli_fail_gates_and_exit_codes() {
    let temp = tempfile::tempdir().expect("tempdir");
    let root = temp.path();

    std::fs::write(
        root.join("sample.ts"),
        "function usedHelper(x: number): number {\n  return x + 1;\n}\n\n\
         function unusedHelper(y: number): number {\n\
           const a = y * 2;\n\
           const b = a + 3;\n\
           const c = b * 4;\n\
           const d = c - 5;\n\
           const e = d + 6;\n\
           return e;\n\
         }\n\n\
         export const value = usedHelper(41);\n",
    )
    .expect("write fixture");

    // Default without fail gate exits 0 even when findings exist
    let default_out = Command::new(cddm_bin())
        .args([
            "dead-code",
            &root.to_string_lossy(),
            "--min-tokens",
            "10",
            "--static-only",
        ])
        .output()
        .expect("run cddm dead-code");
    assert!(
        default_out.status.success(),
        "default dead-code should exit 0, stderr: {}",
        String::from_utf8_lossy(&default_out.stderr)
    );

    // --fail-on-findings exits non-zero when dead functions exist
    let fail_out = Command::new(cddm_bin())
        .args([
            "dead-code",
            &root.to_string_lossy(),
            "--min-tokens",
            "10",
            "--static-only",
            "--fail-on-findings",
        ])
        .output()
        .expect("run cddm dead-code --fail-on-findings");
    assert!(
        !fail_out.status.success(),
        "expected non-zero exit with --fail-on-findings, stdout: {}",
        String::from_utf8_lossy(&fail_out.stdout)
    );

    // --max-items 1000 exits 0
    let max_out = Command::new(cddm_bin())
        .args([
            "dead-code",
            &root.to_string_lossy(),
            "--min-tokens",
            "10",
            "--static-only",
            "--max-items",
            "1000",
        ])
        .output()
        .expect("run cddm dead-code --max-items 1000");
    assert!(
        max_out.status.success(),
        "expected exit 0 with --max-items 1000, stderr: {}",
        String::from_utf8_lossy(&max_out.stderr)
    );
}
