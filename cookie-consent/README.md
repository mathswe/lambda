<!-- Copyright (c) 2024 Tobias Briones. All rights reserved. -->
<!-- This file is part of https://github.com/mathswe/legal -->

# Cookie Consent

It processes the MathSwe cookie consent operations such as storing consent
records.

## Getting Started

Run `npx wrangler dev -e=local` for development. If you need more debugging
information, you can run it like `npx wrangler dev -e=local --log-level debug`.

## Deployment

The microservice is deployed to Cloudflare Workers and requires KV (Key Value)
storage.

### Deploying to Staging

Run `npx wrangler deploy -e staging` to test the service in the staging
environment.

The service will be available
at https://mathswe-cookie-consent-staging.tobiasbriones-dev.workers.dev.

### Deploying to Production

Run `npx wrangler deploy` to deploy the service to the production environment.

The service will be deployed
to https://mathswe-cookie-consent.tobiasbriones-dev.workers.dev.

## Cookie Consent

The cookie consent service processes the MathSwe requests when a user sets their
consent via the cookie banner or preference. Incoming requests are allowed from
domains like mathswe.com, math.software, and mathsoftware.engineer.

### Consent

The consent types are defined in [consent.rs](src/consent.rs), where:

- `Domain`: Defines the accepted MathSwe domains that can sent cookie consent
  requests. For example, the `MathSweCom` or `MathSoftware` sites can request
  consents from their cookie banner.
- `CookieConsentPref`: Defines the consent for each of the cookie categories,
  such as `essential`, `functional`, `analytics`, and `targeting`.
- `CookieConsentValue`: Defines the value or payload that a registered consent
  has. It includes the relevant information like:
    - `Domain`.
    - `CookieConsentPref`.
    - `DateTime<Utc>`.
    - `Geolocation`.
    - `AnonymousIpv4`.
    - User Agent.
- `CookieConsent`: Defines a registered cookie consent. A registered consent was
  already processed by MathSwe, and thus has a unique consent id. It consists
  of:
    - Id.
    - `CookieConsentValue`.

### Register Consent

Provides a `POST` endpoint to request a new cookie consent. It is called right
after the user gave consent from the cookie banner or preference to store the
record correctly.

| Path | Method | Body                | Response        |
|------|--------|---------------------|-----------------|
| `/`  | `POST` | `CookieConsentPref` | `CookieConsent` |

It returns the consent created, so the client can confirm the operation and
store it in cookies to let the user know their current consent information, such
as consent ID and preferences.

#### Cookie Consent Preference

It defines the type of body in the client needs to send for processing a
requesting consent.

For example:

```json
{
    "essential": true,
    "functional": true,
    "analytics": true,
    "targeting": true
}
```

The `CookieConsentPref` defines the body the client sends for registering a
consent. The rest of the values required for registering the consent are taken
form the HTTP request in the server.

### Allowed Domains

Only valid MathSwe `Origin`s are allowed for performing requests.

Valid origins are https://mathswe.com, https://math.software, and
https://mathsoftware.engineer, including all their subdomains.

Requests from unauthorized origins are forbidden, so the response will be
`403`. The only exception is when the app runs in development with `local` mode.

## About

**Cookie Consent | MathSwe Legal**

It processes the MathSwe cookie consent operations such as storing consent
records.

Copyright © 2024 Tobias Briones. All rights reserved.
