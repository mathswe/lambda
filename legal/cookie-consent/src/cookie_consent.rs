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
        .map(|pref| register_consent(req, pref))
        .map_err(|e|
            Error::from(format!("Fail to read request JSON body: {}", e))
        )
}

fn register_consent(req: Request, pref: CookieConsentPref) -> Response {
    console_log!("Received request: {:?}", req);

    Response::from_json(&pref).unwrap()
}
