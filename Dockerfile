FROM rust:1.58.1-alpine3.14

RUN apk --no-cache add make gcc g++

COPY ./ ./

RUN cargo build --release

EXPOSE 3335

CMD ["./target/release/whisper"]