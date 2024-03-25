# Cookie Consent

It processes the MathSwe cookie consent operations such as storing consent
records.

## Deployment

The microservice is deployed to Cloudflare Workers and KV.

### Deploying to Staging

Run `npx wrangler deploy -e staging` to test the service in the staging
environment.

The service will be available
at https://mathswe-cookie-consent-staging.tobiasbriones-dev.workers.dev.
