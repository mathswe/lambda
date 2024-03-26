// Copyright (c) 2024 Tobias Briones. All rights reserved.
// This file is part of https://github.com/mathswe/lambda

use serde::{Deserialize, Serialize};
use crate::anonymous_ip::AnonymousIpv4;
use crate::consent::{CookieConsent, CookieConsentPref, Domain};
use crate::geolocation::Geolocation;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct CookieConsentClientRequest {
    domain: Domain,
    pref: CookieConsentPref,
}

impl CookieConsentClientRequest {
    pub fn to_cookie_consent(
        self,
        geolocation: Geolocation,
        anonymous_ip: Option<AnonymousIpv4>,
        user_agent: String,
    ) -> CookieConsent {
        CookieConsent::new(self.domain, self.pref, geolocation, anonymous_ip, user_agent)
    }
}
