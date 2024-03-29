+++
title = "Project plans for a hosted Gitea online service"
date = "2022-02-16"
description = "Loïc and I are in the process of creating a 100% libre, managed Gitea hosting service, this post documents some of the challenges that we we are trying to solve"
draft = false

[taxonomies]
tags = ['gitea', 'free-software', 'free-software-sustainability']
+++

_This post was originally published on [Loïc Dachary's
blog](https://blog.dachary.org/2022/02/16/project-plans-for-a-hosted-gitea-online-service/)._

Loïc and I are in the process of creating a 100% libre, managed Gitea
hosting service. Successfully monetizing such a service to a point where
it becomes self-sustainable is a very challenging task. Loïc wrote a
post that documents the challenges a libre, managed hosting service
provider would have to solve and he was kind enough to let me re-publish
it on my blog. Enjoy!

---

When an organization asks me about Gitea, I would like to direct them to
a provider where they can rent an instance and just use it, in the same
way they can go to https://discourse.org for a forum, or
https://nextcloud.com for storage. Instead of waiting for that to
happen, [Aravinth](https://batsense.net/about/) and
[myself](https://dachary.org/) decided to do something about it, in a
way that is in line with our shared values: transparency and Free
Software.

After doing some research we found counter examples that showed the
pitfalls to avoid. GitLab because its business model heavily relies on
selling proprietary licenses. CiviCRM because setting it up is complex
and requires training: users can't figure it out on their own. Gitea
images provided by Digital Ocean because they do not include security
upgrades. MySQL configured and run by AWS because of the vendor lock-in
that makes it impossible to self-host.

We concluded that an online service such as Gitea can be hosted in a
sustainable way as long as:

-   It is well maintained and upgrades itself
-   It can be self-hosted
-   The service can automatically be restored from backups when the
    underlying resources fail

GitHub and GitLab make it look like there is a market around software
forges. It is however impossible to figure out if this market exists
only because it is based on proprietary software. How many of these
customers would pay for a Free Software hosted Gitea instance?

Even if these customers do exist, a new service provider would have to
figure out how to convince them to subscribe. The technical development
of the service can be done within weeks but building a sustainable
business takes much longer. Again, there were examples of what can go
wrong, for instance ElasticSearch. After years of work developing a
successful online service and a customer base, AWS entered the
competition and started to take it away from them.

The sustainability of the Free Software ecosystem is a new and very
difficult problem to solve. It is discussed more than it ever was in the
wake of security breaches originating from widely used and yet abandoned
library or disillusioned Free Software authors self-sabotaging their
next release, and everyone has diverging opinions. It falls on each Free
Software author to spend time to think about their own projects because
there are no handbook or good examples to follow. That is what Aravinth
and myself did to find a semblance of clarity and decide how to go about
this hosted Gitea service idea.

# Sustaining Free Software online services

## More mature online services mean less opportunities to sell services

Ideally the software running an online service is rock solid and bugs
are so rare that it can run unattended. This is true of
https://wordpress.org and it is not uncommon for an instance to run for
years while upgrading themselves to get security patches. The cost of
maintaining such a service is negligible and hosting companies can offer
it for free to their customers. They make their profit by renting the
machines on which the service runs.

When the software is not as mature, it is more expensive to run. Bugs
need fixing, upgrades require manual intervention, users must be trained
to overcome the complexity of the user experience, etc. Well known
examples are Discourse or CiviCRM for which companies sell services to
overcome these issues.

But when an organization is both the author of the software and the
provider of paid services to compensate for its lack of maturity, it
creates a conflict of interest. Should they focus their effort on making
the software more mature, they would harm a business model that is based
on this very lack of maturity. For instance, if the author of a software
also sells training courses, they are not motivated to solve user
experience issues. If they did, it would lower the need for training
courses and reduce their income.

## Free Software online services in the wake of the sustainability

crisis

Nowadays all Free Software authors struggle to get enough resources to
produce a steady stream of releases, even when the project is very
popular. This sustainability problem is getting more and more attention
as the number of Free Software projects in use world wide keeps growing.
Even the simplest online service relies on thousands of Free Software
projects, each of which needs work to keep going. Accidents caused by
poorly maintained Free Software projects become more frequent.

This Free Software sustainability crisis is barely emerging and very
much resembles ecological problems such as climate change. In both cases
it is very difficult to figure out how to properly care for the
resources that are consumed. After decades of advocacy, it is generally
accepted that fossil energy won't last forever but there still is a long
way to go. It will also take a long time for the Free Software community
to answer this simple question: how to sustain an ever growing library
of Free Software?

Luckily, it is relatively simpler to solve that problem for an online
service because it has users. They can be reminded that their assistance
is needed to keep the project afloat, for instance by a donation. A
proposition that would be much more difficult to make for the author of
a cryptographic library. Convincing users to pay for an online service
has the best chance of success when the author of the software is also
the service provider. This is the business model of Discourse and
Weblate, but it is relatively fragile because nothing stands in the way
of the competition.

A few years ago ElasticSearch successfully developed an online service
offering. But when AWS entered the competition and was better at
marketing it, ElasticSearch quickly realized they would likely go out of
business. They tried to fight back by [changing their
license](https://www.elastic.co/blog/licensing-change), which was the
wrong answer to a real problem. Discourse or Weblate are also likely to
face competition from hosting companies in the future and they may not
survive it.

In the end, the durable source of income for a Free Software online
service is to rent the resources (CPU/RAM/network/disk) it needs to run.
In other words only hosting companies can make a profit when running
such an online service. And for that reason they also need to share part
of the profits to ensure the sustainability of the Free Software service
their customers need.

# Online services vendor lock-in is cured by Free Software

When hosting companies offer online services they also provide upgrades
and transparent recovery when the hardware fails. But none of them allow
the service to be self-hosted. When their price policy change, or when
the term of services ban users from a given country, migrating the
service elsewhere it costly and difficult. For instance when AWS runs
MySQL for their customers, they allow to download the data but not the
software that runs the proprietary AWS interface used to configure and
control the server. Another example is GitHub where the content of the
git repository can be downloaded but the code that runs GitHub itself is
not Free Software.

If a customer cannot run the same software as their service provider,
they are locked-in, even if they can download their data. It is a common
misconception to think that there is no vendor lock-in as long as it is
possible to download the data in an standard format. Migrating the data
from one software to another is, more often than not, time consuming and
costly to a point that it is effectively a blocker. A GitHub salesperson
would argue that it is possible for people to run GitHub Enterprise on
their own hardware. But the vendor lock-in is still present via the
proprietary license contract. The user experience, maintenance and
upgrades are still exclusively controlled by GitHub.

To guarantee their independence, the customers of an online service need
to be able to:

-   Download their data
-   Run the exact same Free Software as their service provider
-   Run the exact same Free Software infrastructure as code as their
    service provider

The requirement regarding Free Software infrastructure as code refers
to, for instance, the AWS control panel and all that is behind it when
creating a new MySQL service. It includes whatever a competitor would
need to run the same online service. An example would be
https://enough.community, an infrastructure as code dedicated to
creating the services needed by whistleblowers and human rights
defenders. It consumes resources rented by hosting providers, assembles
disks and machines, setup monitoring and intrusion detection, installs
various online services and upgrades them.

The availability of the software that creates the infrastructure is not
only useful to the competitors of a service provider. It also benefits a
non-profit that wants to provide (for instance) Wordpress instances to
its members. Without it they would need to create something from scratch
using building blocks such as CiviCRM. Even though such building blocks
exist, this is a significant undertaking and effectively a blocker.

# Federated online services and durability

All self-hosted services are in danger of loosing the data they contain.
When a Wordpress service is hosted in a home and the machine dies, it
must be restored from backups... when there are backups. Hosting
companies ensure the durability of the data with their own backup
system. It creates a dilemma for people who are looking into self
hosting: independence is desirable, but is it worth taking the risk of
data loss?

Federated online services do not suffer from this problem, because they
can mirror each other. A Gitea instance that is federated with another
will mirror copies of software projects found on its peers. Should one
instance be destroyed, mirrored projects can be resurrected from the
federated instance. Not only is it a practical way to ensure the (rare)
failure of an entire datacenter, it also helps with the (more frequent)
destruction of self-hosted machines. Contrary to backups that require
special attention, the replication involved in federated online service
is built in and works continuously. There is no need for an extra backup
service that is very rarely used and therefore likely to fail when
needed.

Federated services are not yet mainstream and Gitea is one of the rare
services that started to implement the concept. In the interim,
customers of an online hosting service will need to worry about backups
to ensure the durability of their data. But the ultimate solution for
them won't be the emergence of an ideal backup infrastructure, it will
be replication (via federated services) that will continuously ensure
the durability of their data.

# Paths forward

The Gitea project itself, following the footsteps of Discourse or
Weblate, could provide a hosting service. Part of its current user base
may become customers and there does not seem to be any blocker to make
that happen. As with most successful Free Software project, people
working on Gitea daily are already very busy and cannot engage in such a
long term project. But Aravinth and myself can, if they will have us.

Another path forward would be to wrap Gitea into a bundle that existing
hosting companies could easily use to provide such a service to their
customers. The biggest hosting companies are unlikely to be interested:
if Digital Ocean was to provide upgrades on top of their existing Gitea
image, they are more likely to rely on their internal staff to implement
that from scratch, as proprietary software integrated into their
existing infrastructure. But smaller hosting companies such as
https://Octopuce.fr or https://Easter-Eggs.com, who already deploy Gitea
instances for their customers, would use it if, for instance, it helped
with the upgrades. They would then kindly be reminded to give back a
share of their profits in order to sustain the development of the
service they deploy.

Finally it would also be possible to follow the example of GitLab in the
early days (before it turned to proprietary software) or Codeberg and
offer a free shared forge hosting service to build a user base. After a
few years, a percentage of the user base would convert to being paid
customers or donors to sustain the activity and part of the income would
be used to sustain the development of the service.
