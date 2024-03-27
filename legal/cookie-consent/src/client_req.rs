// Copyright (c) 2024 Tobias Briones. All rights reserved.
// This file is part of https://github.com/mathswe/lambda

use strum::IntoEnumIterator;
use worker::{Error, Request};

use crate::consent::Domain;

/// Defines an accepted client origin, where the scheme is `HTTPS`, the hostname is one of
/// `Domain` variants, and has no explicit ports. Further, it allows all subdomains of a given
/// `Domain`.
/// For example, `https://mathswe.com` and `https://staging.mathswe.com` are accepted `Origin`s.
#[derive(PartialEq, Debug)]
pub struct Origin {
    domain: Domain,
    subdomain: Option<String>,
}

impl Origin {
    pub fn from_str(origin: &str) -> Option<Self> {
        struct Hostname(String);

        fn belongs_to_domain(Hostname(hostname): &Hostname, domain: &Domain) -> bool {
            let domain_name = &domain.to_domain_name();

            hostname.strip_suffix(domain_name).is_some()
        }

        fn find_domain(hostname: &Hostname) -> Option<Domain> {
            Domain::iter()
                .find(|domain| belongs_to_domain(hostname, domain))
        }

        fn get_subdomain(Hostname(hostname): &Hostname, domain: &Domain) -> Option<String> {
            let domain_name = &domain.to_domain_name();

            hostname
                .strip_suffix(&format!(".{}", domain_name))
                .map(|str| str.to_string())
        }

        fn get_origin(hostname: &Hostname) -> Option<Origin> {
            find_domain(hostname)
                .map(|domain| {
                    let subdomain = get_subdomain(hostname, &domain);
                    Origin { domain, subdomain }
                })
        }

        origin
            .strip_prefix("https://")
            .map(|hostname| Hostname(hostname.to_string()))
            .map(|hostname| get_origin(&hostname))
            .flatten()
    }

    pub fn from_req(req: &Request) -> Result<Option<Self>, Error> {
        Ok(
            req
                .headers()
                .get("Origin")?
                .as_deref()
                .map(Self::from_str)
                .flatten()
        )
    }

    pub fn domain(self) -> Domain {
        self.domain
    }

    pub fn to_string(self) -> String {
        let domain_name = self.domain.to_domain_name();
        let hostname = match self.subdomain {
            Some(subdomain) => format!("{}.{}", subdomain, domain_name),
            None => domain_name,
        };

        format!("https://{}", hostname)
    }
}


#[cfg(test)]
mod tests {
    use Domain::MathSweCom;

    use crate::client_req::Origin;
    use crate::consent::Domain;
    use crate::consent::Domain::{MathSoftware, MathSoftwareEngineer};

    #[test]
    fn accepts_valid_origins() {
        let valid_origin_cases = vec![
            (
                "https://mathswe.com",
                Some(Origin { domain: MathSweCom, subdomain: None }),
            ),
            (
                "https://staging.mathswe.com",
                Some(Origin { domain: MathSweCom, subdomain: Some("staging".to_string()) }),
            ),
            (
                "https://nested.subdomain.mathswe.com",
                Some(Origin {
                    domain: MathSweCom,
                    subdomain: Some("nested.subdomain".to_string()),
                }),
            ),
            (
                "https://math.software",
                Some(Origin { domain: MathSoftware, subdomain: None }),
            ),
            (
                "https://staging.math.software",
                Some(Origin { domain: MathSoftware, subdomain: Some("staging".to_string()) }),
            ),
            (
                "https://nested.subdomain.math.software",
                Some(Origin {
                    domain: MathSoftware,
                    subdomain: Some("nested.subdomain".to_string()),
                }),
            ),
            (
                "https://mathsoftware.engineer",
                Some(Origin { domain: MathSoftwareEngineer, subdomain: None }),
            ),
            (
                "https://staging.mathsoftware.engineer",
                Some(Origin {
                    domain: MathSoftwareEngineer,
                    subdomain: Some("staging".to_string()),
                }),
            ),
            (
                "https://nested.subdomain.mathsoftware.engineer",
                Some(Origin {
                    domain: MathSoftwareEngineer,
                    subdomain: Some("nested.subdomain".to_string()),
                }),
            ),
        ];

        valid_origin_cases
            .iter()
            .for_each(|(raw_origin, expected)| assert_eq!(
                *expected,
                Origin::from_str(raw_origin)
            ))
    }


    #[test]
    fn rejects_invalid_origins() {
        let invalid_origins = vec![
            "http://mathswe.com",
            "http://math.software",
            "http://mathsoftware.engineer",
            "http://example.com",
            "https://example.com",
            "https://abc.com",
            "https://abc.com",
            "http://abc.software",
            "http://abc.software",
            "https://abc.engineer",
            "https://abc.engineering",
        ];

        invalid_origins
            .iter()
            .for_each(|origin| assert_eq!(
                None,
                Origin::from_str(origin)
            ))
    }

    #[test]
    fn converts_origin_to_str() {
        let origin_cases = vec![
            "https://mathswe.com",
            "https://staging.mathswe.com",
            "https://nested.subdomain.mathswe.com",
            "https://math.software",
            "https://staging.math.software",
            "https://nested.subdomain.math.software",
            "https://mathsoftware.engineer",
            "https://staging.mathsoftware.engineer",
            "https://nested.subdomain.mathsoftware.engineer",
        ];

        origin_cases
            .iter()
            .for_each(|expected| assert_eq!(
                *expected,
                Origin::from_str(expected).unwrap().to_string()
            ))
    }
}
