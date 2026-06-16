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
- `whitelist` — allowed URLs during exam
- `admin_password` — password untuk keluar (Ctrl+Shift+Q)
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
