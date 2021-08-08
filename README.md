suit
-------

Exploring the concept of cross platform wasm powered apps.

My current goal is to build a web, Android and iOS (native) apps that all depend on the same core business logic using WASM.

 - [/suit-logic](./suit-logic) holds the business logic - currently a counter but hopefully soon something more interesting (todos? 😬)

 - [/www](./www) holds the web version. It uses a "WASM platform provider" that's specific to the platform (web/JS - [suit-web](./www/suit-web/src)) that uses the core one, but the rendering is done on the native platform - the browser/JS, currently using React.

## Running

### Web version

1. Build the `suit-web` WASM module
```shell
cd www/suit-web
wasm-pack build --target web
```

2. Build the JS app
```shell
cd .. # back to www/
npm install
npm run build # or 'npm run dev' for watch
```

3. Serve the static web app
```shell
npx serve
```

For development, you can also watch the Rust files with [cargo-watch](https://crates.io/crates/cargo-watch):
```shell
# from www/suit-web
 cargo watch --ignore pkg/ -s 'wasm-pack build --target web'
```

## TODOs / Open questions
 - [x] Add persistence to the app with a platform specific persistence provider (ie localstorage for web)
 - [ ] [web] Figure out why `--target bundle` doesn't work with esbuild and if that's something we should be using here
 - [ ] [web] Come up with a better way to send the closures to the WebClient
 - [ ] [iOS] Build PoC with Swift
 - [ ] [Android] Build PoC with Kotlin
