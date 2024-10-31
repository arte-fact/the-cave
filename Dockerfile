FROM rust:latest
WORKDIR /usr/app
COPY . .
RUN cargo build --release

EXPOSE 7878
CMD ["/usr/app/target/release/the-cave"]
