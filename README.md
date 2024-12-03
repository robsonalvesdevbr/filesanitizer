# namesanitizer

Enfatiza a ideia de "sanitizar" nomes de arquivos e pastas.

# Dependencias

```bash
cargo add clap --features derive # A flag features = ["derive"] permite usar macros derivadas para facilitar a definição de argumentos de linha de comando.
cargo add colored
cargo add chrono
cargo add regex
cargo add unicode-normalization
```

# Build/Run/Test

```bash
cargo run -- --help
cargo test -- --test-threads=1
cargo test -- --nocapture
```

# Utils

```bash
rustup component add rustfmt clippy
cargo fmt                     # Formatar todo o código
cargo install cargo-tarpaulin # Verificar cobertura de test
cargo tarpaulin

```

# Exemplos

https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html
