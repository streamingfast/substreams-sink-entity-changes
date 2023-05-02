# Substreams Entity Change

`substreams-entity-change` crate contains all the definitions for the entity changes which can be emitted by a Substreams and can then be ingested by `graph-node`.

## Usage

We don't have proper documentation yet for this module. The best place to understand how to use it is to look at existing Substreams that emits EntityChanges to be consumed by `graph-node`.

Substreams:
- [substreams-eth-block-meta](https://github.com/streamingfast/substreams-eth-block-meta) (see [graph_out module](https://github.com/streamingfast/substreams-eth-block-meta/blob/master/src/lib.rs#L62) and its accompanying helper file [graph_out.rs](https://github.com/streamingfast/substreams-eth-block-meta/blob/master/src/graph_out.rs#L6))
- [substreams-uniswap-v3](https://github.com/streamingfast/substreams-uniswap-v3) (see [graph_out module](https://github.com/streamingfast/substreams-uniswap-v3/blob/bc2dc1d88d3e7297b15f67bb4cdb81702396f4f7/src/lib.rs#L1305) and its accompanying helper file [db.rs](https://github.com/streamingfast/substreams-uniswap-v3/blob/develop/src/db.rs))

### Contributing

#### Re-generate Protobuf definitions

Install [buf CLI](https://buf.build/product/cli/) and run [./proto/generate.sh](./proto/generate.sh) file.

#### Release

See [RELEASE.md](./RELEASE.md) for release process.