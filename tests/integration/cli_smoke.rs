//! Smoke tests for top-level CLI commands not covered by the scenario harness:
//! `list`, `kill`, and `completions`.

use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};
use tempfile::TempDir;

fn binary_path() -> PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    PathBuf::from(manifest_dir)
        .join("target")
        .join("debug")
        .join("keep-running")
}

struct Env {
    _temp: TempDir,
    session_dir: PathBuf,
    socket_dir: PathBuf,
}

impl Env {
    fn new() -> Self {
        let temp = TempDir::new().expect("tempdir");
        let session_dir = temp.path().join("sessions");
        let socket_dir = temp.path().join("sockets");
        std::fs::create_dir_all(&session_dir).unwrap();
        std::fs::create_dir_all(&socket_dir).unwrap();
        Self {
            _temp: temp,
            session_dir,
            socket_dir,
        }
    }

    fn cmd(&self) -> Command {
        let mut c = Command::new(binary_path());
        c.env("KEEP_RUNNING_SESSION_DIR", &self.session_dir)
            .env("KEEP_RUNNING_SOCKET_DIR", &self.socket_dir);
        c
    }

    fn start_long_session(&self, name: &str) {
        let status = self
            .cmd()
            .args(["start", "--name", name, "--", "sleep", "30"])
            .status()
            .expect("spawn start");
        assert!(status.success(), "start command failed for {}", name);

        let socket = self.socket_dir.join(format!("{}.sock", name));
        let deadline = Instant::now() + Duration::from_secs(5);
        while !socket.exists() && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(20));
        }
        assert!(socket.exists(), "socket for {} never appeared", name);
    }
}

#[test]
fn list_empty_prints_help_text() {
    let env = Env::new();
    let out = env.cmd().arg("list").output().expect("run list");
    assert!(out.status.success(), "list should succeed on empty state");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("No running sessions"),
        "expected empty-list hint, got: {}",
        stdout
    );
}

#[test]
fn list_alias_ls_works() {
    let env = Env::new();
    let out = env.cmd().arg("ls").output().expect("run ls");
    assert!(out.status.success(), "ls alias should succeed");
}

#[test]
fn list_shows_running_session() {
    let env = Env::new();
    env.start_long_session("smoke-list");

    let out = env.cmd().arg("list").output().expect("run list");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("smoke-list"),
        "expected session name in list output, got: {}",
        stdout
    );

    // Clean up so the daemon doesn't outlive the test.
    let _ = env.cmd().args(["kill", "smoke-list"]).status();
}

#[test]
fn kill_by_exact_name_removes_session() {
    let env = Env::new();
    env.start_long_session("smoke-kill");

    let out = env.cmd().args(["kill", "smoke-kill"]).output().unwrap();
    assert!(out.status.success(), "kill should succeed");

    // Session file should be gone within a beat.
    let session_file = env.session_dir.join("smoke-kill.json");
    let deadline = Instant::now() + Duration::from_secs(2);
    while session_file.exists() && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(20));
    }
    assert!(
        !session_file.exists(),
        "session file should be removed after kill"
    );
}

#[test]
fn kill_by_prefix_matches_unique_session() {
    let env = Env::new();
    env.start_long_session("smoke-prefix-target");

    let out = env.cmd().args(["kill", "smoke-pref"]).output().unwrap();
    assert!(
        out.status.success(),
        "prefix kill failed: stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn kill_nonexistent_reports_error() {
    let env = Env::new();
    let out = env.cmd().args(["kill", "does-not-exist"]).output().unwrap();
    assert!(!out.status.success(), "kill of missing session should fail");
}

#[test]
fn completions_bash_emits_script() {
    let out = Command::new(binary_path())
        .args(["completions", "bash"])
        .output()
        .expect("run completions");
    assert!(out.status.success(), "completions bash should succeed");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(!stdout.is_empty(), "bash completions should not be empty");
    assert!(
        stdout.contains("keep-running") || stdout.contains("keep_running"),
        "bash completion should mention the binary name"
    );
}

#[test]
fn completions_zsh_emits_script() {
    let out = Command::new(binary_path())
        .args(["completions", "zsh"])
        .output()
        .expect("run completions");
    assert!(out.status.success(), "completions zsh should succeed");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("#compdef"),
        "zsh completion should start with #compdef"
    );
}

#[test]
fn completions_fish_emits_script() {
    let out = Command::new(binary_path())
        .args(["completions", "fish"])
        .output()
        .expect("run completions");
    assert!(out.status.success(), "completions fish should succeed");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(!stdout.is_empty(), "fish completions should not be empty");
}
