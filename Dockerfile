FROM rust:1.43 AS build

WORKDIR /build
COPY . .
RUN cargo build --release

FROM ubuntu:20.04
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1
COPY --from=build /build/target/release/httprom /httprom
ENTRYPOINT ["/httprom"]
