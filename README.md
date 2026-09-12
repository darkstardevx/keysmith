# 🔑 Keysmith

`Rust` · `Argon2id` · `BLAKE3`

**Password, passphrase, and hash generator.** One who makes keys.

## 🚀 What it does

```bash
keysmith password --length 24 --exclude-ambiguous --copy
keysmith passphrase --words 6 --capitalize
keysmith hash --file some-download.iso
keysmith pwhash                    # hash a password for storage (hidden prompt)
keysmith pwhash --verify '$argon2id$v=19$...'   # check a password against a stored hash
```

- **`password`** — random character-based, configurable charset (`--no-lower/--no-upper/--no-digits/--no-symbols`), `--exclude-ambiguous` drops visually-confusable characters (`0O1lI|`), `--count N` for a batch, `--copy` to the clipboard via `wl-copy`
- **`passphrase`** — diceware-style, real words from the **actual EFF large wordlist** (7776 words, embedded at compile time — not a small hardcoded sample)
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

## 🧩 Layout

```
src/wordlist.rs    embedded EFF large wordlist
src/password.rs    character-based generation + entropy
src/passphrase.rs  diceware generation + entropy
src/hash.rs        SHA-256/SHA-512/BLAKE3 checksums
src/pwhash.rs      Argon2id hash + verify
src/strength.rs    entropy -> label + colored meter bar
src/clipboard.rs   wl-copy integration
```

## 🗺 Known limitations

- Clipboard support is Wayland-only (`wl-copy`) — this box's desktop, not built to be portable
- `hash --file` reads the whole file into memory rather than streaming — fine for normal files, not ideal for something huge

## 📄 License

MIT
