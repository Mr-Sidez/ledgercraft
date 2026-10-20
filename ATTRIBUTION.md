# Attribution

Every asset file in this repository has a row here: its path, author, source
and licence. `cargo xtask assets` fails the build when a file under `assets/`
has no row. Adding an asset without a row is a bug.

| Path | Author | Source | Licence |
|------|--------|--------|---------|
| _(none yet)_ | | | |

Notes

* LedgerCraft bundles no fonts; it uses egui's built-in fonts.
* UI icons, colours and layouts are defined in code (`crates/ui-egui/src/theme.rs`)
  and are original.
* The sample company and default chart of accounts are original text written
  for LedgerCraft (`crates/engine/src/sample.rs`, `crates/ledger/src/template.rs`).
