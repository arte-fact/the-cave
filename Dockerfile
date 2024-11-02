FROM rust:1.82-bullseye
WORKDIR /usr/app
COPY . .

RUN cargo install --path .
EXPOSE 8080
CMD ["the-cave"]
