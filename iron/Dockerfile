FROM rust
RUN mkdir /app
COPY . /app
WORKDIR /app/iron
RUN cargo build --release
CMD cargo run --release
