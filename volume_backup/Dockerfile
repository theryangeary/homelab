FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ftp curl

# Latest releases available at https://github.com/aptible/supercronic/releases
ENV SUPERCRONIC_URL=https://github.com/aptible/supercronic/releases/download/v0.2.34/supercronic-linux-arm64 \
    SUPERCRONIC_SHA1SUM=4ab6343b52bf9da592e8b4bb7ae6eb5a8e21b71e \
    SUPERCRONIC=supercronic-linux-arm64

RUN curl -fsSLO "$SUPERCRONIC_URL" \
 && echo "${SUPERCRONIC_SHA1SUM}  ${SUPERCRONIC}" | sha1sum -c - \
 && chmod +x "$SUPERCRONIC" \
 && mv "$SUPERCRONIC" "/usr/local/bin/${SUPERCRONIC}" \
 && ln -s "/usr/local/bin/${SUPERCRONIC}" /usr/local/bin/supercronic

COPY volume_backup/main.sh .
COPY volume_backup/crontab crontab

RUN supercronic -test crontab

CMD supercronic crontab
