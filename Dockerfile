FROM rust AS build

COPY ./ ./

RUN cargo build --release
RUN mkdir -p /build-out
RUN cp target/release/shortner /build-out/shortner

FROM scratch

COPY --from=build /build-out/shortner /

CMD ["/shortner"]
