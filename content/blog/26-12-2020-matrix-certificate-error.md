+++
title = "My Matrix (mis)adventures" 
date = "2020-12-22"
description = "Keep it simple, stupid. Because there are many like me out there :)"
draft = false
[taxonomies]
tags = [ "matrix", "KISS", "sysadmin" ]
+++

Matrix is probably the largest federated network in production (Mastadon
might be close). I run an instance at https://matrix.batsense.net and I
absolutely love it. It was also the project that got me into the whole
decentralization thing. I loved the idea that I could run a server that
can play along with a bunch of other servers that are run by different
parties and they would all somehow magically work!

## Error
Deployment was fairly straight forward, the project's README
files were sufficiently detailed but after about six months of running
the server, my instance broke! The federation was no longer working. I
could still chat with people that are on my own server but I could
interact with rooms on other servers or even on matrix.org. 


The initial error massages were vague. The client complained that it had
hit some JSON related error. So I went to look for solutions on the
project's issue tracker. There was an open issue that said that when the
person started doing IPv6 DNS lookups, their instance broke. This was
also the time when my ISP moved us to IPv6 lookups so I figured that was
what was causing problems. At this time, I wasn't really using matrix
that much so it didn't bother me. I figured, eventually, massive project
such as matrix might roll out a patch. Several weeks and several updates
later, the issue still persisted. So I tried my hands at it once again. 

## Diagnosis
See, decentralization is complicated. Some matrix dev on Hackernews
remarked that building a decentralized system is six times harder than
building a centralized system. The bug that I was experiencing, wasn't
really a bug. I misdiagnosed it to be one in the first place. I use
Let's encrypt for TLS and their certs are valid for a maximum of three
months. So when I hit that "bug", I was through two cycles and was
beginning the third. Synapse, the matrix homeserver variant that I run,
listens on two ports: one for regular client-server interactions and the
other for federation. The client-server port is proxied by an Nginx
instance so all TLS stuff is delegated by the Nginx instance. The
federation port, however, is directly exposed and all TLS stuff is
handled by Synapse itself. That means Synapse is configured to read and
do TLS stuff with the TLS certs. That means, my stupid self has two
copies of the same TLS cert at two places. Now you could quickly see how
stupid this is and the problems that it could cause. During the third
cycle, I didn't refresh Synapse's copy of the certs. It was using stale
certs. TLS is mandatory for federation in matrix. So naturally, my
federation was broken!

## Lessons learned

- I am stupid
- Maintain sysadmin notes to record all quirks of the systems that you
  administer
- Build simpler services: The more the number of levers to pull on, the
  more likely someone is going to screw up. The second port for
  federation feels necessary. It is a HTTP endpoint so I don't see why
  it needs to be separated. It could've simply be scoped differently.
  That way, the maintenance and deployment complexity can be
  significantly reduced. In this case, firewall rules can become simpler
  and the federation stuff will be handled by the Nginx instance itself. 


But seriously, I should have maintained notes. Three months  without
federation over a stupid error is nuts.
