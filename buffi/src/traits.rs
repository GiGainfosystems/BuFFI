use std::borrow::Cow;
use std::path::PathBuf;

/// This is a marker trait for allowed type mappings between
/// Rust and C++ that are considered safe. It needs to be satisfied
/// to allow `#[buffi(type = Foo)]` to work
///
/// It's fine to add new implementations of this trait for third
/// party traits if you verified that bincode allows to interchange
/// these types.
///
/// It's also fine to add new implementations to buffi itself if
/// that invariant is uphold.
///
/// Not upholding this invariant either results in a deserialization
/// failure or silent data corruption as bincode might interpret the
/// provided byte array differently. As this is not a memory safety issue
/// this is still considered to be a safe trait.
#[diagnostic::on_unimplemented(
    message = "cannot map `{Self}` to `{Rt}` as this is not a safe mapping",
    note = "make sure that the bincode encoding of both types are equivalent before implementing the necessary trait"
)]
pub trait SafeTypeMapping<Rt> {}

impl<'a> SafeTypeMapping<Cow<'a, str>> for String {}
impl<'a> SafeTypeMapping<Option<Cow<'a, str>>> for String {}
impl<'a> SafeTypeMapping<Vec<Cow<'a, str>>> for Vec<String> {}

impl<'a, T> SafeTypeMapping<Cow<'a, [T]>> for Vec<T> where [T]: ToOwned {}
impl<'a, T> SafeTypeMapping<Option<Cow<'a, [T]>>> for Vec<T> where [T]: ToOwned {}
impl<'a, T> SafeTypeMapping<Vec<Cow<'a, [T]>>> for Vec<Vec<T>> where [T]: ToOwned {}

impl SafeTypeMapping<PathBuf> for String {}
impl SafeTypeMapping<Option<PathBuf>> for String {}
impl SafeTypeMapping<Vec<PathBuf>> for Vec<String> {}

// all types are compatible with themself
impl<T> SafeTypeMapping<T> for T {}

#[cfg(feature = "url2")]
impl SafeTypeMapping<url::Url> for String {}
#[cfg(feature = "url2")]
impl SafeTypeMapping<Option<url::Url>> for String {}
#[cfg(feature = "url2")]
impl SafeTypeMapping<Vec<url::Url>> for Vec<String> {}
