# glsl2wgsl

GLSL to WGSL Compiler using naga

## Building
```sh
git clone
```
```sh
cd glsl2wgsl
```
```sh
wasm-pack build --target web
```
## Usage
```javascript
import init, { glsl2wgsl } from "./glsl2wgsl/pkg/glsl2wgsl.js";
init().then(() => {
    window.glsl2wgsl = glsl2wgsl;
});
```
