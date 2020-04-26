FROM rust:1.43 AS build

WORKDIR /build
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=build /build/target/release/httprom /httprom
ENTRYPOINT /httprom
