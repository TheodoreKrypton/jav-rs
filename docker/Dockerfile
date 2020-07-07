# select build image
FROM rustlang/rust:nightly-buster as build

WORKDIR /

RUN git clone https://github.com/TheodoreKrypton/jav-rs.git
RUN cd /jav-rs && cargo build --release

# our final base
FROM debian:buster-slim

# copy the build artifact from the build stage
RUN apt-get update && \
    apt-get install -y --no-install-recommends openssl && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY --from=build /jav-rs/target/release/jav-rs .

# set the startup command to run your binary
CMD ["./jav-rs"]