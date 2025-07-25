---
title: "Why Oh Why Am I Starting a Homelab"
date: "2025-08-01"
tags: ["homelab", "fly.io"]
excerpt: "After evaluating a handful of options for free-tier and cheap cloud hosting, I'm foraying into the wacky world of self-hosting."
---

I feel like homelab gets a bad wrap from people with more hardware on their
hands than they know what to do with building overpowered systems with no real
usecase. That can be cool and fun, but personally I really try to keep the
things in my life that require maintenance to a minimum.

Which begs the question, why *am* I doing this?

## Using Cloud Services

Once again, I come to you with a problem I have been thinking about recently.
I launched [pathfinder](https://pathfinder.prof) a few weeks ago (please play
it and let me know what you think), and used [fly.io](https://fly.io) to do
it. It uses a not particularly resilient single-node Postgres instance, and
requires a slightly cursed configuration. I have two service groups:
  - one which is always on but constrained to only ever have one instance
  - and another which can scale to 0 or more instances

The value of this setup is that the first service runs my cron jobs. Both
services handle API traffic. This prevents having to pay the price of
slow responses when the app is scaled to 0 AND having to pay for two
machines being on all month. The significance of this is that with a single
node for Postgres and a single node for the API server, my bill should remain
under $5, which means fly.io simply won't charge me for my usage that month,
and my bill resets to $0.

**This means I only have to pay if I scale up, which is really a good problem
to have (i.e. it means I have USERS)!** TBD on if that will happen...

## Evaluating Other Hosting Options

So that got me thinking - if I want to build more prototypes or widgets or
whatevers, I'm not going to be able to host them free on fly.io. Which leaves
me with a few options:

1. Commit to fly.io and simply pay real money for things that will very likely have 0 ROI, effectively flushing money down the toilet,
2. Attempt to find free tier solutions on other hosting platforms, or
3. Host things on my own hardware.

Regarding option 1, I don't feel like paying a bunch of money to fly.io to
host some sites/apis that no one but me will ever use. If I ever find myself
with something that has become successful and I need more stable hosting with
better scaling than option 3, I would definitely consider deploying on fly.io,
as it is smooth, polished, and easy to use.

For option 2, using free tier solutions is interesting but I'm not aware of
options that don't have the same pitfall as fly.io: eventually I'll have to
move on to another solution, and that means learning some new
tools/processes[^1]. This can be cool/fun, but I'm trying to "use boring
technology" more, and by doing so free myself up to focus on building whatever
the core technology/value of a particular project is. So I'm not wild about
option 2.

So that leaves us with option 3. Host myself. This has the obvious pitfall of
having to own hardware and pay for power usage (and 1 million other little
things, which is the real value of paying someone else for hosting). But I own
a Raspberry Pi 3. I don't remember why... but I have it, it's sitting in a
closet, and it can consume at most low-single-digits watts. So the financial
cost is pretty negligent.

## So I'm Doing the Homelab Thing

Mainly I want to host a handful of websites and services, some public (like
this site) and some private. I hope to write some future posts about my
explorations, so you'll get to hear about what I get up to.

The details of my setup so far will be in my next post!

[^1]: One honorable mention goes to cloudflare pages, which allows 100 projects,
20,000 files per project, and 500 builds a month. It's not totally unlimited
but it is pretty hard to imagine having the time as an individual to build
that many projects! The downside of this is it is purely for frontend/static
sites, which is pretty limiting. I've considered doing a hybrid homelab with
cloudflare pages serving frontend, but for now I'm opting for the much simpler
all-in-one solution.
