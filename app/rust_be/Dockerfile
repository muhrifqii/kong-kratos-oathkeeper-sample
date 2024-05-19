FROM rust:1.77.1-alpine3.19

RUN apk add --no-cache musl-dev
WORKDIR /app
COPY . .
RUN RUSTFLAGS='-C target-feature=-crt-static' cargo build --release
WORKDIR /app/target/release
EXPOSE 8900
CMD ["./rust_be"]
