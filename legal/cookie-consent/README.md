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

### Deploying to Production

Run `npx wrangler deploy` to deploy the service to the production environment.

The service will be deployed
to https://mathswe-cookie-consent.tobiasbriones-dev.workers.dev.
