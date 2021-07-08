# my-algo-rust-adapter

Rust wrapper library to use [My Algo wallet](https://github.com/randlabs/myalgo-connect) in Rust WASM apps.

```js
my_algo = { git = "https://github.com/i-schuetz/my-algo-rust-adapter", branch = "main" }
```

(Releases and publishing to crates.io on demand)

Last tested My Algo version: 1.0.2

Currently [My Algo is not bundled](https://github.com/i-schuetz/my-algo-rust-adapter/issues/3). It's recommended to use this [template project](https://github.com/i-schuetz/algonaut-myalgo-yew-template), which works out of the box.

⚠️ [Low test coverage](https://github.com/i-schuetz/my-algo-rust-adapter/issues/2). Some transaction types have not been tested at all. Issues should be easy to debug, [here](https://github.com/i-schuetz/my-algo-rust-adapter/blob/main/src/to_my_algo_transaction.rs) are the mappings to the JSON expected by My Algo. Or open an issue!

### Contributing

1. Fork
2. Commit changes to a branch in your fork
3. Push your code and make a pull request
