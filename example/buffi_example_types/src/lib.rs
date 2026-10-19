//! Types that are defined outside the API crate but used in the API

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::time::Duration;

/// Helper types that are not used by the API crate directly
pub mod common {
    use serde::{Deserialize, Serialize};
    use std::borrow::Cow;
    use std::time::Duration;

    /// A proxy type for (de)serializing a `Duration`
    #[derive(Serialize, Deserialize)]
    #[serde(remote = "Duration")]
    pub struct DurationHelper {
        /// whole seconds of the duration
        #[serde(getter = "Duration::as_secs")]
        pub secs: u64,
        /// fractional part of the duration in nanoseconds
        #[serde(getter = "Duration::subsec_nanos")]
        pub nanos: u32,
    }

    impl From<DurationHelper> for Duration {
        fn from(value: DurationHelper) -> Self {
            Duration::new(value.secs, value.nanos)
        }
    }

    /// A label that has the same binary representation as a string
    #[derive(Serialize, Deserialize)]
    pub struct Label {
        /// the text of the label
        pub text: String,
    }

    impl<'a> buffi::SafeTypeMapping<Cow<'a, str>> for Label {}
}

/// A type from another crate that uses types from its own crate in attributes
#[derive(Serialize, Deserialize, buffi::Annotation)]
pub struct Timeout {
    /// A normal struct field without any attributes
    pub retries: u32,
    /// A struct field using a proxy type that's only referenced via `crate::`
    #[serde(with = "crate::common::DurationHelper")]
    pub duration: Duration,
    /// A type overwrite with a type that's only referenced via `crate::`
    #[buffi(type = crate::common::Label)]
    pub label: Cow<'static, str>,
}
