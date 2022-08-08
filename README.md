<div align="center">

  <h1><code>lucky-wheel-wasm</code></h1>

  <strong>This is a project built on WebAssembly and Rust. Using <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>

  <p>Just an example of how to build a WebAssembly project. Audio editing, video editing, image processing can achieve better performance in WebAssembly than in Javascript.</p>

  <h3>
    <a href="https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html">Tutorial</a>
    <span> | </span>
    <a href="https://discordapp.com/channels/442252698964721669/443151097398296587">Chat</a>
  </h3>

  <sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>
</div>

## Usage

### Build with `wasm-pack build`

```
wasm-pack build
```

### Run example of using `lucky-wheel-wasm`

```
cd example

yarn add file:../pkg

yarn dev
```

### Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```
