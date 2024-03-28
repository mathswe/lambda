// Copyright (c) 2024 Tobias Briones. All rights reserved.
// This file is part of https://github.com/mathswe/lambda

use worker::*;

use crate::cookie_consent::post_consent;

mod consent;
mod cookie_consent;
mod geolocation;
mod anonymous_ip;
mod client_req;
mod server;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .post_async("/", post_consent)
        .run(req, env)
        .await
}
