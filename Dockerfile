FROM rust:latest as builder
RUN rustup default nightly

COPY . /builder
WORKDIR /builder
RUN cargo build --release

FROM ubuntu:latest

COPY --from=builder /builder /service
WORKDIR /service

RUN chmod +x entry.sh
RUN apt-get update -y
RUN apt-get install libssl1.1 -y

ENTRYPOINT ["bash", "./entry.sh"]