FROM rust:latest AS build

WORKDIR /app

COPY . .

RUN cargo build --release

EXPOSE 1337

CMD ["cargo", "run"]
