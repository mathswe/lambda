// Copyright (c) 2024 Tobias Briones. All rights reserved.
// This file is part of https://github.com/mathswe/lambda

use worker::{console_log, Error, Request, Response, RouteContext};

use crate::consent::CookieConsentPref;

pub async fn post_consent_pref(
    mut req: Request,
    _ctx: RouteContext<()>,
) -> Result<Response, Error> {
    req
        .json::<CookieConsentPref>()
        .await
        .map_or_else(
            |e| Response::error(format!("Invalid JSON body: {}", e), 400),
            |pref| register_consent(req, pref),
        )
}

fn register_consent(req: Request, pref: CookieConsentPref) -> Result<Response, Error> {
    console_log!("Received request: {:?}", req);

    Ok(Response::from_json(&pref).unwrap())
}
