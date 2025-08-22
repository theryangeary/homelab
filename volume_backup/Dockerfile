FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ftp

COPY volume_backup/main.sh .

CMD sh main.sh
