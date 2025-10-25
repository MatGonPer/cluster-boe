# --- Estágio 1: Build ---
# Usamos Rust nightly que suporta edition2024.
FROM rustlang/rust:nightly-bookworm-slim AS builder

# Cria nosso diretório de trabalho.
WORKDIR /app

RUN cargo install sqlx-cli --no-default-features --features rustls,postgres

# Copia APENAS os arquivos de manifesto de dependências.
COPY Cargo.toml Cargo.lock ./

# Cria diretório src e um main.rs dummy para compilar dependências
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Habilita o modo offline do SQLx para não precisar de banco durante build
ENV SQLX_OFFLINE=true

# Constrói apenas as dependências, criando a camada de cache.
RUN cargo build --release

# Remove o binário dummy (mantém as dependências compiladas em cache)
RUN rm -f target/release/deps/backend*

# Agora, copia o código-fonte real da sua aplicação.
# Se apenas o seu código mudar, o Docker começará a reconstruir a partir daqui,
# o que é muito mais rápido.
COPY ./src ./src

# Copia os metadados do SQLx para compilação offline
COPY ./.sqlx ./.sqlx

# Força recompilação do binário com o código real
RUN touch src/main.rs

# Constrói a sua aplicação. Desta vez, será quase instantâneo, pois
# todas as dependências já estão compiladas e em cache.
RUN cargo build --release

# --- Estágio 2: Final ---
# Usa uma imagem mínima e segura para o resultado final.
FROM gcr.io/distroless/cc-debian12

WORKDIR /app

# Copia APENAS o executável final do estágio de build.
COPY --from=builder /app/target/release/backend .
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
COPY migrations ./migrations

EXPOSE 3000

CMD ["./backend"]
