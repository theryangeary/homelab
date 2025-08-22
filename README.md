# homelab (ryangeary.dev)

This is my homelab, aka personal infrastructure I run at home. It is very WIP.

## Architecture (abridged)

1. cloudflare tunnel to connect a docker network to the public internet.
1. nginx docker container on the same docker network to receive and route connections.
    - this contains static site resources
1. nginx is also on an additional network, where services run.
1. right now there are no other services on the service network. Nginx simply serves static sites.
1. all docker containers are running on a Raspberry Pi 3.

## Deploy

### on your development machine (used to control deployments):
1. set TUNNEL_TOKEN in .env
1. set MANAGER_HOST in .env
1. set FTP_HOST in .env
1. set FTP_USERNAME in .env

### on your swarm manager server (where deploys are hosted):
1. install docker. do not use docker rootless!
1. get a GitHub personal access token (classic) with `read:packages` from https://github.com/settings/tokens/new?scopes=write:packages
1. `export CR_PAT=YOUR_TOKEN && echo $CR_PAT | docker login ghcr.io -u USERNAME --password-stdin`
1. `docker swarm init` to add the host as a manager (all that is needed for a one-node swarm)
1. `docker stack config -c docker-compose.yml | docker stack deploy -c - homelab`

### on swarm nodes that are not managers
// TODO if I ever add some, but basically the above^ but join to the swarm
instead of being a manager

### do the deploy (on dev machine)
1. if you are upgrading the version of a service, bump the image tag in
   docker-compose.yml.
1. run `mise run deploy`

## Design Philosophy

I have to maintian this in my free time. Things should be stupidly simple. If I
come back after time away and forget everything, it should be difficult to mess
things up and easy to do the right things.

Therefore:

1. This repo should aim to be a monorepo. As much as possible should remain
   inside this repo to avoid introducing dependencies. Only home grown projects
   that are truly worth something in their own right outside of my homelab
   should be moved to an independant repository.
1. All data created/accessed by containers that is not ephemeral (i.e. if it is
   destroyed forever no one cares) should be stored in a subdirectory of ./data
1. ./data will be backed up to an external system
1. No config should be volume mounted for deployments (it is fine for
   development)
1. Config should instead be baked directly into the container if it cannot be
   conveyed through env vars (i.e. nginx conf files)[^1].
1. Deploys should be simple and straightforward. commands should be clearly
   defined and not open ended in a way where not remembering how things work
   can be a footgun, as I may be coming back to manage this after a long time
   away. Rollbacks should be similarly simple.
1. Deploys must run from outside the docker hosts. The hosts should be
   treated like cattle and not pets, require special tending beyond docker
   swarm setup. This includes downloading/syncing this git repo to them or
   managing anything about their filesystems.
1. Deploys must run from outside the docker hosts: this means running from the
   development machine (i.e. my laptop) for now but could mean CI pipeline
   later.

## Footnotes

[^1]: This simplifies deployments; there is no need to create a new docker
    config with a unique name, replace the container's config with the new
    config, and potentially clean up the old config. Simply deploy a new image.
    If rollback is needed, just deploy the previous image. Simplicity is
    derived here from the fact that we already have to solve deploying and
    rolling back to new/previous images.
