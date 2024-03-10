// Copyright (c) 2024 Tobias Briones. All rights reserved.
// This file is part of https://github.com/mathswe/lambda

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct CookieConsentPref {
    essential: bool,
    functional: bool,
    analytics: bool,
    targeting: bool,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct CookieConsent {
    id: String,
    pref: CookieConsentPref,
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
        let consent = CookieConsent {
            id: String::from("abc"),
            pref: CookieConsentPref {
                essential: true,
                functional: false,
                analytics: true,
                targeting: false,
            },
        };
        let json = serde_json::to_string(&consent).unwrap();
        let deserialized_consent = serde_json::from_str::<CookieConsent>(&json).unwrap();

        assert_eq!(consent, deserialized_consent);
    }
}
