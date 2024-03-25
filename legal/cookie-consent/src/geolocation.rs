// Copyright (c) 2024 Tobias Briones. All rights reserved.
// This file is part of https://github.com/mathswe/lambda

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Geolocation {
    #[serde(with = "chrono_tz_serde")]
    time_zone: chrono_tz::Tz,
    colo: Option<String>,
    country: Option<String>,
    city: Option<String>,
    continent: Option<String>,
    latitude: Option<String>,
    longitude: Option<String>,
    postal_code: Option<String>,
    metro_code: Option<String>,
    region: Option<String>,
    region_code: Option<String>,
}

mod chrono_tz_serde {
    use std::str::FromStr;
    use chrono_tz::Tz;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(
        tz: &Tz,
        serializer: S,
    ) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(tz.name())
    }

    pub fn deserialize<'de, D>(
        deserializer: D
    ) -> Result<Tz, D::Error> where D: Deserializer<'de> {
        let tz_str = String::deserialize(deserializer)?;

        Tz::from_str(&tz_str).map_err(serde::de::Error::custom)
    }
}
