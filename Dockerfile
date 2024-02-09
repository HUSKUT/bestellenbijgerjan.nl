FROM rust:1.74.1

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

WORKDIR /app
COPY . .

EXPOSE 8080

RUN rustup default nightly
RUN cargo build

CMD ["cargo", "run"]