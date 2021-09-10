---
title: Ideal backup situation
tags: ['self-hosting', 'rant', 'sysadmin']
date: 2021-09-10
draft: false
---

I've been playing fast and loose with backups, and I've been extremely
fortunate to have not suffered any serious system failures or data
loses. I recently recovered from systems failures on my Raspberry Pis
caused by upgrading Debian Buster to Bullseye. The outage wasn't a
because of a bug in Bullseye, the upgrade modified some parameters
`/boot/firmware/config.txt` which, for some reason, prevented the board
from booting.

Also around the same period, my place was being renovated, and the
construction workers had nicked the main power supply lines. As a
result, we were experiencing some wild voltage fluctuations and power
outages. I thought I burnt my boards from either one of these two
things. It was time to set up a proper backup mechanism.

## Requirement

I have roughly 3 TB of storage across multiple devices and none of these
were being backed up. Going by the
[3-2-1](https://en.wikipedia.org/wiki/Backup#3-2-1_rule), I would
require at least 9 TB + 3 TB of storage(3\*3 + some additional space for
versioned backups --- most backup systems these days are chunk snapshot
systems so versioning is very efficient). But since I'm broke and can't
afford offsite storage, my requirements are down to 6 TB + 2 TB.

## My current solution

I bought two 4 TB disks and setup [Borg](https://www.borgbackup.org/)
backup jobs to run at night. I currently have 7 daily, 4 weekly and 4
monthly snapshots on the first disk and the second disk contains 4
weekly and 4 monthly snapshots, and is unplugged and stored at a
different part of the house.

Ideally, it should be plugged in at a different part of the house, away
from my server rack, but I neither have the hardware nor the
infrastructure to implement that.

## Ideal solution

My current set up will work in most cases. I live in a calm neighborhood
in a very safe city(not prone to natural calamities and such) but
_ideally_ there should be at least one copy of offsite backup. So I did
some math to see how much it would cost me:

### AWS S3

AWS is usually the cheapest and there's probably an adapter available
for Borg

+ **pricing:** [\$0.023/GB-month for the first
  50 TB](https://aws.amazon.com/s3/pricing/)
+ **cost:** $0.023 * 3 * 1024 = $70.656/month

But I don't like AWS or anything Amazon

### Tarsnap

Tarsnap is developed by [Dr. Colin
Percival](https://www.daemonology.net/blog/) and is quite popular.

+ **pricing:** [$0.25/GB-month + $0.25/GB
  bandwidth](https://www.tarsnap.com/)
+ **cost:** $0.25 * 3 * 1024 = $768/month + bandwidth costs

And is also quite expensive!

I can't afford either of these solutions. My current setup set cost me
roughly ₹16,000 or \$220 or the two months worth of AWS rent. Which is
ridiculous. So here's my proposal:

As the internet becomes more diverse and more decentralised, home
servers will become a common thing. This means more folks, like myself,
will be on the lookout for cheap backup and failover solutions.
Normally, these home servers won't be at 100% utilization(I'm currently
at 20% across the fleet) so forming rings of trust among friends and
family will be mutually beneficial. This way, the participants within
the ring can rely on each other's facilities for backups and
failover situations for a fraction of what a cloud-based solution would
cost.

Of course, there should be proper measures taken to prevent unauthorized
access and trespassing, but they are implementation details.
