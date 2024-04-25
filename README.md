# Shivarthu client based on leptos

Website: <https://shivarthu.on-fleek.app/>

## Change the auth file

`src/constants/auth.rs.bak`

Change the auth.rs.bak file to auth.rs and give the authentication token

```
cargo install --locked trunk
cd shivarthu-client
trunk serve
```

## Tailwindcss

```bash

npx tailwindcss -i ./src/input.css -o ./css/output.css --watch

```
## Start substrate node
```bash
./target/release/node-template --dev --tmp
```

## daisyUI

```bash
npm i -D daisyui@latest
npm i -D flowbite
```

## Leptos format

### Examples

**Single file**

Format a specific file by name

`leptosfmt ./examples/counter/src/lib.rs`

**Current directory**

Format all .rs files within the current directory

`leptosfmt .`

**Directory**

Format all .rs files within the examples directory

`leptosfmt ./examples`

**Glob**

Format all .rs files ending with `_test.rs` within the examples directory

`leptosfmt ./examples/**/*_test.rs`

## Rust format check

`cargo fmt --all -- --check`

## Rust format

`cargo fmt --all`

## Format a file

`rustfmt src/main.rs `

<!-- ## rpc call to substrate
## transaction
## storage call
## ipfs file upload -->

## Metadata download

```bash
cargo install subxt-cli
```

```bash
subxt metadata -f bytes > metadata.scale
```
