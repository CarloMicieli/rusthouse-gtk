# GitHub Copilot Review Instructions

- All Rust crates must be inside the `crates/` directory; no source files outside it.
- Only Relm4 (for GTK 4 UI) and Diesel (for SQLite database) should be used.
- Code must be idiomatic Rust: use `Result`/`Option`, pattern matching, and avoid panics.
- Use clear, descriptive names for all identifiers.
- Prefer `let` bindings and minimize mutable state.
- Organize code with modules and crates.
- All public items must have `///` doc comments.
- No `unsafe` code is allowed.
- Code must be formatted with `cargo fmt` and pass `cargo clippy` without warnings.
- UI code must follow GNOME HIG: use standard widgets, ensure accessibility, and maintain clear visual hierarchy and spacing.
