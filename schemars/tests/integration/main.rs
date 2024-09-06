#[cfg(feature = "arrayvec07")]
mod arrayvec;
mod bound;
#[cfg(feature = "bytes1")]
mod bytes;
#[cfg(feature = "chrono04")]
mod chrono;

mod util;

mod prelude {
    pub use crate::test;
    pub use crate::util::{arbitrary_nonstring_values, arbitrary_values};
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
}

#[macro_export]
macro_rules! test {
    ($type:ty) => {
        $crate::util::Test::<$type>::new(
            {
                fn f() {}
                fn type_name_of_val<T>(_: T) -> &'static str {
                    core::any::type_name::<T>()
                }
                let test_name = type_name_of_val(f)
                    .trim_end_matches("::f")
                    .split("::")
                    .last()
                    .unwrap();

                format!("{}~{}", core::file!(), test_name)
            },
            schemars::generate::SchemaSettings::default(),
        )
    };
}
