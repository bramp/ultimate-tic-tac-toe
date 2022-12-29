
# Wasm

```shell
# Build the wasm and package into `pkg`.
wasm-pack build

# Start webpack-dev-server to monitor changes.
cd www
npm run start

# To ensure your rust wasm is correctly monitored.
cargo watch -i "pkg/*" \
  -s "wasm-pack build && cp -r pkg/ www/node_modules/ultimate-tic-tac-toe/"
```

# Dev Tools

```shell
cargo install cargo-watch
cargo install cargo-flamegraph
```

# Profiling

```shell
cargo flamegraph --root
```

Games: 2251132 (22511.32/s)
Winners:
    O: 922651 41.0% (goes first)
    X: 827920 36.8% (goes second)
Draws: 500561 22.2%