mod cidr;
mod inet;
mod macaddr;
mod macaddr8;

pub use cidr::*;
pub use macaddr::*;
pub use macaddr8::*;

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
        const AF_INET: u8 = 2;
        const AF_INET6: u8 = 3;

        let mut buf = raw;
        let ip_familly = match crate::from_sql::read_u8(&mut buf)? {
            AF_INET => IpFamilly::Inet,
            AF_INET6 => IpFamilly::Inet6,
            _ => unreachable!(),
        };
        let netmask_bits = crate::from_sql::read_u8(&mut buf)?;
        let is_cidr = crate::from_sql::read_u8(&mut buf)? == 1;
        let _nb = crate::from_sql::read_u8(&mut buf)? as usize;

        let ip = match ip_familly {
            IpFamilly::Inet => crate::from_sql::read_u32(&mut buf)? as u128,
            IpFamilly::Inet6 => crate::from_sql::read_u128(&mut buf)?,
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
