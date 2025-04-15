FROM rust:latest AS builder

# Copy sources inside the builder container
COPY ./Cargo.lock ./Cargo.toml ./
COPY ./src ./src
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl

# Download ttyd binary
RUN wget https://github.com/tsl0922/ttyd/releases/download/1.7.7/ttyd.aarch64
    
FROM alpine:latest

COPY --from=builder /ttyd.aarch64 /bin/ttyd
COPY --from=builder /target/x86_64-unknown-linux-musl/debug/kanash /bin
RUN chmod +x /bin/ttyd
# RUN chmod +x /bin/kanash

CMD ["/bin/ttyd", "-W", "/bin/kanash"]

# LABEL \
#     org.opencontainers.image.title="kanash" \
#     org.opencontainers.image.description="learn kana in a terminal" \
#     org.opencontainers.image.authors="Benoit Leroux" \
#     org.opencontainers.image.licenses="MIT" \
#     org.opencontainers.image.source="https://github.com/${BUILD_REPOSITORY}" \
