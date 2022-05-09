+++
title = "Quitting non-free services and search for blood donor: idea for an MxN contact discovery system"
date = "2022-05-09"
description = "I use a few non-free services, mostly instant messaging services to talk to my friends and acquaintances, but had plans to 'sunset' them by the end of the year. I had to search for blood donors for a friend recently and I'm not sure how I would have done without said services. This post proposes a system to discover a person's alternate means of contact from an existing means of contact."
draft = false

[taxonomies]
tags = ['privacy', 'free-software', 'ideas']
+++

## Background

I use a few non-free services, mostly instant messaging services to talk
to my friends and acquaintances, but had plans to 'sunset' them by the
end of the year. I was going to make a public post or the equivalent for
`$service`, without spamming my whole network, explain my motivation and
advertise libre means of contacts that they can use to reach me. I had a
plan for every `$service`, and I was eagerly waiting to execute them. But
recently I had to search for blood donors for a friend and my search
exclusively used a popular non-free service. I'm not sure if the search
would have been as effective if I had quite though.

The service is popular among us Indians and is run by a not-so-nice
`$BigCorp` but since it is available free of cost, most people are willing
to overlook the fact that it is harmful in the long run. It is one of
the services that I have been trying to get rid of for the longest time;
I've been running it in a sandboxed environment, using [Shelter] (TODO:
get shelter's link), without exposing my contacts database and main file
system to minimize damage. And I was all set to gradually phaseout
and get off of the platform but the recent experience of contacting
multiple people for an urgent cause suggests it will be more difficult
than I had anticipated.

## Problem

If I had to mount a similar search without the non-free instant
messaging `$service`, I would have to ring each of the people that I
messaged, which is more time confusing and requires more effort than
simply forwarding and coordinating with multiple parties par rally via
text messages. Also, the phone call process would require both parties,
my friend and I, to be available at the same time. My friends are more
busier than I am, so the search effort would have been bogged down and
significantly less effective than the instant message approach.

In my opinion, this is problem is much more fundamental than quitting a
platform. How do I contact someone who is on a platform that I don't have
access to or am unwilling to use? Why do platforms, especially the
social kind, impose these barriers implicitly? And how to break them
without screwing up security or ruining user experience for all involved
platforms?

For the web, we have RSS: anyone and everyone can subscribe to websites
that implement RSS feeds, like my blog [here](/blog/atom.xml), and
receive updates whenever a new article is published. The subscriber
isn't required to negotiate with the publisher, all they have to do is
periodically check the feed for new entries, which will automatically be
done for them by [their feed reader of choice] ( TODO: get archlinux
feed reader list)

## The real problem

We have emails and other free, federating alternatives like Matrix and
[the array of Fedi apps](https://fedi.party) but they aren't just as
widely adopted as the popular non-free service is. But that isn't the
problem. _How does one discover alternate ways to contact a person?_

## MxN contact discovery system

I propose a system where a person is able to search for alternate
contacts on alternate networks using any prior contact that they might
have. So if I have a person's Fedi address, I could use it to search for
the person's, say, phone number or even their Matrix ID if they have
one. This way, I get to choose an alternate means of contact that we
both share or agree to use without directly negotiating with them. I
mean, it's first contact, how does one negotiate even before
establishing a means of communication?

The system would store two hashes of every contact identifier: one a
strong, untraversable hash and the other a weak hash using a proved
colliding hashing algorithm like MD5 or SHA-1. The strong hash will be
used to uniquely search for a related contact identifiers' weak hashes,
which will then have to be cracked by the searching party to retrieve
the underlying contact identifier. The contact identifier can carry
metadata like which network it belongs.

### Hashed contacts or how to safely advertise contacts publicly

The hash serves multiple purposes:

#### 1. Combat Spam

Publicly accessible, freely distributed contacts databases are a
spammer's wet dream. A system like this shouldn't hold monopoly over
who/what a person uses to establish communication. But to securely share
contacts publicly, we need a method to restrict spammer/whole database
access without compromising free distribution.

The weak hash generating using collision-prone algorithms like MD5 and
SHA-1 will serve as an unconventional Proof-of-Work mechanism: the user
will have to crack the hash to retrieve a contact. A spammer would have
to do a lot of work cracking the hashes to mount a successful spam
operation but a person that is interested in only single person's
contact will have to spend a only comparatively reasonable amount of
time to retrieve the contact.

##### 2. No trusted entity:

In centralized systems, the service provider will act as a trusted
entity and ensure secrecy of the information they host. This has the
unfortunate effect of monopolizing the data hosted by them. A competing
service would have to start from scratch and aggregate data, which if
done with consent(as it should be), would require convincing people to
start using the service.

The hash-based system doesn't require secrecy
and data aggregated can be freely shared with other providers to create
a sustainable, federated/peer-to-peer network.

---

My schedule at the moment is completely packed but if someone is
interested in implementing this system, they are most welcome to do it.
My contacts are available [here](/contacts) if someone wants to
brainstorm ideas :)
