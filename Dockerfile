FROM rust:slim-bullseye AS base
RUN apt-get update
RUN apt-get install libssl-dev pkg-config -y
RUN cargo install cargo-chef
WORKDIR app

FROM base AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM base AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin obsessed-yanqing

FROM bitnami/minideb:bullseye
RUN apt-get update
RUN apt-get install ca-certificates -y
RUN apt-get clean autoclean
RUN apt-get autoremove --yes
RUN rm -rf /var/lib/{apt,dpkg,cache,log}/
WORKDIR /root/
COPY --from=builder /app/target/release/obsessed-yanqing .
CMD ["./obsessed-yanqing"]