# 🔑 Keysmith

[![CI](https://github.com/darkstardevx/keysmith/actions/workflows/ci.yml/badge.svg)](https://github.com/darkstardevx/keysmith/actions/workflows/ci.yml)
[![Release](https://github.com/darkstardevx/keysmith/actions/workflows/release.yml/badge.svg)](https://github.com/darkstardevx/keysmith/actions/workflows/release.yml)

`Rust` · `Argon2id` · `BLAKE3`

**Password, passphrase, and hash generator.** One who makes keys.

## 📦 Install

```bash
curl -fsSL https://raw.githubusercontent.com/darkstardevx/keysmith/main/install.sh | sh
```

Downloads the latest release for your platform (Linux or macOS, x86_64
or aarch64), verifies its SHA-256 checksum, and installs `keysmith`
to `~/.local/bin`. Or build from source with `cargo build --release`.

Install [CyberVault](https://github.com/darkstardevx/cybervault) too if
you want `--save <label>` (piping a generated secret straight into it)
or CyberVault's own TUI calling back into Keysmith to generate on the
spot (Ctrl+G/Ctrl+P) — each shells out to the other's binary on `PATH`,
neither depends on the other at compile time.

## 🚀 What it does

```bash
keysmith password --length 24 --exclude-ambiguous --copy
keysmith password --length 24 --save github          # pipe straight into CyberVault
keysmith password --raw --length 24                  # bare secret, no banner/meter — for scripts/other tools
keysmith passphrase --words 6 --capitalize
keysmith hash --file some-download.iso
keysmith pwhash                    # hash a password for storage (hidden prompt)
keysmith pwhash --verify '$argon2id$v=19$...'   # check a password against a stored hash
```

- **`password`** — random character-based, configurable charset (`--no-lower/--no-upper/--no-digits/--no-symbols`), `--exclude-ambiguous` drops visually-confusable characters (`0O1lI|`), `--count N` for a batch, `--copy` to the clipboard via `wl-copy`, `--save <label>` to pipe the first generated password straight into [CyberVault](https://github.com/darkstardevx/cybervault) (`cybervault add <label>`), `--raw` for bare machine-readable output (also used by [CyberVault's own TUI](https://github.com/darkstardevx/cybervault) to generate on demand)
- **`passphrase`** — diceware-style, real words from the **actual EFF large wordlist** (7776 words, embedded at compile time — not a small hardcoded sample); also supports `--save <label>` and `--raw`
- **`hash`** — SHA-256/SHA-512/BLAKE3 side by side, of text or a file. Checksums, for integrity — **not** for storing passwords
- **`pwhash`** — Argon2id, the correct primitive for storing a password (deliberately slow + memory-hard + salted). Password is a hidden terminal prompt (`rpassword`, refuses piped/non-TTY input by design), never a CLI argument or plaintext on screen

Every generated password/passphrase prints with its **entropy in bits**
and a colored strength meter (`cybercore`-themed) — not just "here's a
string," but a real sense of how strong it actually is.

## 🎯 Why `hash` and `pwhash` are separate commands

Conflating them would be a real footgun. `hash` is fast general-purpose
checksumming (SHA-256/512, BLAKE3) — correct for verifying a file wasn't
corrupted or tampered with, **wrong** for storing a password (fast hashes
make offline brute-forcing cheap). `pwhash` is Argon2id specifically —
deliberately slow, memory-hard, and salted, the actual correct choice for
"I need to store this password's hash so I can check it later."

## ✅ Verification

30 unit tests, including known SHA-256/SHA-512 test vectors — cross-checked
against `sha512sum`/`sha256sum` directly rather than hand-typed from
memory (caught a transcription error in the test itself this way, not a
code bug — see `hash.rs`'s test comment). The embedded wordlist is
verified to be the real, complete 7776-word EFF list with no duplicates
(also caught a too-strict test assumption this way — 4 legitimate
hyphenated words like `t-shirt`/`yo-yo` needed the lowercase-only
assertion loosened, not the wordlist "fixed"). Argon2id hash/verify
round-tripped for real, including confirming the same password produces a
different stored hash each time (different random salt) while still
verifying correctly either way.

## 🔐 CyberVault integration

`--save <label>` doesn't link [CyberVault](https://github.com/darkstardevx/cybervault)
as a library — it shells out to the standalone `cybervault add <label>`
binary and pipes the first generated secret to it over stdin, the same
"reuse via subprocess" pattern used everywhere else in this toolset.
CyberVault's own master-password prompt still goes straight to the
terminal, so unlocking the vault works exactly as it would running
`cybervault` directly — Keysmith only supplies the secret being stored,
never the master password. Requires `cybervault` to be installed and
on `PATH`.

The integration runs the other direction too: CyberVault's TUI shells
out to `keysmith password --raw`/`keysmith passphrase --raw` (Ctrl+G /
Ctrl+P while entering a new entry's secret) to generate on the spot
instead of requiring you to type or paste one in. `--raw` exists
specifically for this — one secret on stdout, no banner, no color, no
entropy meter, so it's safe for another program to capture directly
rather than parsing colored/formatted output.

## 🧩 Layout

```
src/wordlist.rs    embedded EFF large wordlist
src/password.rs    character-based generation + entropy
src/passphrase.rs  diceware generation + entropy
src/hash.rs        SHA-256/SHA-512/BLAKE3 checksums
src/pwhash.rs      Argon2id hash + verify
src/strength.rs    entropy -> label + colored meter bar
src/clipboard.rs   wl-copy integration
src/vault_save.rs  pipes a generated secret into `cybervault add`
```

## 🗺 Known limitations

- Clipboard support: `wl-copy` on Linux (Wayland only — no X11 fallback), `pbcopy` on macOS
- `hash --file` reads the whole file into memory rather than streaming — fine for normal files, not ideal for something huge

## 📄 License

MIT
