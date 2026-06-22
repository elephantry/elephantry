/**
 * Test type convertion **from** SQL as binary and text format.
 */
#[macro_export]
macro_rules! from {
    ($sql_type:ident, $rust_type:ty, $tests:expr) => {
        #[$crate::test(fixture = "test")]
        fn from_text(connection: $crate::Connection) -> $crate::Result {
            $crate::testing::from_text::<$rust_type>(&connection, stringify!($sql_type), &$tests)
        }

        #[$crate::test(fixture = "test")]
        fn from_binary(connection: $crate::Connection) -> $crate::Result {
            $crate::testing::from_binary::<$rust_type>(&connection, stringify!($sql_type), &$tests)
        }
    };
}

pub use from;

/**
 * Test type convertion **to** SQL as binary and text format.
 */
#[macro_export]
macro_rules! to {
    ($sql_type:ident, $rust_type:ty, $tests:expr) => {
        #[$crate::test(fixture = "test")]
        fn to_text(connection: $crate::Connection) -> $crate::Result {
            $crate::testing::to_text::<$rust_type>(&connection, stringify!($sql_type), &$tests)
        }

        #[$crate::test(fixture = "test")]
        fn to_binary(connection: $crate::Connection) -> $crate::Result {
            $crate::testing::to_binary::<$rust_type>(&connection, stringify!($sql_type), &$tests)
        }
    };
}

pub use to;

/**
 * Test type convertion from and to SQL as binary and text format.
 */
#[macro_export]
macro_rules! convertion {
    ($sql_type:ident, $rust_type:ty, $tests:expr) => {
        mod $sql_type {
            $crate::testing::from!($sql_type, $rust_type, $tests);
            $crate::testing::to!($sql_type, $rust_type, $tests);
        }
    };
}

pub use convertion;
