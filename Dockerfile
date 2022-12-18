FROM rust:1.66 as builder
WORKDIR /mocker
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN cargo build --release

#FROM debian:bullseye-slim
FROM redhat/ubi8-micro:latest
COPY --from=builder /mocker/target/release/mocker /usr/local/bin/mocker
CMD ["mocker"]

# buildah bud -t mocker:latest .