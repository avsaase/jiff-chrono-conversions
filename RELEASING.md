# Releasing

To make a new release:

1. Bump `version` in `Cargo.toml`.
2. Add a new entry to `CHANGELOG.md` for the new version.
3. Run `cargo reedme` to sync `README.md` with the crate docs, if they changed.
4. Describe and push: `jj describe -m "release vX.Y.Z"` then `jj git push`.
5. Tag the commit: `jj tag set vX.Y.Z -r @` (requires jj ≥0.44) and push it: `jj git push`.
6. Publish: `cargo publish`.
