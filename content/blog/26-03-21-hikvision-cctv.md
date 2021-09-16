---
title: "Home defense adventures with Hikvision CCTV cameras"
tags: [ "home-defense", "hikvision", "cctv", "nginx" ]
date: 2021-03-26
description: "There's nothing like an undocumented plug-this-to-the-internet-and-well-spy-on-you CCTV system!"
draft: false
---

We recently got robbed, so we decided to secure the house with some CCTV
cameras. Ours is a modest setup with nine 5MP cameras and a DVR
recording all the footage from those cameras at 1080p. The DVR can
connect to Hikvision's cloud infrastructure so that owners can see live
footage using the internet., which I don't like. I mean, in the past few
years, I've moved all my online life from Big Brothers to my servers for
privacy reasons, so giving free access to live footage of my house to
companies is certainly not acceptable.

So I decided to set up the DVR in an isolated network(without access to
the internet), expose its web interface(the DVR has a web interface) via
a reverse proxy to the internet. This post is an attempt to record my
findings. 


I'm using Nginx for reverse proxying. Setting up the reverse proxy
wasn't straightforward. The web interface has many quirks and no
documentation.


## First Attempt: simple reverse proxy

```
server {
    server_name example.com;
    location / {
    	proxy_pass http://dvr-ip-address;
    }
   listen 443 ssl;
   listen 80;
--- snip ---
}
```

This setup exposed the web interface, I was able to log in, but video
playback wasn't working. It would work when I directly access
`http://dvr-ip-address` but not through the reverse proxy.

I poked around with browser dev tools for a bit and learned that video
was handled by WebSocket connections. And so:

## Second Attempt: there's a WebSocket component!

```
server {
    server_name example.com;
    
    # some static files were taking forever to download
    # so I hooked it up with a cache
    proxy_cache hikvision;
    location / {
        proxy_pass http://dvr-ip-address;
        
        proxy_http_version 1.1;
        # Ensuring it can use websockets
        proxy_set_header   Upgrade $http_upgrade;
        proxy_set_header   Connection "upgrade";
        proxy_redirect     http:// $scheme://;
        
        # These sets the timeout so that the websocket can stay alive
        proxy_connect_timeout 7m;
        proxy_send_timeout 7m;
        proxy_read_timeout 7m;
    }
    
    listen 443 ssl;
	listen 80;

}
```

Some files were taking forever to download, so I hooked it up with a
cache to speed things up. Video playback wouldn't work even with these
modifications. So I had to dig deeper with the browser dev tools. 

Server ports are hardcoded in the frontend application. The DVR system
listens on four ports:
- WebSocket server for video playback (with and without TLS, separate
  ports)
- controls, authentication, etc. (with and without TLS, dedicated ports)

Instead of having the back-end handle HTTP to HTTPS redirection, the
ports are hardcoded in the frontend. The TLS WebSocket server
listens on `7682` while the plain text WebSocket server listens on
`7681`. And the frontend, depending on the URI scheme, toggles between
the ports. The reverse proxy had TLS configured, so it was trying to
contact `wss://example.com:7682`.


## Third and Final attempt: work already :/

```
# server handling configuration, authentication, etc.
server {
    server_name example.com;
    
    proxy_cache hikvision;
    location / {
        proxy_pass http://dvr-ip-address;
    }
    
    listen 443 ssl; # managed by Certbot
    listen 80;

}

# websocket server
server {
    server_name example.com;
    listen 7682 ssl;
    
    location / {
        # nginx talks to backend in plain text
        # and hence 7682-7681 plumbing
        proxy_pass http://dvr-ip-address:7681;
        
        proxy_http_version 1.1;
        # Ensuring it can use websockets
        proxy_set_header   Upgrade $http_upgrade;
        proxy_set_header   Connection "upgrade";
        proxy_redirect     http:// $scheme://;
        
        # These sets the timeout so that the websocket can stay alive
        proxy_connect_timeout 7m;
        proxy_send_timeout 7m;
        proxy_read_timeout 7m;
    }
    
}
```
I opened port `7682` on the reverse proxy to
and forward it to port `7681` on the back-end for video playback and it
works!


## Conclusion
There are a lot of weird things about the Hikvision DVR's web interface.
There was an SSH toggle switch that wouldn't work(I dug up its manual on
the internet, apparently my model doesn't have SSH). The root is not at
`/` but they have a redirect to `/doc/something`, which is weird.
There's a separate WebSocket server that listens on a separate port when
they could have simply scoped it to a certain path on the main server.
The web interface is practically useless on mobile devices: video
decoding is done client-side with WASM. So mobile devices struggle to
process feeds from two cameras simultaneously. I understand that these
systems are designed to Just Work&trade; but software design should be
intuitive and it certainly shouldn't take a whole day to set up a simple
reverse proxy!
