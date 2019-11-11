#[macro_export]
macro_rules! define_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident { $($variant:ident = $val:expr,)* }
    ) => {
        use diesel::sql_types::Integer;
        use diesel::serialize::ToSql;
        use diesel::deserialize::FromSql;

        // 元の enum を必要な derive とともに定義
        $(#[$meta])*
        #[derive(FromSqlRow, AsExpression)]
        #[sql_type = "Integer"]
        pub enum $name {
            $($variant = $val,)*
        }

        // `ToSql`を定義
        impl<DB: diesel::backend::Backend> ToSql<Integer, DB> for $name {
            fn to_sql<W: std::io::Write>(
                &self,
                out: &mut diesel::serialize::Output<W, DB>,
            ) -> Result<diesel::serialize::IsNull, Box<dyn std::error::Error + Send + Sync>> {
                ToSql::<Integer, DB>::to_sql(&(*self as i32), out)
            }
        }

        // `FromSql`を定義
        impl<DB: diesel::backend::Backend> FromSql<Integer, DB> for $name
        where
            i32: FromSql<Integer, DB>,
        {
            fn from_sql(
                bytes: Option<&DB::RawValue>,
            ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
                use self::$name::*;

                match <i32 as FromSql<Integer, DB>>::from_sql(bytes)? {
                    $($val => Ok($variant),)*
                    s => Err(format!("invalid {} value: {}", stringify!($name), s).into()),
                }
            }
        }
    }
}

use rand::{seq::IteratorRandom, thread_rng, Rng};
static KATAKANAS: &str =
    "アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン";

pub fn generate_random_name(len: usize) -> String {
    let mut rng = thread_rng();
    let len = if len == 0 { rng.gen_range(2, 8) } else { len };
    KATAKANAS
        .chars()
        .choose_multiple(&mut rng, len)
        .into_iter()
        .collect()
}

#[macro_export]
macro_rules! dbg {
    () => (eprint!("\x1b[1;33m[DEBUG  ]\x1b[0m \n"));
    ($fmt:expr) => (eprint!(concat!("\x1b[1;33m[DEBUG  ]\x1b[0;33m ", $fmt, "\x1b[0m\n")));
    ($fmt:expr, $($arg:tt)*) => (eprint!(concat!("\x1b[1;33m[DEBUG  ]\x1b[0;33m ", $fmt, "\x1b[0m\n"), $($arg)*));
}

#[macro_export]
macro_rules! err {
    () => (eprint!("\x1b[1;31m[ERROR  ]\x1b[0m \n"));
    ($fmt:expr) => (eprint!(concat!("\x1b[1;31m[ERROR  ]\x1b[0;31m ", $fmt, "\x1b[0m\n")));
    ($fmt:expr, $($arg:tt)*) => (eprint!(concat!("\x1b[1;31m[ERROR  ]\x1b[0;31m ", $fmt, "\x1b[0m\n"), $($arg)*));
}

#[macro_export]
macro_rules! log {
    ($placeholder:expr) => (eprint!("\x1b[1;32m[{:7}]\x1b[0m \n", $placeholder));
    ($placeholder:expr, $fmt:expr) => (eprint!(concat!("\x1b[1;32m[{:7}]\x1b[0;32m ", $fmt, "\x1b[0m\n"), $placeholder));
    ($placeholder:expr, $fmt:expr, $($arg:tt)*) => (eprint!(concat!("\x1b[1;32m[{:7}]\x1b[0;32m ", $fmt, "\x1b[0m\n"), $placeholder, $($arg)*));
}
