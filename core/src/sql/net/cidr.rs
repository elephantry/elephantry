/**
 * Rust type for [cidr](https://www.postgresql.org/docs/current/datatype-net-types.html).
 */
#[cfg_attr(docsrs, doc(cfg(feature = "net")))]
pub type Cidr = ipnetwork::IpNetwork;

#[cfg_attr(docsrs, doc(cfg(feature = "net")))]
impl crate::ToSql for Cidr {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::CIDR
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/network.c#L104
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        self.to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/network.c#L275
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let mut buf = Vec::new();

        let (ip_familly, nb) = match self {
            Cidr::V4(_) => (super::IpFamilly::Inet, 4),
            Cidr::V6(_) => (super::IpFamilly::Inet6, 16),
        };
        buf.push(ip_familly as u8);

        let netmask_bits = self.prefix();
        buf.push(netmask_bits);

        let is_cidr = 1;
        buf.push(is_cidr);

        buf.push(nb);

        match self {
            Cidr::V4(addr) => buf.extend_from_slice(&addr.ip().octets()),
            Cidr::V6(addr) => buf.extend_from_slice(&addr.ip().octets()),
        }

        Ok(Some(buf))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "net")))]
impl crate::FromSql for Cidr {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/network.c#L148
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        crate::from_sql::not_null(raw)?
            .parse()
            .map_err(|_| Self::error(ty, raw))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/network.c#L233
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let network = super::Network::try_from(crate::from_sql::not_null(raw)?)?;

        if !network.is_cidr {
            return Err(Self::error(ty, raw));
        }

        let cidr = match network.ip_familly {
            super::IpFamilly::Inet => {
                let cidr =
                    ipnetwork::Ipv4Network::new((network.ip as u32).into(), network.netmask_bits)
                        .map_err(|_| Self::error(ty, raw))?;
                Cidr::V4(cidr)
            }
            super::IpFamilly::Inet6 => {
                let cidr = ipnetwork::Ipv6Network::new(network.ip.into(), network.netmask_bits)
                    .map_err(|_| Self::error(ty, raw))?;
                Cidr::V6(cidr)
            }
        };

        Ok(cidr)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "net")))]
impl crate::entity::Simple for Cidr {}

#[cfg(test)]
mod test {
    crate::sql_test!(
        cidr,
        crate::Cidr,
        [(
            "'192.168.1.0/24'",
            crate::Cidr::V4("192.168.1.0/24".parse().unwrap())
        )]
    );
}
