#!/bin/sh

if [ -z "$1" ]; then
    echo "Usage: smallweb fetch_github_release <github_username>/<github_repo>"
    exit 1
fi

mkdir -p dist && wget -qO- https://github.com/theryangeary/www.ryangeary.dev/releases/latest/download/dist.tar.gz | gunzip | tar xvf - -C dist

