# Later Operator

[![docs.rs](https://img.shields.io/docsrs/later_operator.svg?style=flat-square&label=docs.rs)](https://docs.rs/later_operator/)
[![changelog](https://img.shields.io/crates/v/later_operator.svg?style=flat-square&label=changelog&color=9b59b6)](https://github.com/Blobfolio/later_operator/blob/master/CHANGELOG.md)<br>
[![crates.io](https://img.shields.io/crates/v/later_operator.svg?style=flat-square&label=crates.io)](https://crates.io/crates/later_operator)
[![ci](https://img.shields.io/github/actions/workflow/status/Blobfolio/later_operator/ci.yaml?style=flat-square&label=ci)](https://github.com/Blobfolio/later_operator/actions)
[![deps.rs](https://deps.rs/repo/github/blobfolio/later_operator/status.svg?style=flat-square&label=deps.rs)](https://deps.rs/repo/github/blobfolio/later_operator)<br>
[![license](https://img.shields.io/badge/license-wtfpl-ff1493?style=flat-square)](https://en.wikipedia.org/wiki/WTFPL)
[![contributions welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square&label=contributions)](https://github.com/Blobfolio/later_operator/issues)

This library provides a `ComparisonOperator` enum that can be used to parse, store, evaluate, and/or print Rust's relational operators: `!=`, `<`, `<=`, `==`, `>=`, `>`.



## Examples

```rust
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

When the optional `serde` crate feature is enabled, `ComparisonOperator` can be de/serialized as a string too:

```rust
use later_operator::ComparisonOperator;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct AffectedVersion {
    operator: ComparisonOperator,
    version: String,
}
```



## Installation

Add `later_operator` to your `dependencies` in `Cargo.toml`, like:

```toml
[dependencies]
later_operator = "0.4.*"
```



## License

Copyright © 2024 [Blobfolio, LLC](https://blobfolio.com) &lt;hello@blobfolio.com&gt;

This work is free. You can redistribute it and/or modify it under the terms of the Do What The Fuck You Want To Public License, Version 2.

    DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
    Version 2, December 2004
    
    Copyright (C) 2004 Sam Hocevar <sam@hocevar.net>
    
    Everyone is permitted to copy and distribute verbatim or modified
    copies of this license document, and changing it is allowed as long
    as the name is changed.
    
    DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
    TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
    
    0. You just DO WHAT THE FUCK YOU WANT TO.
