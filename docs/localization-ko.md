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

Catalog keys, placeholders, escapes and trailing ellipses were validated locally. Rust tests
are included but could not be executed in the authoring environment: it has no Rust compiler
and cannot download dependencies. Compilation, visual layout, Windows IME and DOCX/PDF
round trips require the provided Windows CI workflow and a desktop smoke check.
