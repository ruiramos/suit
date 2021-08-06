suit
-------

Exploring the concept of cross platform wasm powered apps.

Current idea would be to have some common core logic (`./suit-logic`) which is then used by thin WASM platform providers that communicate with the native platform.
For web, JS in `./www/index.html` interfaces with WASM from `./www/suit-web`.

