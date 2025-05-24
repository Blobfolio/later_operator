/*!
# Later Operator: Comparison Operator.
*/

use crate::Error;
use std::{
	borrow::Borrow,
	fmt,
	str::FromStr,
};



#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// # Comparison Operators.
///
/// This enum holds a comparison operator — `!=`, `<`, `<=`, `==`, `>=`, `>` —
/// that can be used to programmatically compare two values.
///
/// Variants can be parsed from string slices using [`FromStr`](#impl-FromStr-for-ComparisonOperator) or [`TryFrom<&str>`](#impl-TryFrom<%26str>-for-ComparisonOperator), or
/// byte slices via [`TryFrom<&[u8]>`](#impl-TryFrom<%26[u8]>-for-ComparisonOperator), and converted to a string slice or byte slice
/// using [`ComparisonOperator::as_str`] or [`ComparisonOperator::as_bytes`] respectively.
///
/// If the `serde` crate feature is enabled, it can also be de/serialized
/// from/to its string representations.
///
/// Use [`ComparisonOperator::compare`] to thusly compare any two values (so
/// long as the type implements [`PartialOrd`](std::cmp::PartialOrd)).
///
/// ## Examples
///
/// ```
/// use later_operator::ComparisonOperator;
///
/// // Parse from a string, then compare two arbitrary values.
/// let op = ComparisonOperator::try_from("<=").unwrap();
/// assert!(op.compare(&3_u8, &u8::MAX)); // 3 <= 255
///
/// // Re-stringify the operator for printing or whatever.
/// assert_eq!(
///     format!("3 {op} 255"),
///     "3 <= 255",
/// );
///
/// // Leading/trailing whitespace is ignored when parsing.
/// assert_eq!(
///     ComparisonOperator::try_from("==").unwrap(),
///     ComparisonOperator::try_from(" ==\n").unwrap(),
/// );
///
/// // But the value has to make sense or it will fail.
/// assert!(ComparisonOperator::try_from("~").is_err());
/// ```
#[cfg_attr(feature = "serde", doc = include_str!("../skel/serde.txt"))]
pub enum ComparisonOperator {
	/// # Not Equal To.
	Ne,

	/// # Less Than.
	Lt,

	/// # Less Than or Equal To.
	Le,

	/// # Equal To.
	Eq,

	/// # Greater Than or Equal To.
	Ge,

	/// # Greater Than.
	Gt,
}

impl AsRef<[u8]> for ComparisonOperator {
	#[inline]
	fn as_ref(&self) -> &[u8] { self.as_bytes() }
}

impl AsRef<str> for ComparisonOperator {
	#[inline]
	fn as_ref(&self) -> &str { self.as_str() }
}

impl Borrow<str> for ComparisonOperator {
	#[inline]
	fn borrow(&self) -> &str { self.as_str() }
}

impl fmt::Display for ComparisonOperator {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		<str as fmt::Display>::fmt(self.as_str(), f)
	}
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de> ::serde::Deserialize<'de> for ComparisonOperator {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where D: ::serde::de::Deserializer<'de> {
		/// # Visitor.
		struct Visitor;

		impl ::serde::de::Visitor<'_> for Visitor {
			type Value = ComparisonOperator;

			#[inline]
			fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				f.write_str("string")
			}

			#[inline]
			fn visit_str<S>(self, src: &str) -> Result<ComparisonOperator, S>
			where S: ::serde::de::Error {
				<ComparisonOperator>::try_from(src).map_err(::serde::de::Error::custom)
			}

			#[inline]
			fn visit_bytes<S>(self, src: &[u8]) -> Result<ComparisonOperator, S>
			where S: ::serde::de::Error {
				<ComparisonOperator>::try_from(src).map_err(::serde::de::Error::custom)
			}
		}

		deserializer.deserialize_str(Visitor)
	}
}

impl FromStr for ComparisonOperator {
	type Err = Error;

	#[inline]
	fn from_str(src: &str) -> Result<Self, Self::Err> {
		Self::try_from(src.as_bytes())
	}
}

impl PartialEq<str> for ComparisonOperator {
	#[inline]
	fn eq(&self, other: &str) -> bool { self.as_str() == other }
}

impl PartialEq<ComparisonOperator> for str {
	#[inline]
	fn eq(&self, other: &ComparisonOperator) -> bool { self == other.as_str() }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl ::serde::Serialize for ComparisonOperator {
	#[inline]
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: ::serde::ser::Serializer { self.as_str().serialize(serializer) }
}

impl TryFrom<&[u8]> for ComparisonOperator {
	type Error = Error;

	#[inline]
	fn try_from(src: &[u8]) -> Result<Self, Self::Error> {
		match src.trim_ascii() {
			b"!=" =>        Ok(Self::Ne),
			b"<"  =>        Ok(Self::Lt),
			b"<=" =>        Ok(Self::Le),
			b"==" | b"=" => Ok(Self::Eq),
			b">=" =>        Ok(Self::Ge),
			b">"  =>        Ok(Self::Gt),
			_     =>        Err(Error),
		}
	}
}

impl TryFrom<&str> for ComparisonOperator {
	type Error = Error;

	#[inline]
	fn try_from(src: &str) -> Result<Self, Self::Error> { Self::try_from(src.as_bytes()) }
}

impl ComparisonOperator {
	#[must_use]
	/// # As Bytes.
	///
	/// Return the operator as a byte slice.
	///
	/// ## Examples
	///
	/// ```
	/// use later_operator::ComparisonOperator;
	///
	/// assert_eq!(ComparisonOperator::Eq.as_bytes(), b"==");
	/// ```
	pub const fn as_bytes(self) -> &'static [u8] {
		match self {
			Self::Lt => b"<",
			Self::Le => b"<=",
			Self::Eq => b"==",
			Self::Ge => b">=",
			Self::Gt => b">",
			Self::Ne => b"!=",
		}
	}

	#[must_use]
	/// # As Str.
	///
	/// Return the operator as a string slice.
	///
	/// ## Examples
	///
	/// ```
	/// use later_operator::ComparisonOperator;
	///
	/// assert_eq!(ComparisonOperator::Eq.as_str(), "==");
	/// ```
	pub const fn as_str(self) -> &'static str {
		match self {
			Self::Lt => "<",
			Self::Le => "<=",
			Self::Eq => "==",
			Self::Ge => ">=",
			Self::Gt => ">",
			Self::Ne => "!=",
		}
	}

	#[must_use]
	/// # Is Empty.
	///
	/// This method is added only for consistency; it always returns `false`.
	pub const fn is_empty(self) -> bool { false }

	#[must_use]
	/// # Length.
	///
	/// Return the byte length of the operator's string representation.
	pub const fn len(self) -> usize {
		match self {
			Self::Lt | Self::Gt => 1,
			_ => 2,
		}
	}
}

/// # Helper: Generate Is Equal/Less/Etc. Methods.
macro_rules! is {
	($name:literal, $fn:ident, $ty:ident) => (
		#[must_use]
		#[doc = concat!("# Is ", $name, "?")]
		#[doc = ""]
		#[doc = concat!("Returns `true` for [`ComparisonOperator::", stringify!($ty), "`].")]
		pub const fn $fn(self) -> bool { matches!(self, Self::$ty) }
	);
}

impl ComparisonOperator {
	is!("Not Equal",             is_ne, Ne);
	is!("Less Than",             is_lt, Lt);
	is!("Less Than/Equal To",    is_le, Le);
	is!("Equal",                 is_eq, Eq);
	is!("Greater Than/Equal To", is_ge, Ge);
	is!("Greater Than",          is_gt, Gt);
}

impl ComparisonOperator {
	/// # Compare.
	///
	/// Compare `lhs` to `rhs` using a given operator.
	///
	/// ## Examples
	///
	/// ```
	/// use later_operator::ComparisonOperator;
	///
	/// let a: u32 = 50;
	/// let b: u32 = 60;
	///
	/// assert_eq!(ComparisonOperator::Ne.compare(&a, &b), true);  // a != b
	/// assert_eq!(ComparisonOperator::Lt.compare(&a, &b), true);  // a < b
	/// assert_eq!(ComparisonOperator::Le.compare(&a, &b), true);  // a <= b
	/// assert_eq!(ComparisonOperator::Eq.compare(&a, &b), false); // a == b
	/// assert_eq!(ComparisonOperator::Ge.compare(&a, &b), false); // a >= b
	/// assert_eq!(ComparisonOperator::Gt.compare(&a, &b), false); // a > b
	/// ```
	pub fn compare<T: PartialOrd>(self, lhs: &T, rhs: &T) -> bool {
		match self {
			Self::Ne => lhs != rhs,
			Self::Lt => lhs < rhs,
			Self::Le => lhs <= rhs,
			Self::Eq => lhs == rhs,
			Self::Ge => lhs >= rhs,
			Self::Gt => lhs > rhs,
		}
	}
}



#[cfg(test)]
mod tests {
	use super::*;
	use serde as _;
	use serde_json as _;

	#[test]
	fn t_parse() {
		macro_rules! test {
			($ty:ident, $val:literal) => (
				// TryFrom<str>.
				assert_eq!(
					Ok(ComparisonOperator::$ty),
					ComparisonOperator::try_from($val)
				);

				// FromStr<str>.
				assert_eq!(
					Ok(ComparisonOperator::$ty),
					ComparisonOperator::from_str($val)
				);

				// TryFrom<[u8]>
				assert_eq!(
					Ok(ComparisonOperator::$ty),
					ComparisonOperator::try_from($val.as_bytes())
				);

				// TryFroms with leading/trailing whitespace.
				assert_eq!(
					Ok(ComparisonOperator::$ty),
					ComparisonOperator::try_from(concat!(" ", $val, " "))
				);
				assert_eq!(
					Ok(ComparisonOperator::$ty),
					ComparisonOperator::try_from(concat!(" ", $val, " ").as_bytes())
				);
			)
		}

		test!(Ne, "!=");
		test!(Lt, "<");
		test!(Le, "<=");
		test!(Eq, "==");
		test!(Eq, "=");
		test!(Ge, ">=");
		test!(Gt, ">");
	}

	#[test]
	fn t_as() {
		// Redundantly check as_str, as_bytes sanity.
		for (op, v) in [
			(ComparisonOperator::Ne, "!="),
			(ComparisonOperator::Lt, "<"),
			(ComparisonOperator::Le, "<="),
			(ComparisonOperator::Eq, "=="),
			(ComparisonOperator::Ge, ">="),
			(ComparisonOperator::Gt, ">"),
		] {
			assert_eq!(op.as_str(), v);
			assert_eq!(op.as_bytes(), v.as_bytes());
			assert_eq!(op.as_bytes().len(), op.len());
		}
	}

	#[cfg(feature = "serde")]
	#[test]
	fn t_serde() {
		for op in [
			ComparisonOperator::Ne,
			ComparisonOperator::Lt,
			ComparisonOperator::Le,
			ComparisonOperator::Eq,
			ComparisonOperator::Ge,
			ComparisonOperator::Gt,
		] {
			// Serialization should look like as_str with extra quotes.
			let s = serde_json::to_string(&op).expect("Serialization failed.");
			assert_eq!(s, format!("{:?}", op.as_str()));

			// Deserialization should give us the original value.
			let d: ComparisonOperator = serde_json::from_str(&s).expect("Deserialization failed.");
			assert_eq!(op, d);
		}
	}

	#[test]
	fn t_compare() {
		const SET: [u8; 7] = [0, 1, 2, 3, 4, 5, 6];

		macro_rules! count {
			($op:ident, $lhs:literal) => (
				SET.iter().filter(|v| ComparisonOperator::$op.compare(&$lhs, v)).count()
			);
		}

		// Compare the middle value against the ordered set.
		assert_eq!(count!(Ne, 3), 6);
		assert_eq!(count!(Lt, 3), 3);
		assert_eq!(count!(Le, 3), 4);
		assert_eq!(count!(Eq, 3), 1);
		assert_eq!(count!(Ge, 3), 4);
		assert_eq!(count!(Gt, 3), 3);

		// Check highs and lows to make sure we implemented the less/more
		// comparisons in the *correct* order. ;)
		assert_eq!(count!(Lt, 0), 6);
		assert_eq!(count!(Le, 0), 7);
		assert_eq!(count!(Ge, 6), 7);
		assert_eq!(count!(Gt, 6), 6);
	}
}

