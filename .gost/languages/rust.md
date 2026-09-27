# GOST Language Specification: Rust
**Identificador da Stack:** `rust`  
**Padrão Base:** GOST `1.0.0`

---

## 1. Visão Geral e Filosofia
Esta especificação define os requisitos de engenharia, ferramentas e padrões para desenvolvimento em **Rust** (edição 2021 ou superior) no ecossistema GOST.

---

## 2. Ferramental Mandatório de Testes & Qualidade

| Requisito GOST | Ferramenta Padrão | Comando de Validação no CI |
|---|---|---|
| **Test Runner** | **Cargo Test** | `cargo test --all-targets --all-features` |
| **Cobertura (≥ 90%)** | **cargo-llvm-cov** / **tarpaulin** | `cargo llvm-cov --fail-under-lines 90` |
| **Testes de Mutação** | **cargo-mutants** | `cargo mutants --no-shuffle` |
| **Linter (Zero Warnings)** | **Clippy** | `cargo clippy --all-targets --all-features -- -D warnings` |
| **Formatação Determinística** | **rustfmt** | `cargo fmt -- --check` |
| **Auditoria de Segurança** | **cargo-audit** | `cargo audit` |

---

## 3. Gestão de Dependências (Twelve-Factor)
1. **Lockfile Obrigatório:** O arquivo `Cargo.lock` **DEVE** estar commitado na raiz.
2. **Dependências Mínimas:** Evitar crates pesadas quando a biblioteca padrão for suficiente. Feature flags ativadas apenas sob necessidade estrita.
3. **Compilação Hermética:** Builds de produção usam `--locked` para garantir que nenhuma dependência transitória flutue:
   ```bash
   cargo build --release --locked
   ```

---

## 4. Arquitetura de Domínio & SOLID Pragmático
1. **Estrutura de Pastas Canônica:**
   ```text
   src/
   ├── domain/             # Modelos e regras de negócio puras (sem I/O)
   │   ├── mod.rs
   │   ├── entity.rs       # Invariantes e structs de dados
   │   ├── status.rs       # Enums nativos para máquinas de estado
   │   └── ports.rs        # Traits de I/O (Output Ports)
   ├── application/        # Use cases e orquestração
   │   ├── mod.rs
   │   └── use_cases.rs
   ├── adapters/           # Adaptadores de I/O
   │   ├── mod.rs
   │   ├── in_memory.rs    # Fake em memória com HashMap (testes herméticos)
   │   ├── sqlite.rs       # Persistência concreta
   │   └── cli.rs          # Parser de linha de comando (clap)
   └── main.rs             # Composition Root (bootstrap e injeção)
   ```
2. **In-Memory Fakes:** Testes unitários instanciam structs que implementam os traits de domínio armazenando dados em `HashMap` ou `RwLock<Vec<T>>` na memória, rodando em microssegundos sem dependência de SQLite ou sockets.
3. **Parse, Don't Validate:** Validação de entradas externas na borda (parsers `clap` ou DTOs `serde`). O Core opera com primitivos confiáveis ou structs transparentes.
4. **Estados com Enums Nativos:**
   ```rust
   pub enum TaskState {
       Pending,
       Running { started_at: u64 },
       Finished { exit_code: i32 },
       Failed { error: String },
   }
   ```

---

## 5. Padrões de Sistema e Containers OCI
1. **Conformidade XDG:**
   Resolução de diretórios respeita variáveis de ambiente com fallback para `$HOME`:
   ```rust
   pub fn config_dir(app: &str) -> std::path::PathBuf {
       std::env::var("XDG_CONFIG_HOME")
           .map(std::path::PathBuf::from)
           .unwrap_or_else(|_| {
               let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
               std::path::PathBuf::from(home).join(".config")
           })
           .join(app)
   }
   ```
2. **Dockerfile Canônico (Multi-stage + Scratch Estático musl):**
   ```dockerfile
   FROM rust:1.80-alpine3.20 AS builder
   RUN apk add --no-cache musl-dev
   WORKDIR /build
   COPY Cargo.toml Cargo.lock ./
   COPY src ./src
   RUN cargo build --release --target x86_64-unknown-linux-musl --locked

   FROM scratch
   USER 1000:1000
   COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/app /usr/local/bin/app
   ENTRYPOINT ["/usr/local/bin/app"]
   ```
3. **Streams:** Saída pura de dados via `println!()`, logs e mensagens diagnósticas via `eprintln!()` ou crate `tracing`/`env_logger` configurada para `stderr`.
