/*!
# Later Operator

[![docs.rs](https://img.shields.io/docsrs/later_operator.svg?style=flat-square&label=docs.rs)](https://docs.rs/later_operator/)
[![changelog](https://img.shields.io/crates/v/later_operator.svg?style=flat-square&label=changelog&color=9b59b6)](https://github.com/Blobfolio/later_operator/blob/master/CHANGELOG.md)<br>
[![crates.io](https://img.shields.io/crates/v/later_operator.svg?style=flat-square&label=crates.io)](https://crates.io/crates/later_operator)
[![ci](https://img.shields.io/github/actions/workflow/status/Blobfolio/later_operator/ci.yaml?style=flat-square&label=ci)](https://github.com/Blobfolio/later_operator/actions)
[![deps.rs](https://deps.rs/repo/github/blobfolio/later_operator/status.svg?style=flat-square&label=deps.rs)](https://deps.rs/repo/github/blobfolio/later_operator)<br>
[![license](https://img.shields.io/badge/license-wtfpl-ff1493?style=flat-square)](https://en.wikipedia.org/wiki/WTFPL)
[![contributions welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square&label=contributions)](https://github.com/Blobfolio/later_operator/issues)

This library provides a [`ComparisonOperator`] enum that can be used to parse, store, evaluate, and/or stringify Rust's relational operators: `!=`, `<`, `<=`, `==`, `>=`, `>`.

## Examples

```
use later_operator::ComparisonOperator;

// Parse from a string, then compare two arbitrary values.
let op = ComparisonOperator::try_from("<=").unwrap();
assert!(op.compare(&3_u8, &u8::MAX)); // 3 <= 255

// Re-stringify the operator for printing or whatever.
assert_eq!(
    format!("3 {op} 255"),
    "3 <= 255",
);

// Leading/trailing whitespace is ignored when parsing.
assert_eq!(
    ComparisonOperator::try_from("==").unwrap(),
    ComparisonOperator::try_from(" ==\n").unwrap(),
);

// But the value has to make sense or it will fail.
assert!(ComparisonOperator::try_from("~").is_err());
```
*/
#![cfg_attr(feature = "serde", doc = include_str!("../skel/serde.txt"))]

#![forbid(unsafe_code)]

#![warn(
	clippy::filetype_is_file,
	clippy::integer_division,
	clippy::needless_borrow,
	clippy::nursery,
	clippy::pedantic,
	clippy::perf,
	clippy::suboptimal_flops,
	clippy::unneeded_field_pattern,
	macro_use_extern_crate,
	missing_copy_implementations,
	missing_debug_implementations,
	missing_docs,
	non_ascii_idents,
	trivial_casts,
	trivial_numeric_casts,
	unreachable_pub,
	unused_crate_dependencies,
	unused_extern_crates,
	unused_import_braces,
)]

#![allow(clippy::module_name_repetitions)]

#![cfg_attr(docsrs, feature(doc_cfg))]

mod cmp;
mod error;
pub(crate) mod macros;

pub use cmp::ComparisonOperator;
pub use error::Error;
