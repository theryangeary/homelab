# homelab (ryangeary.dev)

This is my homelab, aka personal infrastructure I run at home. It is very WIP.

It uses ansible to set up infrastructure and deploy services. It assumes a host with ssh and ssh keys for a user with sudo privileges.

## Architecture (abridged)

1. cloudflare tunnel to connect a docker network to the public internet.
1. caddy connected to the cloudflared container, to reverse proxy to public services.
1. another, separate caddy_internal which connects to private (non-internet exposed) services.

## Deploy

### on your development machine (used to control deployments):
1. set up bitwarden secrets with ` export BWS_ACCESS_TOKEN=<blah>` ensuring the leading space, to prevent keeping in shell history.
1. activate the ansible venv
1. `cd ansible`
1. run `ansible-playbook playbooks/site.yml` (optionally specify a --inventory or --tags)
