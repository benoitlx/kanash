FROM rust:latest AS builder

# Copy sources inside the builder container
COPY ./Cargo.lock ./Cargo.toml ./
COPY ./src ./src
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl
    
FROM alpine:latest

COPY --from=builder /target/x86_64-unknown-linux-musl/debug/kanash /usr/bin
RUN apk update && apk add ttyd

CMD ["/usr/bin/ttyd", "-W", "/usr/bin/kanash"]

# LABEL \
#     org.opencontainers.image.title="kanash" \
#     org.opencontainers.image.description="learn kana in a terminal" \
#     org.opencontainers.image.authors="Benoit Leroux" \
#     org.opencontainers.image.licenses="MIT" \
#     org.opencontainers.image.source="https://github.com/${BUILD_REPOSITORY}" \
