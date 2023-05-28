# https://www.lpalmieri.com/posts/fast-rust-docker-builds/
FROM rust:alpine AS rust
RUN apk add musl-dev openssl-dev
RUN cargo install cargo-chef
WORKDIR app

FROM rust AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin obsessed-yanqing


FROM alpine:latest
RUN apk add --no-cache libc6-compat
COPY --from=builder /app/target/release/obsessed-yanqing /usr/local/bin
ENTRYPOINT [ "/usr/local/bin/obsessed-yanqing" ]