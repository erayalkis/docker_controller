FROM rust:1.69 as build

RUN USER=root cargo new --bin docker_controller
WORKDIR /docker_controller

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/docker_controller*
RUN cargo build --release

FROM debian:bullseye-20230703-slim

COPY --from=build /docker_controller/target/release/docker_controller .

EXPOSE 8000

CMD ["./docker_controller"]
