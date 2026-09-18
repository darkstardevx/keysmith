//! Copies text to the clipboard: `wl-copy` on Linux/Wayland, `pbcopy` on
//! macOS now that this ships as a cross-platform release binary
//! (previously Hyprland/Omarchy-only, matching CyberVault's own fix).

use std::io::Write;
use std::process::{Command, Stdio};

#[cfg(target_os = "macos")]
const CLIPBOARD_CMD: &str = "pbcopy";

#[cfg(not(target_os = "macos"))]
const CLIPBOARD_CMD: &str = "wl-copy";

pub fn copy(text: &str) -> std::io::Result<()> {
    let mut child = Command::new(CLIPBOARD_CMD)
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|e| {
            std::io::Error::new(
                e.kind(),
                format!("failed to run '{CLIPBOARD_CMD}': {e} (is it installed and on PATH?)"),
            )
        })?;
    child
        .stdin
        .take()
        .expect("stdin was piped")
        .write_all(text.as_bytes())?;
    let status = child.wait()?;
    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(format!(
            "{CLIPBOARD_CMD} exited with a non-zero status"
        )))
    }
}
