[![Latest Version](https://img.shields.io/badge/crates.io-v0.0.2-green.svg)](https://crates.io/crates/rcmath)

# rcmath
*Rust Cryptographic Math Library*

#### Core
- [x] Big unsigned integer (**uint**): [document](./src/uint) Like: U256, U512
- [x] Group (**group**) [document](./src/group)
- [x] Finite field (**ff**) [document](./src/ff)
- [x] Fast Fourier transform (**fft**) [document](./src/fft)
- [x] Pairing (**pairing**) [document](./src/pairing) - optional feature
  - [x] Pairing Curves (**curves**) [document](./src/pairing/curves) - optional feature

#### Auxiliary
- [x] no-std - optional feature
- [x] parallel - optional feature
- [ ] simd - optional feature
- [ ] asm - optional feature
- [x] [serde](https://crates.io/crates/serde) - optional feature

## License

This project is licensed under， it's your choice.

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)
