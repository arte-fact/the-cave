FROM rust:slim-bullseye


WORKDIR /usr/app
COPY . .

RUN cargo install --path .
EXPOSE 9999
CMD ["the-cave"]
