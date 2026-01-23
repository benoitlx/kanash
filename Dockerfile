FROM rust:latest AS builder

# Copy sources inside the builder container
COPY ./ ./
RUN cargo install --locked trunk
RUN rustup target add wasm32-unknown-unknown
RUN cd kanash-ratzilla && trunk build
    
FROM python:3

COPY --from=builder /kanash-ratzilla/dist/ /dist

CMD ["python3", "-m", "http.server", "8000", "-d", "/dist"]

# LABEL \
#     org.opencontainers.image.title="kanash" \
#     org.opencontainers.image.description="learn kana in a terminal" \
#     org.opencontainers.image.authors="Benoit Leroux" \
#     org.opencontainers.image.licenses="MIT" \
#     org.opencontainers.image.source="https://github.com/${BUILD_REPOSITORY}" \
