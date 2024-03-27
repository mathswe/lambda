// Copyright (c) 2024 Tobias Briones. All rights reserved.
// This file is part of https://github.com/mathswe/lambda

use chrono::{DateTime, Utc};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use Domain::{MathSoftware, MathSoftwareEngineer, MathSweCom};

use crate::anonymous_ip::AnonymousIpv4;
use crate::geolocation::Geolocation;

#[derive(PartialEq, Clone, EnumIter, Debug, Serialize, Deserialize)]
pub enum Domain {
    MathSweCom,
    MathSoftware,
    MathSoftwareEngineer,
}

impl Domain {
    pub fn to_domain_name(&self) -> String {
        match self {
            MathSweCom => "mathswe.com".to_string(),
            MathSoftware => "math.software".to_string(),
            MathSoftwareEngineer => "mathsoftware.engineer".to_string(),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CookieConsentPref {
    essential: bool,
    functional: bool,
    analytics: bool,
    targeting: bool,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct CookieConsentValue {
    domain: Domain,
    pref: CookieConsentPref,
    created_at: DateTime<Utc>,
    geolocation: Geolocation,
    anonymous_ip: Option<AnonymousIpv4>,
    user_agent: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct CookieConsent {
    id: String,
    value: CookieConsentValue,
}

impl CookieConsent {
    pub fn new(
        domain: Domain,
        pref: CookieConsentPref,
        geolocation: Geolocation,
        anonymous_ip: Option<AnonymousIpv4>,
        user_agent: String,
    ) -> Self {
        CookieConsent {
            id: nanoid!(),
            value: CookieConsentValue {
                domain,
                pref,
                created_at: Utc::now(),
                geolocation,
                anonymous_ip,
                user_agent,
            },
        }
    }

    pub fn to_kv(&self) -> (String, CookieConsentValue) {
        (self.id.to_string(), self.value.clone())
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;
    use super::*;

    #[test]
    fn cookie_consent_pref_serialization() {
        let pref = CookieConsentPref {
            essential: true,
            functional: false,
            analytics: true,
            targeting: false,
        };
        let json = serde_json::to_string(&pref).unwrap();
        let deserialized_pref = serde_json::from_str::<CookieConsentPref>(&json).unwrap();

        assert_eq!(pref, deserialized_pref);
    }

    #[test]
    fn cookie_consent_serialization() {
        let consent = CookieConsent::new(MathSweCom, CookieConsentPref {
            essential: true,
            functional: false,
            analytics: true,
            targeting: false,
        }, dummy_geolocation(), dummy_ip(), dummy_user_agent());
        let json = serde_json::to_string(&consent).unwrap();
        let deserialized_consent = serde_json::from_str::<CookieConsent>(&json).unwrap();

        assert_eq!(
            consent,
            deserialized_consent,
            "generated consents are equal when serializing"
        );
        assert_eq!(
            consent.to_json(),
            json,
            "generated consent JSONs are equal when serializing"
        );
    }

    #[test]
    fn synthetic_cookie_consent_serialization() {
        let synthetic_consent = CookieConsent {
            id: String::from("abc"),
            value: CookieConsentValue {
                domain: MathSweCom,
                pref: CookieConsentPref {
                    essential: true,
                    functional: false,
                    analytics: true,
                    targeting: false,
                },
                created_at: "2024-03-10 17:49:01.613437 UTC".parse().unwrap(),
                geolocation: dummy_geolocation(),
                anonymous_ip: dummy_ip(),
                user_agent: dummy_user_agent(),
            },
        };
        let json = serde_json::to_string(&synthetic_consent).unwrap();
        let deserialized_consent = serde_json::from_str::<CookieConsent>(&json).unwrap();

        assert_eq!(
            synthetic_consent,
            deserialized_consent,
            "synthetic consents are equal when serializing"
        );
        assert_eq!(
            synthetic_consent.to_json(),
            json,
            "synthetic consent JSONs are equal when serializing"
        );
    }

    fn dummy_ip() -> Option<AnonymousIpv4> {
        Some(AnonymousIpv4::from_ipv4(Ipv4Addr::new(1, 1, 1, 1)))
    }

    fn dummy_geolocation() -> Geolocation {
        Geolocation::empty_with(chrono_tz::Tz::America__Tegucigalpa, String::from(""))
    }

    fn dummy_user_agent() -> String {
        "User Agent Is: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36 Edg/122.0.0.0"
            .to_string()
    }
}
