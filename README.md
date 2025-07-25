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

1. set up a host with docker. do not use docker rootless!
1. clone this repo
1. get a GitHub personal access token (classic) with `read:packages` from https://github.com/settings/tokens/new?scopes=write:packages
1. `export CR_PAT=YOUR_TOKEN && echo $CR_PAT | docker login ghcr.io -u USERNAME --password-stdin`
1. set TUNNEL_TOKEN in .env
1. `docker swarm init` to add the host as a manager (all that is needed for a one-node swarm)
1. `docker stack config -c docker-compose.yml | docker stack deploy -c - homelab`
