/**
 * Test type convertion **from** SQL as binary and text format.
 */
#[macro_export]
macro_rules! from {
    (fixture: $fixture:literal, sql_type: $sql_type:ident, rust_type: $rust_type:ty, tests: $tests:expr $(,)*) => {
        #[$crate::test(fixture = $fixture)]
        fn from_text(connection: $crate::Connection) -> $crate::Result {
            $crate::testing::from_text::<$rust_type>(&connection, stringify!($sql_type), &$tests)
        }

        #[$crate::test(fixture = $fixture)]
        fn from_binary(connection: $crate::Connection) -> $crate::Result {
            $crate::testing::from_binary::<$rust_type>(&connection, stringify!($sql_type), &$tests)
        }
    };

    (sql_type: $sql_type:ident, rust_type: $rust_type:ty, tests: $tests:expr $(,)*) => {
        #[$crate::test]
        fn from_text(connection: $crate::Connection) -> $crate::Result {
            $crate::testing::from_text::<$rust_type>(&connection, stringify!($sql_type), &$tests)
        }

        #[$crate::test]
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
    (fixture: $fixture:literal, sql_type: $sql_type:ident, rust_type: $rust_type:ty, tests: $tests:expr $(,)*) => {
        #[$crate::test(fixture = $fixture)]
        fn to_text(connection: $crate::Connection) -> $crate::Result {
            $crate::testing::to_text::<$rust_type>(
                &connection.clone(),
                stringify!($sql_type),
                &$tests,
            )
        }

        #[$crate::test(fixture = $fixture)]
        fn to_binary(connection: $crate::Connection) -> $crate::Result {
            $crate::testing::to_binary::<$rust_type>(
                &connection.clone(),
                stringify!($sql_type),
                &$tests,
            )
        }
    };

    (sql_type: $sql_type:ident, rust_type: $rust_type:ty, tests: $tests:expr $(,)*) => {
        #[$crate::test]
        fn to_text(connection: $crate::Connection) -> $crate::Result {
            $crate::testing::to_text::<$rust_type>(
                &connection.clone(),
                stringify!($sql_type),
                &$tests,
            )
        }

        #[$crate::test]
        fn to_binary(connection: $crate::Connection) -> $crate::Result {
            $crate::testing::to_binary::<$rust_type>(
                &connection.clone(),
                stringify!($sql_type),
                &$tests,
            )
        }
    };
}

pub use to;

/**
 * Test type convertion from and to SQL as binary and text format.
 */
#[macro_export]
macro_rules! convertion {
    (fixture: $fixture:literal, sql_type: $sql_type:ident, rust_type: $rust_type:ty, tests: $tests:expr $(,)*) => {
        mod $sql_type {
            $crate::testing::from!(fixture: $fixture, sql_type: $sql_type, rust_type: $rust_type, tests: $tests);
            $crate::testing::to!(fixture: $fixture, sql_type: $sql_type, rust_type: $rust_type, tests: $tests);
        }
    };

    (sql_type: $sql_type:ident, rust_type: $rust_type:ty, tests: $tests:expr $(,)*) => {
        mod $sql_type {
            $crate::testing::from!(sql_type: $sql_type, rust_type: $rust_type, tests: $tests);
            $crate::testing::to!(sql_type: $sql_type, rust_type: $rust_type, tests: $tests);
        }
    };
}

pub use convertion;
