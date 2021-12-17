# nrc-721-template

Add custom behavior:

* edit 'contracts/custom_nft/src/entry.rs'
* choose base functionality adding or removing base behaviors to 'Composite script' section.
* add desired behavior to 'Custom behavior' section.

Build contracts:

```sh
capsule build
```

Run tests:

```sh
capsule test
```

Build release contracts:

```sh
capsule build --release
```