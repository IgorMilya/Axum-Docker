# Slim Docker File

FROM rust:1.76.0-alpine3.19 as builder

RUN apk add --no-cache musl-dev openssl-dev pkgconfig

WORKDIR /usr/src/Axum
COPY . .
RUN cargo build --release

FROM alpine:3.19.0
COPY --from=builder /usr/src/Axum/target/release/Axum /
CMD ["./Axum"]

