FROM rust:1.93-alpine AS build
WORKDIR /src
RUN apk add --no-cache musl-dev
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release

FROM alpine:3.22
RUN apk add --no-cache ca-certificates
COPY --from=build /src/target/release/logsblox-otel-sdk-rust /app
ENTRYPOINT ["/app"]
