impl crate::ToSql for std::net::IpAddr {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::INET
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_sql()
    }
}

impl crate::FromSql for std::net::IpAddr {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        crate::not_null(raw)?
            .parse()
            .map_err(|_| Self::error(ty, "std::net::IpAddr", raw))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/network.c#L267
     */
    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        use byteorder::ReadBytesExt;

        const AF_INET: u8 = 2;
        const AF_INET6: u8 = 3;

        let mut buf = crate::from_sql::not_null(raw)?;
        let ip_familly = buf.read_u8()?;
        let _bits = buf.read_u8()?;
        let is_cidr = buf.read_u8()?;
        let _nb = buf.read_u8()? as usize;

        if is_cidr == 1 {
            return Err(Self::error(ty, "std::net::IpAddr", raw));
        }

        let ip = if ip_familly == AF_INET {
            let ipv4 = std::net::Ipv4Addr::from(
                buf.read_u32::<byteorder::BigEndian>()?,
            );

            ipv4.into()
        }
        else if ip_familly == AF_INET6 {
            let ipv6 = std::net::Ipv6Addr::from(
                buf.read_u128::<byteorder::BigEndian>()?,
            );

            ipv6.into()
        }
        else {
            return Err(Self::error(ty, "std::net::IpAddr", raw));
        };

        Ok(ip)
    }
}

#[cfg(test)]
mod test {
    crate::sql_test!(inet, std::net::IpAddr, [
        (
            "'127.0.0.1'",
            std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)
        ),
        (
            "'127.0.0.1/32'",
            std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)
        ),
        ("'::1'", std::net::IpAddr::V6(std::net::Ipv6Addr::LOCALHOST)),
    ]);
}
