# Proof of concept for ZMK Studio RPC work.

Requires ZMK checked out next to this repository, to access the protobuf `.proto` files
for the RPC message type generation.

## How to install

```sh
npm install
```

## How to run in debug mode

```sh
# Builds the project and opens it in a new browser tab. Auto-reloads when the project changes.
RUSTFLAGS=--cfg=web_sys_unstable_apis npm launch
```
