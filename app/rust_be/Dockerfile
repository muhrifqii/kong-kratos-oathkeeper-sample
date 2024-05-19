FROM rust:1.49-slim as builder

WORKDIR /app
COPY . /app
RUN cargo build --release

FROM alpine:latest

WORKDIR /app
COPY --from=builder /app/target/release/main /app/main
EXPOSE 8900

CMD ["./main"]
