// Copyright (c) 2024 Tobias Briones. All rights reserved.
// This file is part of https://github.com/mathswe/lambda

use std::fmt::Display;
use std::net::Ipv4Addr;
use std::str::FromStr;

use worker::{console_log, Cors, Error, Method, Request, Response, RouteContext};

use crate::anonymous_ip::AnonymousIpv4;
use crate::client_req::Origin;
use crate::consent::{CookieConsent, CookieConsentPref, Domain};
use crate::consent::Domain::MathSweCom;
use crate::geolocation::Geolocation;

pub async fn post_consent(
    mut req: Request,
    ctx: RouteContext<()>,
) -> Result<Response, Error> {
    let origin = Origin::from_req(&req)?;

    if origin.is_none() {
        let is_local_mode = is_local_dev_mode(&ctx)?;

        if !is_local_mode {
            return Response::empty()
                .map(|res| res.with_status(403));
        }
    }

    // If origin is None (i.e., local development) set MathSweCom by default
    let domain = origin.clone().map(Origin::domain).unwrap_or(MathSweCom);
    let json = req.json::<CookieConsentPref>().await;
    let geolocation = Geolocation::from_req(&req);
    let ip = req
        .headers()
        .get("cf-connecting-ip")
        .unwrap_or(None)
        .map(|raw_ip| Ipv4Addr::from_str(&raw_ip))
        .and_then(Result::ok)
        .map(AnonymousIpv4::from_ipv4);

    let user_agent = req
        .headers()
        .get("user-agent")
        .unwrap_or(None)
        .unwrap_or("".to_string());

    match json {
        Ok(pref) => register_consent(
            ctx,
            domain,
            pref,
            geolocation,
            ip,
            user_agent,
        )
            .await
            .and_then(|res| handle_cors(res, origin)),
        Err(e) => Response::error(format!("Invalid JSON body: {}", e), 400),
    }
}

async fn register_consent(
    ctx: RouteContext<()>,
    domain: Domain,
    pref: CookieConsentPref,
    geolocation: Geolocation,
    anonymous_ip: Option<AnonymousIpv4>,
    user_agent: String,
) -> Result<Response, Error> {
    let cookie_consent_kv = "COOKIE_CONSENT";
    let consent = CookieConsent::new(
        domain,
        pref,
        geolocation,
        anonymous_ip,
        user_agent,
    );
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

fn cors(res: Response, origin: Origin) -> Result<Response, Error> {
    res
        .with_cors(&Cors::new()
            .with_origins(vec![origin.to_string()])
            .with_methods(vec![Method::Post])
            .with_max_age(86400)
        )
}

fn handle_cors(mut res: Response, origin_option: Option<Origin>) -> Result<Response, Error> {
    origin_option
        .map(|origin| cors(res.cloned()?, origin))
        .unwrap_or(Ok(res))
}

fn is_local_dev_mode(ctx: &RouteContext<()>) -> Result<bool, Error> {
    let mode = ctx.env.var("MODE")?.to_string();

    Ok(mode == "local")
}

fn internal_error(msg: impl Into<String>, error: impl Display) -> Result<Response, Error> {
    console_log!("{}", format!("{}", error));
    Response::error(msg, 500)
}
