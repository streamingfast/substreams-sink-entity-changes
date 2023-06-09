# Change log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.2.1]

* Speed up `tables::Tables#to_entity_changes` by removing multiple `clone` that were not required.

## [1.2.0]

* Added `tables` module so that you can use it as a better abstraction to build up your entity changes.

  ```rust
  let mut tables = Tables::new();

  // Create a row (<entity_name>, <id>).[set(<column>, <value>), ...]
  tables
    .create_row("Factory", id)
    .set("poolCount", &bigint0)
    .set("txCount", &bigint0)
    .set("totalVolumeUSD", &bigdecimal0)
    .set("totalVolumeETH", &bigdecimal0)
    .set("totalFeesUSD", &bigdecimal0)
    .set("totalFeesETH", &bigdecimal0)
    .set("untrackedVolumeUSD", &bigdecimal0)
    .set("totalValueLockedUSD", &bigdecimal0)
    .set("totalValueLockedETH", &bigdecimal0)
    .set("totalValueLockedUSDUntracked", &bigdecimal0)
    .set("totalValueLockedETHUntracked", &bigdecimal0)
    .set("owner", &format!("0x{}", Hex(utils::ZERO_ADDRESS).to_string()));

  // Update a row (<entity_name>, <id>).[set(<column>, <value>), ...]
  tables
    .update_row("Bundle", "1").set("ethPriceUSD", &delta.new_value);
  ```

* Added clean invalid string byte sequence when using a `String`, `&String`, `Delta<String>`, `Delta<&String>`, `Vec<String>`, `&Vec<String>`, `DeltaArray<String>` and `&DeltaArray<String>`

## [1.1.1]

* Added implementations on the ToField trait used by the EntityChange struct for BigInt, BigDecimal, and Bytes arrays.

## [1.1.0]

### Deprecation

* **Deprecated** `sf.substreams.entity.v1.EntityChange#ordinal`, this is not used in `graph-node`.
* **Deprecated** `sf.substreams.entity.v1.Field#old_value`, this is not used in `graph-node`.

#### Added

* Added `FINAL` to `sf.substreams.entity.v1.EntityChange#Operation` enum list.

## [1.0.1](https://github.com/streamingfast/substreams-entity-change/releases/tag/v1.0.1)
* Adding `FINAL` entity change operation

## [1.0.0](https://github.com/streamingfast/substreams-entity-change/releases/tag/v1.0.0)

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
