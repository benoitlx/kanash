FROM rust:latest AS builder

# Copy sources inside the builder container
WORKDIR /usr/src
COPY kanash-ratzilla/ kanash-ratzilla/
COPY kanash-components/ kanash-components/
COPY kanash/ kanash/
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall -y trunk
RUN rustup target add wasm32-unknown-unknown
RUN rustup target add x86_64-unknown-linux-musl
RUN cd kanash-ratzilla && trunk build
RUN cd kanash && cargo build --target x86_64-unknown-linux-musl
    
FROM python:3

COPY --from=builder /usr/src/kanash-ratzilla/dist/ /dist
COPY --from=builder /usr/src/kanash/target/x86_64-unknown-linux-musl/debug/kanash /usr/bin

CMD ["python3", "-m", "http.server", "8000", "-d", "/dist"]

# LABEL \
#     org.opencontainers.image.title="kanash" \
#     org.opencontainers.image.description="learn kana in a terminal" \
#     org.opencontainers.image.authors="Benoit Leroux" \
#     org.opencontainers.image.licenses="MIT" \
#     org.opencontainers.image.source="https://github.com/${BUILD_REPOSITORY}" \
