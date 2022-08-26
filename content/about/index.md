---
title: "About"
draft: false
---

Hello, I'm Aravinth.

I find computers fascinating and making them do a specific thing in a
specific manner even more fascinating. So I spend most of my time
building and breaking software.

## Projects that I'm currently working on

-   [mCaptcha](https://mcaptcha.org): a privacy focused, [proof of work
    based](https://en.wikipedia.org/wiki/Proof_of_work) libre CAPTCHA
    system.

    One of my biggest gripes with existing CAPTCHA systems is that they
    incorrectly judge a visitor using TOR or has their cookies turned on
    to be a bot. I wanted to create something that could effectively
    ratelimt bots while still allowing users with tracking
    countermeasures enabled. And I think mCaptcha is effective in that
    regard.

    Also the World Wide Web Consortium reports [that existing CAPTCHA
    systems pose accessibility
    problems](https://www.w3.org/TR/turingtest/) to users with special
    needs. The proof of work algorithm that mCaptcha uses requires minimal
    user interaction as it only requires the click of a button!

-   [ForgeFlux](https://forgeflux.org): an attempt at [software
    forge](<https://en.wikipedia.org/wiki/Forge_(software)>) federation.

    The original [ForgeFed](https://forgefed.peers.community/)
    required involvement from forge developers themselves to enable
    federation. While it's a reasonable approach, such an endeavor from
    the forge developers' side will require massive commitment. Our
    approach is different as we are trying to implement federation
    entirely using the APIs exposed by the software forges. If
    successful, the lessons learnt from
    [ForgeFlux](https://forgeflux.org) can be used to externally enable
    federation in a variety of services.

-   [Hostea](https://hostea.org): A 100% Free software forge suite with
    [Gitea](https://gitea.io) for the forge, [Woodpecker
    CI](https://woodpecker-ci.org) for CI/CD,
    [Pages](https://github.com/realaravinth/pages) for static sites and
    [GitPad](https://gitpad.org) for gists.

    [Loïc Dachary](https://dachary.org) and I wanted to create a 100%
    Free business, based on the idea [discussed
    here](../blog/16-02-2022-gitea-managed-hosting-plans.md), we have
    now dropped the business part and are now working towards a
    reliable, self-hostable software development suite, with a
    payments processor, that may optionally be used to manage users.

-   [kavasam](https://github.com/kavasam): Privacy-focused phone and
    email spam protection. In India,
    [Truecaller](https://www.truecaller.com/) is the most popular method
    to block spam phone calls. While effective, it uploads its users'
    entire phone book to its servers. This has the undesirable effect of
    creepy phone calls from stalkers(I uninstalled Truecaller in 2017,
    my experience might be outdated).

    Kavasam's approach is to use [cryptographic hash
    functions](https://en.wikipedia.org/wiki/Cryptographic_hash_function)
    and [digital
    signatures](https://en.wikipedia.org/wiki/Digital_signature) to both
    anonymously blacklist an identifier(phone number, email ID, etc) and
    anonymously file the blacklist report. And it employs a
    peer-validation mechanism for effective blacklisting: most spam
    calls are the effect of database leaks or irresponsible service
    providers selling their customers' contacts. Usually, spammers work
    through the contents of the database in their spam campaign. So if
    Alice, a victim of a campaign, blacklists the spam calls she
    receives and Bob, a victim of the same campaign, has noticed Alice's
    blacklist reports against the spam calls he received, can instruct
    Kavasam to implicitly block phone calls based on Alice's reports,
    both future and current.

## Available for hire

I'm passionate about decentralisation and digital privacy therefore, I
hope to make a career out of free software. If you are a free software
company and like what I'm doing, please consider hiring me. My résumé is
attached below and my contacts are available [here](/contact).

**- Résumé:** [click here to download](/realaravinth-resume.pdf)
