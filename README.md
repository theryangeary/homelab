# homelab (ryangeary.dev)

This is my homelab, aka personal infrastructure I run at home. It is very WIP.

## Architecture (abridged)

1. cloudflare tunnel to connect a docker network to the public internet.
1. nginx docker container on the same docker network to receive and route connections.
    - this contains static site resources
1. nginx is also on an additional network, where services run.
1. right now there are no other services on the service networ. Nginx simply serves static sites.
1. all docker containers are running on a Raspberry Pi 3.

## Deploy

0. set up a host with docker
1. clone this repo
2. set CF_TOKEN in .env
4. docker compose up -d
