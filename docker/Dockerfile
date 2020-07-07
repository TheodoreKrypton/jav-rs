FROM rustlang/rust:nightly-buster as build

WORKDIR /

RUN git clone https://github.com/TheodoreKrypton/jav-rs.git
RUN cd /jav-rs && cargo build --release

FROM debian:buster-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends openssl && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY --from=build /jav-rs/target/release/jav-rs .

CMD ["./jav-rs"]