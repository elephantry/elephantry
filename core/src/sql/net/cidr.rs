#[cfg_attr(docsrs, doc(cfg(feature = "net")))]
impl crate::ToSql for ipnetwork::IpNetwork {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::CIDR
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/network.c#L104
     */
    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/network.c#L275
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let mut buf = Vec::new();

        let (ip_familly, nb) = match self {
            ipnetwork::IpNetwork::V4(_) => (super::IpFamilly::Inet, 4),
            ipnetwork::IpNetwork::V6(_) => (super::IpFamilly::Inet6, 16),
        };
        buf.push(ip_familly as u8);

        let netmask_bits = self.prefix();
        buf.push(netmask_bits);

        let is_cidr = 1;
        buf.push(is_cidr);

        buf.push(nb);

        match self {
            ipnetwork::IpNetwork::V4(addr) => buf.extend_from_slice(&addr.ip().octets()),
            ipnetwork::IpNetwork::V6(addr) => buf.extend_from_slice(&addr.ip().octets()),
        }

        Ok(Some(buf))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "net")))]
impl crate::FromSql for ipnetwork::IpNetwork {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/network.c#L148
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        crate::not_null(raw)?
            .parse()
            .map_err(|_| Self::error(ty, "ipnetwork::IpNetwork", raw))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/network.c#L233
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        use std::convert::TryFrom;

        let network = super::Network::try_from(crate::from_sql::not_null(raw)?)?;

        if !network.is_cidr {
            return Err(Self::error(ty, "ipnetwork::IpNetwork", raw));
        }

        let cidr = match network.ip_familly {
            super::IpFamilly::Inet => {
                let cidr =
                    ipnetwork::Ipv4Network::new((network.ip as u32).into(), network.netmask_bits)
                        .map_err(|_| Self::error(ty, "ipnetwork::IpNetwork", raw))?;
                ipnetwork::IpNetwork::V4(cidr)
            }
            super::IpFamilly::Inet6 => {
                let cidr = ipnetwork::Ipv6Network::new(network.ip.into(), network.netmask_bits)
                    .map_err(|_| Self::error(ty, "ipnetwork::IpNetwork", raw))?;
                ipnetwork::IpNetwork::V6(cidr)
            }
        };

        Ok(cidr)
    }
}

#[cfg(test)]
mod test {
    crate::sql_test!(
        cidr,
        ipnetwork::IpNetwork,
        [(
            "'192.168.1.0/24'",
            ipnetwork::IpNetwork::V4("192.168.1.0/24".parse().unwrap())
        )]
    );
}
