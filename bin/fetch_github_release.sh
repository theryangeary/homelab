#!/bin/sh
set -x

if [ -z "$1" ]; then
    echo "Usage: $0 <github_username>/<github_repo>"
    exit 1
fi

subdomain="$(echo $1 | cut -f 2- -d '/' | cut -f -1 -d .)"
outdir=$(git rev-parse --show-toplevel)/static/$subdomain

# backup existing deployment
if [ -d "$outdir" ]; then
    mv $outdir $outdir.bak
fi

# attempt to replace deployment
mkdir -p $outdir && wget -qO- https://github.com/$1/releases/latest/download/dist.tar.gz | gunzip | tar xvf - -C $outdir

# restore or cleanup previous deployment
if [ $? -eq 0 ]; then
    rm -rf $outdir.bak
else
    mv $outdir.bak $outdir
fi

