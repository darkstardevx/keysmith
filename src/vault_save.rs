//! Pipes a generated secret into `cybervault add <label>` — Keysmith
//! doesn't link CyberVault as a library, it shells out to the standalone
//! binary, same "reuse via subprocess" pattern as every other
//! cross-tool integration this session (WraithFlow -> systemctl,
//! Undertow -> pacman, etc.). CyberVault's own master-password prompt
//! still goes straight to /dev/tty regardless of this piped stdin, so
//! there's no conflict between "the secret arrives via stdin" and "the
//! master password still needs real interactive input."

use std::io::Write;
use std::process::{Command, Stdio};

pub fn save(label: &str, secret: &str) -> std::io::Result<bool> {
    let mut child = Command::new("cybervault")
        .args(["add", label])
        .stdin(Stdio::piped())
        .spawn()?;
    child
        .stdin
        .take()
        .expect("stdin was piped")
        .write_all(secret.as_bytes())?;
    Ok(child.wait()?.success())
}
