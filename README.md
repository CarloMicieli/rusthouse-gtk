# RustHouse

RustHouse is a GTK4-based desktop application for model railway collectors to organize their collections and wish lists. It runs on Linux and Windows, using an SQLite database for local data storage. The application enables users to manage detailed information about models, rolling stock, manufacturers, shops, and wish lists, with features for searching, filtering, and importing/exporting data.

## Features

- Manage a personal collection of model railway items
- Create and organize multiple wish lists
- Record detailed information about models, rolling stock, manufacturers, and shops
- Search, filter, and sort entries by various attributes
- Import/export data in CSV/JSON formats
- Preferences for currency, measurement system, favorite scales, companies, and eras
- GTK4 interface following GNOME HIG, with accessibility and theming support

## Building and Running

### Prerequisites

- **Rust toolchain** (https://rustup.rs/)
- **GTK4 development libraries**
  - On Linux: `libgtk-4-dev`, `libadwaita-1-dev`, and dependencies
  - On Windows: [MSYS2](https://www.msys2.org/) with `mingw-w64-x86_64-gtk4` and `mingw-w64-x86_64-adwaita-gtk`
- **Diesel CLI** for database migrations: `cargo install diesel_cli --no-default-features --features sqlite`

### Linux

1. Install dependencies:
   ```sh
   sudo apt update
   sudo apt install build-essential libgtk-4-dev libadwaita-1-dev pkg-config sqlite3 libsqlite3-dev
   ```
2. Clone the repository and enter the project directory:
   ```sh
   git clone https://github.com/carlomicieli/rusthouse-gtk.git
   cd rusthouse-gtk
   ```
3. Build the application:
   ```sh
   cargo build --release
   ```
4. Run the application:
   ```sh
   cargo run --release --manifest-path crates/app/Cargo.toml
   ```

### Windows

1. Install [MSYS2](https://www.msys2.org/) and update packages:
   ```sh
   pacman -Syu
   pacman -S mingw-w64-x86_64-toolchain mingw-w64-x86_64-gtk4 mingw-w64-x86_64-adwaita-gtk pkg-config sqlite3
   ```
2. Open the MSYS2 MinGW 64-bit shell and install Rust:
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. Add the Rust bin directory to your PATH if needed.
4. Clone the repository and enter the project directory:
   ```sh
   git clone https://github.com/carlomicieli/rusthouse-gtk.git
   cd rusthouse-gtk
   ```
5. Build the application:
   ```sh
   cargo build --release
   ```
6. Run the application:
   ```sh
   cargo run --release --manifest-path crates/app/Cargo.toml
   ```

### Database

- The SQLite database file is created automatically in the user’s home directory:
  - Linux: `~/.local/share/rusthouse/rusthouse.sqlite`
  - Windows: `%USERPROFILE%\AppData\Local\rusthouse\rusthouse.sqlite`

### Notes

- For development, use `cargo run` as shown above.
- For packaging, refer to Flatpak (Linux) or a Windows installer/portable build.
- Ensure GTK4 and Adwaita libraries are available in your environment.

## Contribution

Contributions are always welcome!

See [CONTRIBUTING.md](CONTRIBUTING.md) for ways to get started.

Please adhere to this project's [code of conduct](CODE_OF_CONDUCT.md).

## License

[Apache 2.0 License](https://choosealicense.com/licenses/apache-2.0/)

```
   Copyright 2022 Carlo Micieli

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
```
