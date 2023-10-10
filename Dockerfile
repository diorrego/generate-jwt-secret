# Build stage
FROM rust:1 AS builder
WORKDIR /app
COPY . .
RUN cargo install --path .

# Production stage
FROM debian:buster-slim AS runner
COPY --from=builder /usr/local/cargo/bin/your-app /usr/local/bin/your-app
WORKDIR /usr/local/bin
RUN chmod +x your-app

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["generate-jwt-secret"]