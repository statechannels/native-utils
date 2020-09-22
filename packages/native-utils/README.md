# @statechannels/native-utils

This repository contains native Node.js utilities to speed up encoding and
hashing states, signing states and recovering signer addresses from signed
states.

This package is tries to build native Node.js bindings if Rust is available. If
this is not the case, it falls back to `@statechannels/wasm-utils`.

# License

Copyright &copy; 2020 State Channel contributors.

Licensed under the [MIT License](LICENSE).
