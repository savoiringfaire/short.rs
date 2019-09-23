FROM ekidd/rust-musl-builder as build

COPY ./ ./

RUN sudo chown -R rust:rust /home/rust

RUN cargo build --release

FROM scratch

COPY --from=build \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/shortner \
    /shortner

CMD ["/shortner"]
