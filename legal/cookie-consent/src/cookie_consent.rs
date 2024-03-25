// Copyright (c) 2024 Tobias Briones. All rights reserved.
// This file is part of https://github.com/mathswe/lambda

use std::fmt::Display;

use worker::{console_log, Error, Request, Response, RouteContext};

use crate::consent::{CookieConsent, CookieConsentClientRequest};
use crate::geolocation::Geolocation;

pub async fn post_consent_pref(
    mut req: Request,
    ctx: RouteContext<()>,
) -> Result<Response, Error> {
    let json = req.json::<CookieConsentClientRequest>().await;
    let geolocation = Geolocation::from_req(req);

    match json {
        Ok(req) => register_consent(ctx, req, geolocation).await,
        Err(e) => Response::error(format!("Invalid JSON body: {}", e), 400),
    }
}

async fn register_consent(
    ctx: RouteContext<()>,
    user_req: CookieConsentClientRequest,
    geolocation: Geolocation,
) -> Result<Response, Error> {
    let cookie_consent_kv = "COOKIE_CONSENT";
    let consent = CookieConsent::from_client_req(user_req, geolocation);
    let (id, value) = consent.to_kv();

    ctx
        .kv(cookie_consent_kv)?
        .put(&id, value)?
        .execute()
        .await
        .map_or_else(
            |e| internal_error("Fail to store cookie consent", e),
            |_| Response::ok(consent.to_json()),
        )
}

fn internal_error(msg: impl Into<String>, error: impl Display) -> Result<Response, Error> {
    console_log!("{}", format!("{}", error));
    Response::error(msg, 500)
}
