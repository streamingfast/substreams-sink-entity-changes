# Change log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0]

* **BREAKING** Changed database proto package path from ~~substreams.entity.v1~~ to `sf.substreams.entity.v1`


## [0.3.0](https://github.com/streamingfast/substreams-entity-change/releases/tag/v0.3.0)

* Bump `substreams` library to `0.5.0`

## [0.2.1](https://github.com/streamingfast/substreams-entity-change/releases/tag/v0.2.1)

* Added support for `u64` converted to `Typed::Bigint`.
* Added possibility to record a `change` using `new: Into<Typed>` to simulate a value creation.
* Added possibility to record a `change` using `(old: Into<Typed>, new: Option<Into<Typed>>)` to simulate a value deletion.
* Added possibility to record a `change` using `(old: Into<Typed>, new: Into<Typed>)`.
* Refactored to allow delta to be taken from any `Into<Typed>` which makes it much easier to extend when there is missing types.
* Refactored to reduce amount of `clone` perform.
* Add `f64` and `i64` and there respective Deltas to be converted to `BigDecimal` and `BigInt`

## [0.2.0](https://github.com/streamingfast/substreams-entity-change/releases/tag/v0.2.0)

* Replace `DeltaI32` with `DeltaInt32`
* Bump `substreams`

## [0.1.0](https://github.com/streamingfast/substreams-entity-change/releases/tag/v0.1.0)

* Added `entity.proto` containing proto message definitions
* Added `change.rs` containing struct definitions for `EntityChange` types and `helper.rs` containing helpers for respective struct definitions for `EntityChange`
