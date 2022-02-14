FROM ekidd/rust-musl-builder:latest as builder

ADD --chown=rust:rust . ./

RUN cargo build --release

FROM alpine:latest

RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/whisper-rust \
    /usr/local/bin/

EXPOSE 3335

CMD ["/usr/local/bin/whisper-rust"]