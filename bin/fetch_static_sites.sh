#!/bin/sh
set -x

if [ -z "$1" ]; then
    echo "Usage: $0 <manifest file>"
    exit 1
fi

while read site; do
    $(git rev-parse --show-toplevel)/bin/fetch_github_release.sh $site
done < "$1"
