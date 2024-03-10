// Copyright (c) 2024 Tobias Briones. All rights reserved.
// This file is part of https://github.com/mathswe/lambda

use std::fmt::Display;
use worker::{console_log, Error, Request, Response, RouteContext};

use crate::consent::{CookieConsent, CookieConsentPref};

pub async fn post_consent_pref(
    mut req: Request,
    ctx: RouteContext<()>,
) -> Result<Response, Error> {
    let json = req.json::<CookieConsentPref>().await;

    match json {
        Ok(pref) => register_consent(ctx, pref).await,
        Err(e) => Response::error(format!("Invalid JSON body: {}", e), 400),
    }
}

async fn register_consent(
    ctx: RouteContext<()>,
    pref: CookieConsentPref,
) -> Result<Response, Error> {
    let cookie_consent_kv = "COOKIE_CONSENT";
    let consent = CookieConsent::new(pref);
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
