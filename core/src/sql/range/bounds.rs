use std::ops::Bound::{self, *};

pub(crate) struct Bounds<T> {
    pub start: Bound<T>,
    pub end: Bound<T>,
}

impl<'a, T> From<&'a std::ops::Range<T>> for Bounds<&'a T> {
    fn from(range: &'a std::ops::Range<T>) -> Self {
        Self {
            start: Included(&range.start),
            end: Excluded(&range.end),
        }
    }
}

impl<T> TryFrom<Bounds<T>> for std::ops::Range<T> {
    type Error = ();

    fn try_from(value: Bounds<T>) -> Result<Self, Self::Error> {
        let start = match value.start {
            Included(start) => start,
            _ => return Err(()),
        };
        let end = match value.end {
            Excluded(end) => end,
            _ => return Err(()),
        };

        Ok(Self { start, end })
    }
}

impl<'a, T> From<&'a std::ops::RangeFrom<T>> for Bounds<&'a T> {
    fn from(range: &'a std::ops::RangeFrom<T>) -> Self {
        Self {
            start: Included(&range.start),
            end: Unbounded,
        }
    }
}

impl<T> TryFrom<Bounds<T>> for std::ops::RangeFrom<T> {
    type Error = ();

    fn try_from(value: Bounds<T>) -> Result<Self, Self::Error> {
        if !matches!(value.end, Unbounded) {
            return Err(());
        }

        let start = match value.start {
            Included(start) => start,
            _ => return Err(()),
        };

        Ok(Self { start })
    }
}

impl<'a, T> From<&'a std::ops::RangeTo<T>> for Bounds<&'a T> {
    fn from(range: &'a std::ops::RangeTo<T>) -> Self {
        Self {
            start: Unbounded,
            end: Excluded(&range.end),
        }
    }
}

impl<T> TryFrom<Bounds<T>> for std::ops::RangeTo<T> {
    type Error = ();

    fn try_from(value: Bounds<T>) -> Result<Self, Self::Error> {
        if !matches!(value.start, Unbounded) {
            return Err(());
        }

        let end = match value.end {
            Excluded(end) => end,
            _ => return Err(()),
        };

        Ok(Self { end })
    }
}

impl<'a, T> From<&'a std::ops::RangeToInclusive<T>> for Bounds<&'a T> {
    fn from(range: &'a std::ops::RangeToInclusive<T>) -> Self {
        Self {
            start: Unbounded,
            end: Included(&range.end),
        }
    }
}

impl<'a, T> From<&'a std::ops::RangeInclusive<T>> for Bounds<&'a T> {
    fn from(range: &'a std::ops::RangeInclusive<T>) -> Self {
        Self {
            start: Included(range.start()),
            end: Included(range.end()),
        }
    }
}

impl<'a, T> From<&'a (Bound<T>, Bound<T>)> for Bounds<&'a T> {
    fn from(range: &'a (Bound<T>, Bound<T>)) -> Self {
        fn as_ref<T>(bound: &Bound<T>) -> Bound<&T> {
            match bound {
                Included(ref x) => Included(x),
                Excluded(ref x) => Excluded(x),
                Unbounded => Unbounded,
            }
        }

        Self {
            start: as_ref(&range.0),
            end: as_ref(&range.1),
        }
    }
}

impl<T> From<Bounds<T>> for (Bound<T>, Bound<T>) {
    fn from(bounds: Bounds<T>) -> Self {
        (bounds.start, bounds.end)
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub(crate) struct Flags: u8 {
        const EMPTY = 0x01;
        const LB_INC = 0x02;
        const UB_INC = 0x04;
        const LB_INF = 0x08;
        const UB_INF = 0x10;
    }
}

impl<'a, T: crate::ToSql> crate::ToSql for Bounds<&'a T> {
    fn ty(&self) -> crate::pq::Type {
        use crate::pq::ToArray;

        match self.start {
            Included(start) | Excluded(start) => return start.ty().to_range(),
            _ => (),
        }

        match self.end {
            Included(end) | Excluded(end) => return end.ty().to_range(),
            _ => (),
        }

        crate::pq::types::UNKNOWN
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/rangetypes.c#L123
     */
    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        let mut vec = Vec::new();

        let start_char = match self.start {
            Included(_) => b'[',
            _ => b'(',
        };
        vec.push(start_char);

        bound_to_text(&mut vec, &self.start)?;

        vec.push(b',');

        bound_to_text(&mut vec, &self.end)?;

        let end_char = match self.end {
            Included(_) => b']',
            _ => b')',
        };
        vec.push(end_char);

        vec.push(b'\0');

        Ok(Some(vec))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/rangetypes.c#L246
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let mut buf = Vec::new();

        let mut flags = Flags::empty();
        match self.start {
            Included(_) => flags.insert(Flags::LB_INC),
            Unbounded => flags.insert(Flags::LB_INF),
            _ => (),
        };

        match self.end {
            Included(_) => flags.insert(Flags::UB_INC),
            Unbounded => flags.insert(Flags::UB_INF),
            _ => (),
        };

        buf.push(flags.bits());

        bound_to_binary(&mut buf, &self.start)?;
        bound_to_binary(&mut buf, &self.end)?;

        Ok(Some(buf))
    }
}

macro_rules! bound {
    ($bound:ident, $op:ident) => {{
        let bound = match $bound {
            Included(bound) => bound,
            Excluded(bound) => bound,
            Unbounded => panic!(),
        };

        match bound.$op()? {
            Some(bound) => bound,
            None => return Ok(()),
        }
    }};
}

fn bound_to_text<T: crate::ToSql>(buf: &mut Vec<u8>, bound: &Bound<&T>) -> crate::Result<()> {
    if !matches!(bound, Unbounded) {
        let mut b = bound!(bound, to_text);
        b.pop(); // removes \0
        buf.append(&mut b);
    }

    Ok(())
}

fn bound_to_binary<T: crate::ToSql>(buf: &mut Vec<u8>, bound: &Bound<&T>) -> crate::Result<()> {
    if !matches!(bound, Unbounded) {
        let mut b = bound!(bound, to_binary);
        crate::to_sql::write_i32(buf, b.len() as i32)?;
        buf.append(&mut b);
    }

    Ok(())
}

impl<T: crate::FromSql> crate::FromSql for Bounds<T> {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/rangetypes.c#L81
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let raw = crate::not_null(raw)?;

        if raw == "empty" {
            log::error!("Unsuported empty range");
            return Err(Self::error(ty, raw));
        }

        lazy_static::lazy_static! {
            static ref REGEX: regex::Regex =
                regex::Regex::new(r"[\[\(](?P<start>.?*),(?P<end>.?*)[\]\)]")
                    .unwrap();
        }

        let matches = REGEX.captures(raw).unwrap();

        // tstzrange are quoted
        let start_str = matches
            .name("start")
            .map(|x| x.as_str().trim_matches('\"'))
            .unwrap_or_default();
        let start_bound = if start_str.is_empty() {
            Bound::Unbounded
        } else {
            let start = T::from_text(ty, Some(start_str))?;

            match raw.chars().next() {
                Some('[') => Bound::Included(start),
                Some('(') => Bound::Excluded(start),
                _ => return Err(Self::error(ty, raw)),
            }
        };

        // tstzrange are quoted
        let end_str = matches
            .name("end")
            .map(|x| x.as_str().trim_matches('\"'))
            .unwrap_or_default();
        let end_bound = if end_str.is_empty() {
            Bound::Unbounded
        } else {
            let end = T::from_text(ty, Some(end_str))?;

            match raw.chars().last() {
                Some(']') => Bound::Included(end),
                Some(')') => Bound::Excluded(end),
                _ => return Err(Self::error(ty, raw)),
            }
        };

        Ok(Self {
            start: start_bound,
            end: end_bound,
        })
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/rangetypes.c#L163
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let mut buf = crate::from_sql::not_null(raw)?;

        let flag = Flags::from_bits_truncate(crate::from_sql::read_u8(&mut buf)?);
        if flag.contains(Flags::EMPTY) {
            log::error!("Unsuported empty range");
            return Err(Self::error(ty, raw));
        }

        let start_bound = if flag.contains(Flags::LB_INF) {
            Bound::Unbounded
        } else {
            let start_bound_len = crate::from_sql::read_i32(&mut buf)?;
            let mut start = Vec::new();
            for _ in 0..start_bound_len {
                let b = crate::from_sql::read_u8(&mut buf)?;

                start.push(b);
            }

            if flag.contains(Flags::LB_INC) {
                Bound::Included(T::from_binary(ty, Some(&start))?)
            } else {
                Bound::Excluded(T::from_binary(ty, Some(&start))?)
            }
        };

        let end_bound = if flag.contains(Flags::UB_INF) {
            Bound::Unbounded
        } else {
            let end_bound_len = crate::from_sql::read_i32(&mut buf)?;
            let mut end = Vec::new();
            for _ in 0..end_bound_len {
                let b = crate::from_sql::read_u8(&mut buf)?;

                end.push(b);
            }

            if flag.contains(Flags::UB_INC) {
                Bound::Included(T::from_binary(ty, Some(&end))?)
            } else {
                Bound::Excluded(T::from_binary(ty, Some(&end))?)
            }
        };

        Ok(Self {
            start: start_bound,
            end: end_bound,
        })
    }
}
