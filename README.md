# Native State Channel Utilities

This repository contains WASM and Node.js native bindings for various state
channel utilities such as hashing states, signing states ad recovering signer
addresses from signed states.

The repository contains one `wasm-utils` directory at the root. This
generates a NPM package for the WASM utilities, which is written to
`packages/wasm-utils`. This needs to be run before anything else.

After this, the package can be treated as a regular Lerna/Yarn monorepo.

The primary package in this repository is `@statechannels/native-utils` in
`packages/native-utils`. This package depends on Neon and
`@statechannels/wasm-utils`. When it is installed, it will try to find
`cargo` (the Rust package manager). If Rust is available, it will build
native Node.js bindings for the state channel utilities. If Rust is not
available, it will fall back to using the WASM package behind the scenes.

## Build

```sh
# This ensures that packages/wasm-utils/ is generated from wasm-utils/
yarn bootstrap
```

## Publish packages to NPM

This will publish `@statechannels/wasm-utils` and `@statechannels/native-utils`.

```sh
yarn publish
```

## Run test suite

This tests the native _and_ WASM bindings.

```sh
yarn test
```

## Run benchmarks

This compares the pre-existing utilities in Nitro Protocol / Server Wallet,
the native bindings (if Rust is available) and the WASM bindings.

## License

Copyright &copy; 2020 State Channels contributors.

Licensed under the [MIT License](./LICENSE).
