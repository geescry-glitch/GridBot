
FROM rust:latest
WORKDIR /app
COPY backend ./backend
WORKDIR /app/backend
RUN cargo build --release
CMD ["./target/release/fed-grid-institutional-ultimate"]
