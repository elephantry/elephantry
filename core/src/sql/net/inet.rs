impl crate::ToSql for std::net::IpAddr {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::INET
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/network.c#L14
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        self.to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/network.c#L267
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let mut buf = Vec::new();

        let (ip_familly, netmask_bits, nb) = match self {
            std::net::IpAddr::V4(_) => (super::IpFamilly::Inet, 32, 4),
            std::net::IpAddr::V6(_) => (super::IpFamilly::Inet6, 128, 16),
        };

        buf.push(ip_familly as u8);
        buf.push(netmask_bits);

        let is_cidr = 0;
        buf.push(is_cidr);

        buf.push(nb);

        match self {
            std::net::IpAddr::V4(addr) => buf.extend_from_slice(&addr.octets()),
            std::net::IpAddr::V6(addr) => buf.extend_from_slice(&addr.octets()),
        }

        Ok(Some(buf))
    }
}

impl crate::FromSql for std::net::IpAddr {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/network.c#L96
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        crate::from_sql::not_null(raw)?
            .parse()
            .map_err(|_| Self::error(ty, raw))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/network.c#L225
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let network = super::Network::try_from(crate::from_sql::not_null(raw)?)?;

        if network.is_cidr {
            return Err(Self::error(ty, raw));
        }

        let ip = match network.ip_familly {
            super::IpFamilly::Inet => {
                let ipv4 = std::net::Ipv4Addr::from(network.ip as u32);
                ipv4.into()
            }
            super::IpFamilly::Inet6 => {
                let ipv6 = std::net::Ipv6Addr::from(network.ip);
                ipv6.into()
            }
        };

        Ok(ip)
    }
}

impl crate::entity::Simple for std::net::IpAddr {}

#[cfg(test)]
mod test {
    crate::sql_test!(
        inet,
        std::net::IpAddr,
        [
            (
                "'127.0.0.1'",
                std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)
            ),
            (
                "'127.0.0.1/32'",
                std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)
            ),
            ("'::1'", std::net::IpAddr::V6(std::net::Ipv6Addr::LOCALHOST)),
        ]
    );
}
