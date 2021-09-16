---
title: NAT Troubles or how my ISP screwed me over
tags: ['NAT', 'frp', 'self-hosting', 'rant', 'ACT Fibernet']
date: 2021-07-18
description: "NATs are a pain and I just got put behind one"
draft: false
---

Couple of days ago, I woke with my inbox filled with alerts from my
[monitoring service](https://stats.uptimerobot.com/EQ7VJHWylx), the
dashboard was lit up like a Christmas tree. I'm not new to having
downtimes, but this was the most unusual one: the servers were alright,
the internet connection was fine, but the services were
inaccessible. I had a bad feeling about this one.

![Inbox full of alerts](/img/blog/18-07-21-NAT-troubles/inbox-nightmare.png)

You see, most of my internet presence comes out of my bedroom. This
website, my search engine, and even [my favourite instant
messaging platform](https://matrix.org) is operated right out of my
bedroom. This has been possible because I've had a publicly accessible
IP address. It's dynamically allocated but thanks to
[DDNS](https://en.wikipedia.org/wiki/Dynamic_DNS), it works! Everything
was fine until my ISP decided to put me behind a NAT. I told them that
I'm okay with paying for a static IP, but they said they only offer
static IPs for enterprise accounts. I love my ISP, I think they are
great but a static IP costs ₹200/month(~\$3) only, but they want me to
upgrade to a plan that costs four times more than what I'm paying now.

I can't buy an enterprise connection and I can't migrate the fleet to
the could because it's expensive, and my parents will disown me. The only
logical option is to switch to an ISP that offers static IPs. But in the
meantime I have set up a tunnel between my servers and a cheap
DigitalOcean box using
[`frp`](https://github.com/fatedier/frp#rewriting-the-http-host-header).

It's only been a few hours since the tunnel was set up but so far,
everything seems to be okay. There's a client and a server component,
the client is run on machines behind NAT and the server on a machine
that's exposed to the internet. The client connects to the server and
sets up a reverse tunnel. They support a wide range of protocols
including `HTTP`, `HTTPS`, `UDP` and `TCP`. Compared to my previous
setup, performance is a little
sluggish, but that's expected since the application now sits behind
two reverse proxies instead of one. But it's usable.

Here's an example `frpc`(`frp` client) configuration for `HTTP` and
`HTTPS`:

```ini
[libreddit-https]
type = https
local_ip = 127.0.0.1
local_port = 443
use_encryption = true
use_compression = true
host_header_rewrite = libreddit.batsense.net
custom_domains = libreddit.batsense.net

[libreddit]
type = http
local_ip = 127.0.0.1
local_port = 80
use_encryption = true
use_compression = true
host_header_rewrite = libreddit.batsense.net
custom_domains = libreddit.batsense.net
```

Hopefully `IPv6` deployment increases and this post is rendered useless
but if there's someone out there who's trying to get past the NAT and
requires assistance, feel free to contact me!
