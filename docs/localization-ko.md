# Korean interface (ko)

The Korean catalog has 2,448 entries, including all keys of the Bulgarian reference catalog
at upstream commit 159b84f54c74e9e82fb3109b758ae7dd6f4e3ab0. Feature names use familiar Korean
office terminology; help, instructions and errors are written independently from the source
meaning. Product names, command ids, file contents and placeholders are unchanged.

Choose File > Options > Interface language > 한국어. The existing automatic locale selector
recognizes ko-KR and ko_KR.UTF-8. The existing CJK font discovery includes the Korean catalog
because its language name contains Hangul; the Windows build also adds an unchanged, openly licensed Nanum Gothic font through an external craft-fonts input. Proprietary fonts are not copied or bundled.

This adds interface translation, not Korean spelling or grammar checking. English fallback
remains for strings outside the catalog and UI strings that upstream has not localized yet.

Validation on 2026-10-13: Windows x64 release build and all six project quality gates passed
(fmt, clippy, tests, assets, layering and wasm). Korean locale, full catalog coverage, placeholders,
font glyphs and saved language preference tests passed. The offscreen 1440 x 900 interface snapshot
was inspected: Korean ribbon captions and document text render correctly.

Validated source: `6dc0ace6ba0816cf7a5c3f7bce4910175c374c25`.
Build: https://github.com/ppongppang/wordcraft/actions/runs/38351751735
The portable artifact contains `wordcraft.exe`, licences, its SHA256 and any CI formatting/lockfile
patch. GitHub keeps these artifacts for 30 days.

Actual Windows keyboard composition and compatibility of existing complex DOCX/PDF files still
require a desktop smoke check; the offscreen screenshot is a rendering check.
