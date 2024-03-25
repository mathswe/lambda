// Copyright (c) 2024 Tobias Briones. All rights reserved.
// This file is part of https://github.com/mathswe/lambda

use chrono::{DateTime, Utc};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use crate::geolocation::Geolocation;

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CookieConsentPref {
    essential: bool,
    functional: bool,
    analytics: bool,
    targeting: bool,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct CookieConsent {
    id: String,
    value: CookieConsentValue,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct CookieConsentValue {
    created_at: DateTime<Utc>,
    pref: CookieConsentPref,
    geolocation: Geolocation,
}

impl CookieConsent {
    pub fn new(pref: CookieConsentPref, geolocation: Geolocation) -> Self {
        CookieConsent {
            id: nanoid!(),
            value: CookieConsentValue {
                created_at: Utc::now(),
                pref,
                geolocation,
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
        let consent = CookieConsent::new(CookieConsentPref {
            essential: true,
            functional: false,
            analytics: true,
            targeting: false,
        }, dummy_geolocation());
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
                created_at: "2024-03-10 17:49:01.613437 UTC".parse().unwrap(),
                pref: CookieConsentPref {
                    essential: true,
                    functional: false,
                    analytics: true,
                    targeting: false,
                },
                geolocation: dummy_geolocation(),
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

    fn dummy_geolocation() -> Geolocation {
        Geolocation::empty_with(chrono_tz::Tz::America__Tegucigalpa, String::from(""))
    }
}
