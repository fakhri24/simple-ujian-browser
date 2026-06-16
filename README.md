# Simple Ujian Browser

Custom exam browser untuk Windows — bagian dari ekosistem ujian digital.

## Strategi Hybrid

| Platform | Solusi |
|----------|--------|
| macOS/iPad | Safe Exam Browser (SEB) — install dari App Store |
| **Windows** | **Simple Ujian Browser — this app (Tauri + Rust)** |
| **Android** | **Simple Ujian Browser — planned** |

## Quick Start (Development)

```bash
cd ~/project/ujian/simple-ujian-browser
cargo tauri dev
```

## Build via CI/CD

Push a tag to trigger Windows build:
```bash
git tag v0.x.x
git push origin v0.x.x
```
GitHub Actions builds .msi + .exe artifacts.

## Config

Edit `config.json` that ships with the app:
- `exam_url` — URL ujian (default: simple-ujian.web.app)
- `whitelist` — allowed URLs during exam (origin; `*` matches one host segment, e.g. `https://*.googleapis.com`)
- `admin_password_hash` — **SHA-256** dari password keluar (Ctrl+Shift+Q). Password tidak pernah disimpan plaintext maupun dikirim ke frontend. Generate dengan:
  ```sh
  printf '%s' 'password-anda' | sha256sum   # Linux
  printf '%s' 'password-anda' | shasum -a 256   # macOS
  ```
  Default `guru2026` → `8ae731714ca5770a7b2f2c88f6e9e444e116aa61caa9e3874e04f4406c9d62ef`.
- `exam_name` — nama ujian yang tampil di info bar

## Exit

**Ctrl+Shift+Q** → enter admin password → app closes.

## Related Projects

- [simple-ujian](../simple-ujian/) — Website ujian online
- [grader-mtk](../grader-mtk/) — Sistem grading
- [web-ujian-mandiri](../web-ujian-mandiri/) — Ujian mandiri

## Docs

- [Architecture Guide](docs/ARCHITECTURE.md)
- [Implementation Plan](.hermes/plans/2026-06-16_desktop-implementation.md)
