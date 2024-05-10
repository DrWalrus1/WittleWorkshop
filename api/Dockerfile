FROM rust:latest

COPY ./ /api

WORKDIR /api

RUN cargo build

ENV ROCKET_PROFILE=release

EXPOSE 8000

CMD ["cargo", "run"]
