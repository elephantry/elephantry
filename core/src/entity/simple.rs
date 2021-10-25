pub trait Simple: crate::ToSql + crate::FromSql {
}

macro_rules! simple_entity {
    ($ty:ty) => {
        impl Simple for $ty {
        }
    };
}

simple_entity!(());
simple_entity!(bool);
simple_entity!(char);
simple_entity!(f32);
simple_entity!(f64);
simple_entity!(i16);
simple_entity!(i32);
simple_entity!(i64);
simple_entity!(u16);
simple_entity!(u32);
simple_entity!(String);

impl<T: Simple> Simple for Option<T> {
}

impl<T: Simple + Clone> Simple for Vec<T> {
}
