FROM rust:latest as builder
WORKDIR /malluscript
COPY . /malluscript/
RUN cargo build

FROM debian:bullseye-slim
COPY --from=builder /malluscript/target/debug/malluscript .
RUN touch /test.ms && chmod 777 /test.ms