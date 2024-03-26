# Cookie Consent

It processes the MathSwe cookie consent operations such as storing consent
records.

## Getting Started

Run `npx wrangler dev` for development. If you need more debugging information,
you can run it like `npx wrangler dev --log-level debug`.

## Deployment

The microservice is deployed to Cloudflare Workers and KV.

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

