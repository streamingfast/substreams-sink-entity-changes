## Release Process

### Instructions

> **Warning** Do not forget to replace `${version}` by your real version like `0.1.3` in the commands below!  (`export version=0.1.3`)

You will need [sfreleaser](https://github.com/streamingfast/sfreleaser) (install from source using Golang with `go install github.com/streamingfast/sfreleaser@latest`) to perform the release process.

- Find & replace all occurrences of Regex `^version = "[^"]+"` in all `Cargo.toml` files to `version = "${version}"` and update the [CHANGELOG.md](CHANGELOG.md) to update the `## Unreleased` header to become `## ${version}`

  Using [sd](https://github.com/chmln/sd):

  ```bash
  find . -type f -name Cargo.toml -not -path "./target/*" | xargs -n1 sd '^version = "[^"]+"' "version = \"${version}\""
  sd '## Unreleased' "## ${version}" CHANGELOG.md
  ```

- Commit everything with message `Preparing release of ${version}`.

  ```bash
  git add -A . && git commit -m "Preparing release of ${version}"
  ```

- Perform release

  ```bash
  sfreleaser release
  ```

