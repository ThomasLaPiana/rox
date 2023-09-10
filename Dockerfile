# --- Builder Stage
FROM rust:latest as builder

WORKDIR /usr/src/rox

COPY . .

RUN cargo build --release

RUN cargo install --path .

CMD ["/usr/local/cargo/bin/rox"]

# --- Final Stage

FROM alpine:latest

COPY --from=builder /usr/local/cargo/bin/rox /usr/local/bin/rox

CMD ["rox"]
