FROM rust:latest as builder
RUN rustup default nightly

COPY . /builder
WORKDIR /builder
RUN cargo build --release

FROM ubuntu:latest

COPY --from=builder /builder /service
WORKDIR /service

RUN chmod +x entry.sh

ENTRYPOINT ["bash", "./entry.sh"]