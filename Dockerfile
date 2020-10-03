FROM ekidd/rust-musl-builder as build

WORKDIR /home/rust
RUN USER=rust cargo new --bin shortner
WORKDIR /home/rust/shortner

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
COPY ./static ./static
COPY ./img ./img
COPY ./templates ./templates

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/shortner*
RUN cargo build --release

FROM scratch

COPY --from=build \
    /home/rust/shortner/target/x86_64-unknown-linux-musl/release/shortner \
    /shortner

COPY --from=build \
    /home/rust/shortner/templates/* \
    /templates/

CMD ["/shortner"]
