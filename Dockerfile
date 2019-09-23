FROM ekidd/rust-musl-builder

COPY ./ ./

RUN cargo build --target x86_64-unknown-linux-musl --release
RUN mkdir -p /build-out
RUN cp target/x86_64-unknown-linux-musl/release/shortner /build-out/shortner

FROM scratch

COPY --from=build /build-out/shortner /

CMD ["/shortner"]
