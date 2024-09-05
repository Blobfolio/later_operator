/*!
# Later Operator: Macros
*/

/// # Helper: Generate `AsRef`/`Borrow` impls.
macro_rules! as_ref_borrow {
	($from:ty, $fn:ident, $to:ty) => (
		impl AsRef<$to> for $from {
			#[inline]
			fn as_ref(&self) -> &$to { self.$fn() }
		}
		impl ::std::borrow::Borrow<$to> for $from {
			#[inline]
			fn borrow(&self) -> &$to { self.$fn() }
		}
	);
}

#[cfg(feature = "serde")]
/// # Helper: Deserialize as String.
macro_rules! deserialize_str {
	($ty:ty, $fn:ident) => (
		#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
		impl<'de> ::serde::Deserialize<'de> for $ty {
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where D: ::serde::de::Deserializer<'de> {
				struct Visitor;

				impl<'de> ::serde::de::Visitor<'de> for Visitor {
					type Value = $ty;

					fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
						f.write_str("string")
					}

					fn visit_str<S>(self, src: &str) -> Result<$ty, S>
					where S: ::serde::de::Error {
						<$ty>::$fn(src).map_err(::serde::de::Error::custom)
					}

					fn visit_bytes<S>(self, src: &[u8]) -> Result<$ty, S>
					where S: ::serde::de::Error {
						<$ty>::$fn(src).map_err(::serde::de::Error::custom)
					}
				}

				deserializer.deserialize_str(Visitor)
			}
		}
	);
}

/// # Helper: Display as String.
macro_rules! display_str {
	($ty:ty, $fn:ident) => (
		impl ::std::fmt::Display for $ty {
			#[inline]
			fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
				f.write_str(self.$fn())
			}
		}
	);
}

/// # Helper: From String.
macro_rules! from_str {
	($ty:ty, $fn:ident) => (
		impl ::std::str::FromStr for $ty {
			type Err = $crate::Error;
			#[inline]
			fn from_str(src: &str) -> Result<Self, Self::Err> { Self::$fn(src) }
		}
	);
}

/// # Helper: Symmetrical `PartialEq`.
macro_rules! partial_eq {
	($from:ty, $fn:ident, $to:ty) => (
		impl PartialEq<$to> for $from {
			#[inline]
			fn eq(&self, other: &$to) -> bool { self.$fn() == other }
		}
		impl PartialEq<$from> for $to {
			#[inline]
			fn eq(&self, other: &$from) -> bool { self == other.$fn() }
		}
	);
}

#[cfg(feature = "serde")]
/// # Helper: Serialize as String.
macro_rules! serialize_str {
	($ty:ty, $fn:ident) => (
		#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
		impl ::serde::Serialize for $ty {
			fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
			where S: ::serde::ser::Serializer { self.$fn().serialize(serializer) }
		}
	);
}



/// # Re-export.
pub(super) use {
	as_ref_borrow,
	display_str,
	from_str,
	partial_eq,
};
#[cfg(feature = "serde")] pub(super) use {
	deserialize_str,
	serialize_str,
};
