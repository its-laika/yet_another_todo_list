FROM rust:latest AS build

WORKDIR /app

COPY migration/src migration/src
COPY migration/Cargo.lock migration/Cargo.lock
COPY migration/Cargo.toml migration/Cargo.toml

COPY src src
COPY Cargo.lock .
COPY Cargo.toml .

RUN cargo build --release

FROM rust:slim

WORKDIR /app

COPY --from=build /app/target/release/yet_another_todo_list .

CMD ["./yet_another_todo_list"]