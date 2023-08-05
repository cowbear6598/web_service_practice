FROM rust:1.71 as builder
WORKDIR /usr/src/app
COPY . .
RUN Cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/web_service_pratice /usr/local/bin/web_service_pratice
CMD ["web_service_pratice"]