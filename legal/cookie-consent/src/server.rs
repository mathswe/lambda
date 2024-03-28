// Copyright (c) 2024 Tobias Briones. All rights reserved.
// This file is part of https://github.com/mathswe/lambda

use std::fmt::Display;
use worker::{console_log, Cors, Error, Method, Request, Response, RouteContext};
use crate::client_req::Origin;
use crate::consent::Domain;
use crate::consent::Domain::MathSweCom;

/// Defines an `Origin` managed by the server by wrapping the actual `Origin` and defining
/// operations to allow development mode, in which there is no origin at all. If a `OriginProxy`
/// value exists is because its `Origin` is valid. Its `Origin` is `None` when the request comes
/// from local mode.
#[derive(Clone)]
pub struct OriginProxy(Option<Origin>);

impl OriginProxy {
    pub fn from_req(req: &Request, ctx: &RouteContext<()>) -> Result<Option<OriginProxy>, Error> {
        let origin_option = Origin::from_req(req)?;

        match origin_option {
            Some(origin) => Ok(Some(OriginProxy(Some(origin)))),
            None => {
                let is_local_mode = is_local_dev_mode(&ctx)?;

                if is_local_mode {
                    Ok(Some(OriginProxy(None)))
                } else {
                    Ok(None)
                }
            }
        }
    }

    /// Returns the `Domain` of `Origin`. If there's no `Origin`, local mode is assumed and
    /// returns `MathSweCom` by default.
    pub fn domain(self) -> Domain {
        self
            .0
            .map(Origin::domain)
            .unwrap_or(MathSweCom)
    }

    /// It handles CORS for the underlying `Origin` on the given `Response`. If this
    /// `ProxyOrigin` doesn't have an `Origin`, then local mode is assumed and the same
    /// `Response` is returned without modifications.
    pub fn handle_cors(self, mut res: Response) -> Result<Response, Error> {
        self
            .0
            .map(|origin| cors(res.cloned()?, origin))
            .unwrap_or(Ok(res))
    }
}

pub fn is_local_dev_mode(ctx: &RouteContext<()>) -> Result<bool, Error> {
    let mode = ctx.env.var("MODE")?.to_string();

    Ok(mode == "local")
}

pub fn internal_error(msg: impl Into<String>, error: impl Display) -> Result<Response, Error> {
    console_log!("{}", format!("{}", error));
    Response::error(msg, 500)
}

fn cors(res: Response, origin: Origin) -> Result<Response, Error> {
    res
        .with_cors(&Cors::new()
            .with_origins(vec![origin.to_string()])
            .with_methods(vec![Method::Post])
            .with_max_age(86400)
        )
}
