# <img src="logo.svg" alt="Rather Labs" height="40px">

**nrc-721-template | Template for custom NFT implementation for the Nervos Network.**

[![Gitpod
Ready-to-Code](https://img.shields.io/badge/Gitpod-Ready--to--Code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/rather-labs/nrc-721-template)

# Requirements

```sh
cargo install ckb-capsule
```

# Implementation procedure

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

## License

Rather Labs NRC-721 is released under the [MIT License](LICENSE).