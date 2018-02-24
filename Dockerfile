FROM rust
RUN mkdir /app
COPY ./iron /app
WORKDIR /app/iron
RUN cargo build --release
CMD cargo run --release
