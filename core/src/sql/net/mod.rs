mod cidr;
mod inet;
mod macaddr;
mod macaddr8;

#[derive(Eq, PartialEq)]
enum IpFamilly {
    Inet = 2,
    Inet6 = 3,
}

struct Network {
    ip_familly: IpFamilly,
    netmask_bits: u8,
    is_cidr: bool,
    ip: u128,
}

impl TryFrom<&[u8]> for Network {
    type Error = crate::Error;

    fn try_from(raw: &[u8]) -> crate::Result<Self> {
        use byteorder::ReadBytesExt;

        const AF_INET: u8 = 2;
        const AF_INET6: u8 = 3;

        let mut buf = raw;
        let ip_familly = match buf.read_u8()? {
            AF_INET => IpFamilly::Inet,
            AF_INET6 => IpFamilly::Inet6,
            _ => unreachable!(),
        };
        let netmask_bits = buf.read_u8()?;
        let is_cidr = buf.read_u8()? == 1;
        let _nb = buf.read_u8()? as usize;

        let ip = match ip_familly {
            IpFamilly::Inet => buf.read_u32::<byteorder::BigEndian>()? as u128,
            IpFamilly::Inet6 => buf.read_u128::<byteorder::BigEndian>()?,
        };

        let network = Network {
            ip_familly,
            netmask_bits,
            is_cidr,
            ip,
        };

        Ok(network)
    }
}
