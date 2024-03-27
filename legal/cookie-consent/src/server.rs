// Copyright (c) 2024 Tobias Briones. All rights reserved.
// This file is part of https://github.com/mathswe/lambda

use std::fmt::Display;
use worker::{console_log, Cors, Error, Method, Response, RouteContext};
use crate::client_req::Origin;

pub fn handle_cors(mut res: Response, origin_option: Option<Origin>) -> Result<Response, Error> {
    origin_option
        .map(|origin| cors(res.cloned()?, origin))
        .unwrap_or(Ok(res))
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
