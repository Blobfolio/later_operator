/*!
# Later Operator: Errors
*/

use std::{
	borrow::Borrow,
	fmt,
};



#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// # Error.
pub struct Error;

impl AsRef<str> for Error {
	#[inline]
	fn as_ref(&self) -> &str { self.as_str() }
}

impl Borrow<str> for Error {
	#[inline]
	fn borrow(&self) -> &str { self.as_str() }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.pad(self.as_str()) }
}

impl Error {
	#[must_use]
	/// # As Str.
	///
	/// Return the error as a string slice.
	pub const fn as_str(self) -> &'static str { "unable to parse comparison operator" }
}
