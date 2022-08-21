# `efm32hg309f64-pac`

Peripheral access API for efm32hg309f64 microcontroller from Silicon Labs' efm32hg family.

The efm32hg register definitions were retrieved from from keil.com and provided here in ./svd as a convenience only.

Additionally, this crate includes SVD patch for DEVINFO register which isn't included in the original SVD file from keil.

## Supported Series
Currently supported and tested series:

- efm32hg309f64

Other HG series might be able to use this crate but is not supported until it's stated otherwise.

## [Documentation](https://docs.rs/efm32hg309f64-pac)

## Requirements
The crate can be used with Rust v1.56 or newer.

## Regenerate
The src directory is generated. If you want to upstream changes here, don't manually edit, but rather send a PR to one of the upstream tools used in tools/gen. However if you have tools or svd version bumps you can regenerate this pac. The script needs to apply patches to the svd via the Makefile which requires [svdtools](https://github.com/stm32-rs/svdtools) to be installed. Then you can run `make` to patch, generate and format the pac from source svd.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
