/*!
# Later Operator: Errors
*/

use crate::macros;



#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// # Error.
pub enum Error {
	ComparisonOperator,
}

macros::as_ref_borrow!(Error, as_str, str);
macros::display_str!(Error, as_str);

impl std::error::Error for Error {}

impl Error {
	#[must_use]
	/// # As Str.
	///
	/// Return the error as a string slice.
	pub const fn as_str(self) -> &'static str {
		match self {
			Self::ComparisonOperator => "unable to parse comparison operator",
		}
	}
}
