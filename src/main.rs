mod clipboard;
mod hash;
mod passphrase;
mod password;
mod pwhash;
mod strength;
mod vault_save;
mod wordlist;

use clap::{Parser, Subcommand};
use password::Charset;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(
    name = "keysmith",
    version = "0.1.0",
    about = "Password, passphrase, and hash generator"
)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    /// Disable cybercore color output.
    #[arg(long, global = true)]
    no_color: bool,

    /// Skip the startup banner.
    #[arg(long, global = true)]
    quiet: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Random character-based password.
    Password {
        #[arg(short, long, default_value_t = 20)]
        length: usize,
        #[arg(long)]
        no_lower: bool,
        #[arg(long)]
        no_upper: bool,
        #[arg(long)]
        no_digits: bool,
        #[arg(long)]
        no_symbols: bool,
        /// Exclude visually-ambiguous characters: 0 O 1 l I |
        #[arg(long)]
        exclude_ambiguous: bool,
        #[arg(short, long, default_value_t = 1)]
        count: usize,
        /// Copy the (first) generated password to the clipboard (wl-copy on Linux, pbcopy on macOS).
        #[arg(long)]
        copy: bool,
        /// Save the (first) generated password into CyberVault under this
        /// label (shells out to `cybervault add <label>` — CyberVault must
        /// be installed and its master-password prompt will appear).
        #[arg(long)]
        save: Option<String>,
        /// Machine-readable output: just the secret(s), one per line, no
        /// banner/entropy meter/color — implies --quiet. For scripting or
        /// another program (e.g. CyberVault's TUI) capturing stdout.
        #[arg(long)]
        raw: bool,
    },
    /// Diceware-style word passphrase (EFF large wordlist, 7776 words).
    Passphrase {
        #[arg(short, long, default_value_t = 6)]
        words: usize,
        #[arg(short, long, default_value = "-")]
        separator: String,
        #[arg(long)]
        capitalize: bool,
        #[arg(short, long, default_value_t = 1)]
        count: usize,
        #[arg(long)]
        copy: bool,
        /// Save the (first) generated passphrase into CyberVault under
        /// this label.
        #[arg(long)]
        save: Option<String>,
        /// Machine-readable output: just the secret(s), one per line, no
        /// banner/entropy meter/color — implies --quiet.
        #[arg(long)]
        raw: bool,
    },
    /// Checksum text or a file across SHA-256/SHA-512/BLAKE3.
    Hash {
        #[arg(long, conflicts_with = "file")]
        text: Option<String>,
        #[arg(long, conflicts_with = "text")]
        file: Option<PathBuf>,
    },
    /// Argon2id password hashing for storage — prompts for the password
    /// (hidden input, never a CLI argument or echoed to the terminal).
    Pwhash {
        /// Verify a password against this PHC hash string instead of
        /// hashing a new one.
        #[arg(long)]
        verify: Option<String>,
    },
}

fn banner(color_on: bool) {
    let (c, r) = if color_on {
        (cybercore::palette::purple(), cybercore::palette::RESET)
    } else {
        (String::new(), "")
    };
    println!(
        "{c}
  _  __             _           _ _   _
 | |/ /___ _   _ ___| |_ __ ___ (_) |_| |__
 | ' // _ \\ | | / __| | '_ ` _ \\| | __| '_ \\
 | . \\  __/ |_| \\__ \\ | | | | | | | |_| | | |
 |_|\\_\\___|\\__, |___/_|_| |_| |_|_|\\__|_| |_|
           |___/{r}"
    );
    println!("  » Password, passphrase & hash generator\n");
}

fn charset_from_flags(
    no_lower: bool,
    no_upper: bool,
    no_digits: bool,
    no_symbols: bool,
    exclude_ambiguous: bool,
) -> Charset {
    Charset {
        lower: !no_lower,
        upper: !no_upper,
        digits: !no_digits,
        symbols: !no_symbols,
        exclude_ambiguous,
    }
}

/// Shared by `password`/`passphrase`: handles `--copy` and `--save` on
/// the first generated secret, once generation is done. Neither flag
/// touches generation itself, so this stays common rather than
/// duplicated per command. In `--raw` mode these status lines go to
/// stderr instead of stdout, so stdout stays exactly the secret(s) for
/// whatever's capturing it.
fn handle_copy_and_save(first: Option<&str>, copy: bool, save: Option<&str>, raw: bool) {
    let Some(secret) = first else { return };
    let status = |msg: String| {
        if raw {
            eprintln!("{msg}")
        } else {
            println!("{msg}")
        }
    };
    if copy {
        match clipboard::copy(secret) {
            Ok(()) => status("(copied to clipboard)".to_string()),
            Err(e) => eprintln!("keysmith: failed to copy to clipboard: {e}"),
        }
    }
    if let Some(label) = save {
        match vault_save::save(label, secret) {
            Ok(true) => status(format!("(saved to CyberVault as \"{label}\")")),
            Ok(false) => eprintln!(
                "keysmith: cybervault reported failure saving \"{label}\" (see its output above)"
            ),
            Err(e) => eprintln!("keysmith: failed to run cybervault — is it installed? ({e})"),
        }
    }
}

fn run_password(
    length: usize,
    cs: &Charset,
    count: usize,
    copy: bool,
    save: Option<&str>,
    raw: bool,
    color_on: bool,
) -> ExitCode {
    let mut first: Option<String> = None;
    for _ in 0..count {
        match password::generate(length, cs) {
            Some(pw) => {
                if raw {
                    println!("{pw}");
                } else {
                    let bits = password::entropy_bits(length, cs);
                    println!(
                        "{}  {} {} bits ({})",
                        pw,
                        strength::meter(bits, 24, color_on),
                        bits.round(),
                        strength::label(bits)
                    );
                }
                if first.is_none() {
                    first = Some(pw);
                }
            }
            None => {
                eprintln!("keysmith: no character classes selected (or length is 0) — nothing to generate");
                return ExitCode::FAILURE;
            }
        }
    }
    handle_copy_and_save(first.as_deref(), copy, save, raw);
    ExitCode::SUCCESS
}

// Each parameter is a distinct clap-parsed CLI flag, same shape as
// run_password's 7 -- a struct wrapper would just move the same fields
// one level down for a thin dispatch function with a single call site.
#[allow(clippy::too_many_arguments)]
fn run_passphrase(
    words: usize,
    separator: &str,
    capitalize: bool,
    count: usize,
    copy: bool,
    save: Option<&str>,
    raw: bool,
    color_on: bool,
) -> ExitCode {
    let list = wordlist::words();
    let mut first: Option<String> = None;
    for _ in 0..count {
        match passphrase::generate(words, separator, capitalize, &list) {
            Some(p) => {
                if raw {
                    println!("{p}");
                } else {
                    let bits = passphrase::entropy_bits(words, list.len());
                    println!(
                        "{}  {} {} bits ({})",
                        p,
                        strength::meter(bits, 24, color_on),
                        bits.round(),
                        strength::label(bits)
                    );
                }
                if first.is_none() {
                    first = Some(p);
                }
            }
            None => {
                eprintln!("keysmith: word count is 0 — nothing to generate");
                return ExitCode::FAILURE;
            }
        }
    }
    handle_copy_and_save(first.as_deref(), copy, save, raw);
    ExitCode::SUCCESS
}

fn run_hash(text: Option<String>, file: Option<PathBuf>) -> ExitCode {
    let result = if let Some(t) = text {
        hash::hash_bytes(t.as_bytes())
    } else if let Some(f) = file {
        match hash::hash_file(&f) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("keysmith: failed to read {}: {e}", f.display());
                return ExitCode::FAILURE;
            }
        }
    } else {
        eprintln!("keysmith: hash needs either --text or --file");
        return ExitCode::FAILURE;
    };
    println!("sha256  {}", result.sha256);
    println!("sha512  {}", result.sha512);
    println!("blake3  {}", result.blake3);
    ExitCode::SUCCESS
}

fn run_pwhash(verify: Option<String>) -> ExitCode {
    let password = match rpassword::prompt_password("Password: ") {
        Ok(p) => p,
        Err(e) => {
            eprintln!("keysmith: failed to read password: {e}");
            return ExitCode::FAILURE;
        }
    };

    match verify {
        Some(phc) => match pwhash::verify_password(&password, &phc) {
            Ok(true) => {
                println!("match");
                ExitCode::SUCCESS
            }
            Ok(false) => {
                println!("no match");
                ExitCode::FAILURE
            }
            Err(e) => {
                eprintln!("keysmith: {e}");
                ExitCode::FAILURE
            }
        },
        None => match pwhash::hash_password(&password) {
            Ok(phc) => {
                println!("{phc}");
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("keysmith: {e}");
                ExitCode::FAILURE
            }
        },
    }
}

fn main() -> ExitCode {
    let args = Args::parse();
    let color_on = !args.no_color;
    let raw = matches!(&args.command, Commands::Password { raw, .. } | Commands::Passphrase { raw, .. } if *raw);

    if !args.quiet && !raw {
        banner(color_on);
    }

    match args.command {
        Commands::Password {
            length,
            no_lower,
            no_upper,
            no_digits,
            no_symbols,
            exclude_ambiguous,
            count,
            copy,
            save,
            raw,
        } => {
            let cs =
                charset_from_flags(no_lower, no_upper, no_digits, no_symbols, exclude_ambiguous);
            run_password(length, &cs, count, copy, save.as_deref(), raw, color_on)
        }
        Commands::Passphrase {
            words,
            separator,
            capitalize,
            count,
            copy,
            save,
            raw,
        } => run_passphrase(
            words,
            &separator,
            capitalize,
            count,
            copy,
            save.as_deref(),
            raw,
            color_on,
        ),
        Commands::Hash { text, file } => run_hash(text, file),
        Commands::Pwhash { verify } => run_pwhash(verify),
    }
}
