FROM ghcr.io/pomdtr/smallweb:latest

RUN apt update && apt install -y wget

RUN mkdir -p smallweb/www/dist && wget -qO- https://github.com/theryangeary/www.ryangeary.dev/releases/latest/download/dist.tar.gz | gunzip | tar xvf - -C smallweb/www/dist
