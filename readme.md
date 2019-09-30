# card-game.rs

Run pre-built binaries:

    - ./target/release/card-game.exe

Build from source:
    
    - Download rustup: https://www.rust-lang.org/tools/install
    
    - In project root: cargo build --release
        - Build and run: cargo run --release

    INFO: 
    - rustup version 1.18.3 (435397f48 2019-05-22)
    - cargo version 1.38 (23ef9a4ef 2019-08-20)
    - (Exact version shouldn't matter, I tested it on multiple versions)

Note: This project has a single dependency: rand. When building the project from source, cargo will automatically download, compile and link dependencies. 
