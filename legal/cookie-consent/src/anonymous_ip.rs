// Copyright (c) 2024 Tobias Briones. All rights reserved.
// This file is part of https://github.com/mathswe/lambda

use std::net::Ipv4Addr;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct AnonymousIpv4(String);

impl AnonymousIpv4 {
    pub fn from_ipv4(ipv4addr: Ipv4Addr) -> AnonymousIpv4 {
        let octets = ipv4addr.octets();
        let octet1 = octets[0];
        let octet2 = octets[1];
        let octet3 = octets[2];
        let anonymous_ip = format!("{}.{}.{}.0", octet1, octet2, octet3);

        AnonymousIpv4(anonymous_ip)
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;
    use crate::anonymous_ip::AnonymousIpv4;

    #[test]
    fn creates_anonymous_ipv4_from_original_ip() {
        assert_eq!(
            AnonymousIpv4("1.1.1.0".to_string()),
            AnonymousIpv4::from_ipv4(Ipv4Addr::new(1, 1, 1, 1))
        );

        assert_eq!(
            AnonymousIpv4("123.213.231.0".to_string()),
            AnonymousIpv4::from_ipv4(Ipv4Addr::new(123, 213, 231, 85))
        );

        assert_eq!(
            AnonymousIpv4("240.80.150.0".to_string()),
            AnonymousIpv4::from_ipv4(Ipv4Addr::new(240, 80, 150, 210))
        );
    }
}
