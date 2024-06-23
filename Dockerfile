FROM rust:latest

COPY ./ /api

WORKDIR /api

RUN cargo build --release

ENV ROCKET_PROFILE=release

EXPOSE 8000

CMD ["cargo", "run", "--release"]
