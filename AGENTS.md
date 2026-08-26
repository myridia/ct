# AGENTS.md — ct

## What it is

**ct** is a Rust desktop text editor with built-in AES-128-GCM encryption. It encrypts/decrypts text files using a password, producing a hex-encoded string in the format `nonce/ciphertext/mac`. Two variants exist:

- **ct** — GUI app (eframe/egui) with file dialogs, clipboard, search, i18n
- **ct_nox** — CLI/headless version, same crypto, usable from other programs

## Stack

- **Language**: Rust (edition 2024, rust-version 1.94.1)
- **GUI**: eframe 0.26.2 (egui)
- **Crypto**: `rust-crypto` crate (AES-128-GCM via `AesGcm`)
- **CLI**: `clap` (arg parsing in ct_nox)
- **i18n**: `i18n-embed` + `i18n-embed-fl` (Fluent system), 95 languages
- **Config**: TOML (`~/.ct/config.toml`), `serde` + `toml` crates
- **Clipboard**: `cli-clipboard`
- **File dialogs**: `rfd` (native file dialogs)
- **Image**: `png` crate (binary frame encode/decode)
- **License**: GPL-3.0
- **Git**: `ssh://git@codeberg.org/veto/ct` (+ GitHub releases)

## Structure

```
ct/
├── src/                 # GUI app (main crate)
│   ├── main.rs          # eframe entry, CT struct, UI layout
│   ├── lib.rs           # re-exports config, icon
│   ├── config.rs        # ~/.ct/config.toml read/write (AppConfig { language })
│   ├── icon.rs          # hardcoded 64x64 RGBA icon
│   └── assets/fonts/    # 19 .ttf/.otf fonts for international scripts
├── ct_nox/              # CLI subcrate
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs      # clap CLI, encrypt/decrypt by args
│   │   ├── lib.rs       # re-exports modules + shared get_valid_key()
│   │   ├── ct_nox.rs    # read_file(), write_file()
│   │   ├── encrypt.rs   # encrypt(text, password) -> hex_nonce/hex_cipher/hex_mac
│   │   ├── decrypt.rs   # decrypt(ciphertext, password) -> plaintext
│   │   └── image_strip.rs # encode text→PNG frame, decode PNG frame→text
│   ├── run.sh, add.sh, remove.sh, revert.sh
│   └── ct_nox           # prebuilt binary
├── i18n/                # 95 Fluent .ftl locale dirs (e.g. en-US/ct.ftl)
├── pages/public/        # favicon, images
├── test/                # experiments: hello_world, rust, java, kotlin, filedialog
├── en-US.ftl            # root Fluent file (used by i18n-embed at build)
├── i18n.toml            # i18n config: fallback_language = "en-US"
├── Cargo.toml           # workspace root, members = ["ct_nox"]
├── run.sh               # cargo watch dev runner
├── ftl_maker            # binary tool for .ftl files
└── rustfmt.toml
```

## Crypto details

- **Algorithm**: AES-128-GCM (AEAD)
- **Key**: password bytes, padded with 0x00 to 16 bytes or truncated to 16 bytes
- **Nonce/IV**: 12 random bytes
- **MAC**: 16 bytes (authentication tag)
- **Output format**: `{hex_nonce}/{hex_ciphertext}/{hex_mac}`
- **Note**: decryption returns original text on failure (no error shown to user)

## Image Strip (binary frame)

Encodes encrypted text as a binary frame around a 256x256 PNG image. Each character = 8 pixels (1px dots), black = 1, white = 0. Bits go clockwise from top-left. ct64.png logo centered in the middle. Max capacity: 127 characters.

**CLI usage:**
```bash
# Encrypt text → image
ct_nox image-encode -t "secret" -p "password" -o out.png

# Encrypt file → image
ct_nox image-encode -f secret.txt -p "password" -o out.png

# Decode image → decrypted text
ct_nox image-decode -f out.png -p "password"
```

**GUI**: "Export as Image" and "Import from Image" buttons in toolbar + File menu.

**Format**: `{hex_nonce}/{hex_ciphertext}/{hex_mac}` encoded as black/white pixels around the image perimeter.

## Config

Stored at `~/.ct/config.toml`:
```toml
language = "en-US"
```
Created automatically on first run. Defaults to system locale if available in i18n/, else `en-US`.

## Build & Run

```bash
# Linux deps
sudo apt-get install -y libclang-dev libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev

# Dev (hot-reload)
./run.sh  # cargo watch -w src/ -w ct_nox/src -x run

# Release build (Linux)
cargo build --release

# Cross-compile Windows
rustup target add x86_64-pc-windows-gnu
sudo apt install mingw-w64
cargo build --target x86_64-pc-windows-gnu --release

# Cross-compile Mac (needs osxcross)
PATH="$(pwd)/osxcross/target/bin:$PATH" cargo build --target x86_64-apple-darwin
```

## Conventions

- **No comments in code unless asked.**
- Workspace-wide rules in `~/webs/Agents/AGENTS.md`.
- Verification: `cargo check && cargo build` (run on target host — no cargo in dev container).
- `ct_nox` is a library crate consumed by both the GUI app and its own CLI binary.
- i18n keys are defined in `en-US.ftl` (root) and mirrored under `i18n/{locale}/ct.ftl`.
