---
date: 2021-03-04
title: mCaptcha - to hell with captchas!
tags: ['mCaptcha']
draft: true
---

While working on [Identity](https://github.com/shuttlecraft/identity), I
noticed some of the endpoints were computationally expensive. The target
platform for Shuttlecraft systems are cheap computers such as Raspberry
Pis so I learnt a [new language](https://rust-lang.org) and picked up a
new [web framework](https://actix.rs) among many things, to make things
go faster. And those computationally expensive endpoints were dog slow
and vulnerable to denial-of-service attacks if left unprotected.

I came up with the following ways to rate-limit those endpoints:

1. Captcha: reCaptcha/hCAptcha types
2. IP based filtering: `x` requests per IP address during a specific
   interval of time
3. Do nothing: there's only so much that you can do :p

Let's analyse each one of these solutions.

## 1. Captcha:

### Pros:

- Captcha widgets are the norm for detecting bot activity.
- Large companies like Google and Cloudflare are invested in making captchas work.
- Use machine learning to study bot activity and flag users down
  whenever there's resemblance.
- No user learning curve: netizens are familar with captchas

### Cons:

- **Proprietary software:** what is the backend doing? What sort of
  algorithms are they using? Do they exhibit unfair bias when letting
  users through? We will never know. We have to take them by their word
  and have no way of verifying their claims. They can, in effect, ban
  users from visiting your website if they wanted do.

- **Privacy invasive:** Captchas mine data. I have Gpogle domains
  blacklisted by default and every time I visit a site that uses
  Google's reCaptcha, I am forced to temporarily whitelist them. I don't
  like doing that.

- **Annoying:** It sucks that I have to manually solve captchas to tell
  a website that I am, in fact, a legitimate user. We should have
  automated this stuff a long time ago!

##### {#anecdote} 
> **True story:** My university has a student portal that we use submit
> assignments and view scorecards. They have a dumb captcha at the
> signin page to keep bots away. The IT staff figured they'll be better
> protected if they replace it with reCaptcha. And they did.
>{{< figure src="/img/blog/04-03-21-mcaptcha/portal-dumb-captcha.png"
 caption="dumb captcha university portal" alt="dumb captcha university portal" >}}
>
> I have terrible luck with captcha systems. They always flag me as
> a bot. So after this change, when I went to online to upload an
> assignment, I had to solve 49 captchas _and_ get flagged as a bot and do
> another 49 only to get flagged as a bot again. Only this time, Google
> wouldn't let me try again and asked me to "try again later".
>
> Luckily I wasn't alone. You see, my university is behind a
> [NAT](https://en.wikipedia.org/wiki/Network_address_translation) so
> 10,000 people share a single public IP address. From Google's
> perspective, we would appear as a giant bot that generates a lot
> traffic(stopped using Google search after I moved there because of all
> the captchas that they made me solve).
>
> The next day, the IT team rolled back to the old dumb captchas

## 2. IP based filtering:

### Pros:

- **Easy to implement:** IP addresses are ubiquotous on the internet

- **Stateful:** Unless and until users are hopping between networks,
  they'll have the same IP address throughout their session with the
  site. So it's trivially easy to use it as an ID to rate-limit clients.

### Cons:

There's only one that I could think of:

- **Doesn't work for people behind NAT:** When there are multiple users
  using the same IP address, there isn't an effective way to uniquely
  identify each on of them. So an IP based mechanism would fail in such
  situations.

[See previous section](#anecdote) for anecdotal evidence.

## 3. Don't do anything:

Not really an option.

---

So what we need is something that that will work behind NATs, is privacy
respecting but most importantly, it should be free(as in freedom)

## Enter mCaptcha

mCaptcha is a proof-of-work(PoW) based rate-limiting system that uses
variable difficulty.

When a user visits an mCaptcha enabled website,
their client automatically generates a PoW to qualify for performing an
action(submitting forms, etc.). This enforces a rate-limit.

When server is under load, the difficulty will rise making the clients
spend more time preparing the request than the server processing the
request. This makes DoS attacks expensive for the attackers as well.
