FROM rust:1.82.0 AS builder

WORKDIR /tmp

COPY Cargo.toml Cargo.lock /tmp/
COPY meme_generator /tmp/meme_generator
COPY meme_generator_cli /tmp/meme_generator_cli
COPY meme_generator_py /tmp/meme_generator_py
COPY meme_generator_server /tmp/meme_generator_server
COPY meme_options_derive /tmp/meme_options_derive

RUN cargo build --release --bin server

FROM debian:bookworm-slim AS app

WORKDIR /app

EXPOSE 2233

ENV TZ=Asia/Shanghai \
  MEME_DISABLED_LIST="[]" \
  GIF_MAX_FRAMES=200 \
  DEFAULT_FONT_FAMILIES="['Noto Sans SC', 'Noto Color Emoji']"

RUN mkdir -p /root/.meme_generator/resources \
  && echo "\
[meme]\n\
meme_disabled_list = $MEME_DISABLED_LIST\n\
[encoder]\n\
gif_max_frames = $GIF_MAX_FRAMES\n\
[font]\n\
use_local_fonts = false\n\
default_font_families = $DEFAULT_FONT_FAMILIES\n\
[server]\n\
host = '0.0.0.0'\n\
port = 2233" > /root/.meme_generator/config.toml

COPY --from=builder /tmp/target/release/server /app/server
COPY resources/fonts /usr/share/fonts/meme-fonts/
COPY resources/images /root/.meme_generator/resources/images

RUN apt-get update \
  && apt-get install -y --no-install-recommends pkg-config libssl-dev fontconfig \
  && fc-cache -fv \
  && rm -rf /var/lib/apt/lists/*

CMD ["/app/server"]
