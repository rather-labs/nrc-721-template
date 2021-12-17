# <img src="logo.svg" alt="Rather Labs" height="40px">

**nrc-721-template | Template for custom NFT implementation for the Nervos Network.**

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