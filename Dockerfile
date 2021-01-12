FROM alpine:edge AS build

RUN apk update &&\
    apk add git build-base openssl-dev rust cargo &&\
    mkdir /app

WORKDIR /app
COPY . .

ENV RUSTFLAGS="-C target-feature=+crt-static -C link-arg=-s"
RUN cargo build --target x86_64-alpine-linux-musl --release

FROM scratch
COPY --from=build /app/target/x86_64-alpine-linux-musl/release/counter-rust /
EXPOSE 8080
ENTRYPOINT ["/counter-rust"]
