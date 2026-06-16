# Simple Ujian Browser

Custom exam browser untuk sekolah — bagian dari ekosistem ujian digital.

## Strategi: Hybrid Approach

| Platform | Solusi | Status |
|----------|--------|--------|
| macOS | Safe Exam Browser (SEB) | ✅ Siap pakai |
| iOS/iPad | Safe Exam Browser (SEB) | ✅ Siap pakai |
| Windows | Custom Tauri + Rust | 🔨 Planned |
| Android | Custom Tauri 2 + Rust | 🔨 Planned |

## Dokumentasi

- [Architecture & Implementation Guide](docs/ARCHITECTURE.md)

## Related Projects

- [simple-ujian](../simple-ujian/) — Website ujian online (Firebase)
- [grader-mtk](../grader-mtk/) — Sistem grading matematika
- [web-ujian-mandiri](../web-ujian-mandiri/) — Ujian mandiri

## Stack

- **Backend:** Rust (Tauri 2)
- **Frontend:** Vanilla JS
- **Config:** JSON (local file)
- **Detection:** User agent compatible with simple-ujian SEB check
