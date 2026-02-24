FROM rust:1.93.0 AS builder

WORKDIR /tmp

COPY Cargo.toml Cargo.lock /tmp/
COPY meme_generator /tmp/meme_generator
COPY meme_generator_cli /tmp/meme_generator_cli
COPY meme_generator_core /tmp/meme_generator_core
COPY meme_generator_memes /tmp/meme_generator_memes
COPY meme_generator_node /tmp/meme_generator_node
COPY meme_generator_py /tmp/meme_generator_py
COPY meme_generator_server /tmp/meme_generator_server
COPY meme_generator_utils /tmp/meme_generator_utils
COPY meme_options_derive /tmp/meme_options_derive

RUN export MEME_IMAGES_DIR=/app/resources/images \
  && cargo build --release --bin server

FROM debian:bookworm-slim AS app

EXPOSE 2233

ENV TZ=Asia/Shanghai \
  MEME_HOME=/data \
  RUST_LOG=info

VOLUME /data

WORKDIR /app

COPY --from=builder /tmp/target/release/server /app/server
COPY resources/images /app/resources/images/
COPY resources/fonts /usr/share/fonts/meme-fonts/

RUN apt-get update \
  && apt-get install -y --no-install-recommends openssl fontconfig \
  && fc-cache -fv \
  && rm -rf /var/lib/apt/lists/*

CMD ["/app/server"]
