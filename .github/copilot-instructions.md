# GitHub Copilot Instructions

## Folder Structure
- All Rust crates must be placed inside the `crates/` directory.
- No crates or source files should exist outside `crates/`.

## Libraries and Frameworks
- Use [Relm4](https://relm4.org/) for building GTK 4 desktop user interfaces.
- Use [Diesel](https://diesel.rs/) as the ORM for database access.
- The application is a GTK 4 desktop app using an SQLite database.

## Coding Standards
- Write idiomatic Rust: prefer `Result` and `Option` over panics, use pattern matching, and leverage the type system.
- Use clear, descriptive names for variables, functions, and types.
- Prefer `let` bindings and avoid mutable state unless necessary.
- Use modules and crates to organize code logically.
- Document public items with `///` doc comments.
- Do not use `unsafe` code.
- Format code with `cargo fmt` and lint with `cargo clippy`.

## UI Guidelines
- Follow the [GNOME Human Interface Guidelines](https://developer.gnome.org/hig/).
- Use GNOME default widgets and layouts for consistency.
- Ensure accessibility: provide labels, keyboard navigation, and high-contrast support.
- Use clear, concise language and standard GNOME icons.
- Maintain visual hierarchy and spacing for readability.
