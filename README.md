nrf51
=====

_nrf51_ contains the peripheral access API for the Nordic Semiconductor NRF51
series microcontroller.

The register definitions were created from the collection of CMSIS SVD files at
[cmsis-svd][] with the help of [svd2rust][] to generate the Rust code. 

[cmsis-svd]: https://github.com/posborne/cmsis-svd.git
[svd2rust]: https://github.com/japaric/svd2rust

`memory.x`
------------

This crate can provide the linker with the locations and sizes of flash and RAM, as long as the relevant chip-variant feature is enabled:

| Feature     | Flash | RAM  |
|-------------|-------|------|
| `memory-aa` | 256KB | 16KB |
| `memory-ab` | 128KB | 16KB |
| `memory-ac` | 256KB | 32KB |

License
-------

[0-clause BSD license](LICENSE-0BSD.txt).
