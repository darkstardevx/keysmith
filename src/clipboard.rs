//! Copies text to the Wayland clipboard via `wl-copy` — the right tool
//! for this Hyprland/Omarchy desktop, not a cross-platform clipboard
//! crate this box doesn't need.

use std::io::Write;
use std::process::{Command, Stdio};

pub fn copy(text: &str) -> std::io::Result<()> {
    let mut child = Command::new("wl-copy").stdin(Stdio::piped()).spawn()?;
    child.stdin.take().expect("stdin was piped").write_all(text.as_bytes())?;
    let status = child.wait()?;
    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "wl-copy exited with a non-zero status"))
    }
}
