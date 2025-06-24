# homelab (ryangeary.dev)

This is my homelab, aka personal infrastructure I run at home. It is very WIP.

## Architecture (abridged)

1. cloudflare tunnel to connect a docker network to the public internet.
1. nginx docker container on the same docker network to receive and route connections.
1. nginx is also on an additional network, where services run.
1. smallweb is on this additional network and handles subdomains not handled by
   specific nginx configurations.
1. all docker containers are intended to be run on a Raspberry Pi (WIP).

### Smallweb

[Smallweb](https://www.smallweb.run) is a newer project that allows deploying
some minimal websites fairly easily.

#### Smallweb external repo sites

I've added some sugar on top of smallweb to pull in external repos I want to
deploy through smallweb. See the `www` subdomain for an example. This requires:
1. an external repo which publishes a `dist.tar.gz` as a GitHub release.
2. a `smallweb.json` which specifies a `"root": "./dist"` and a `cron` to call
   `fetch_github_release` on some cadence.
