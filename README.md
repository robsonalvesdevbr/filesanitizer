# filesanitizer

Enfatiza a ideia de "sanitizar" nomes de arquivos e pastas.

## Comandos Resumidos com Descrições

### Python

```bash
winget uninstall Python.Launcher --all-versions    # Remove todas as versões do Python Launcher
winget uninstall Python.Python.3.10 --all-versions # Remove todas as versões do Python 3.10
winget install Python.Python.3.10                  # Instala o Python 3.10
```

### LLVM

```bash
winget install LLVM.LLVM                         # Instala o LLVM
clang --version                                  # Verifica a instalação do Clang
lldb --version                                   # Verifica a instalação do LLDB
```

### Rust

```bash
winget install Rustlang.Rustup                  # Instala o gerenciador Rustup
rustup install stable-gnu                       # Instala o Rust estável com suporte GNU
rustup default stable-gnu                       # Define o Rust GNU como padrão
rustup show                                     # Exibe as configurações do Rust
rustup component add rust-src                   # Adiciona o código-fonte do Rust
rustup component add llvm-tools-preview         # Adiciona ferramentas LLVM
rustup component add rust-analysis              # Adiciona arquivos de análise
rustup component add rustfmt clippy             # Adiciona ferramentas de formatação e linting
```

---

Com esta configuração, você terá um ambiente funcional para Python, LLVM e Rust. Para dúvidas ou problemas, sinta-se à vontade para pedir ajuda. 🚀

```

```

# Dependencias

```bash
cargo add clap --features derive # A flag features = ["derive"] permite usar macros derivadas para facilitar a definição de argumentos de linha de comando.
cargo add colored
cargo add chrono
cargo add regex
cargo add unicode-normalization
cargo add unicode-general-category
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
https://patriksvensson.se/posts/2018/02/debugging-rust-on-windows-using-vscode

---
