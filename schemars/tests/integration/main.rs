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
    pub use serde_json::json;
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
                type_name_of_val(f)
                    .trim_end_matches("::f")
                    .trim_end_matches("::{{closure}}")
                    .trim_start_matches("integration::")
            },
            schemars::generate::SchemaSettings::default(),
        )
    };
}
