FROM rust:latest AS builder
ARG APP=bot-rs
WORKDIR /usr/src/${APP}
COPY . .
RUN cargo build --release

FROM debian:buster-slim 
ARG APP=bot-rs
COPY --from=builder /usr/src/${APP}/target/release/${APP} /usr/local/bin/${APP}
CMD ["bot-rs"]