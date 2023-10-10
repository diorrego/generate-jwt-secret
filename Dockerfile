# Build stage
FROM rust:1 AS builder
WORKDIR /app
COPY . .
RUN cargo install --path .

# Production stage
FROM debian:buster-slim AS runner
COPY --from=builder /usr/local/cargo/bin/generate-jwt-secret /usr/local/bin/generate-jwt-secret
WORKDIR /usr/local/bin
RUN chmod +x generate-jwt-secret

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["generate-jwt-secret"]