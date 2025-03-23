# Simple webpack by rust

### 1. get module path

### 2. get code by path

### 3. build dep graph

### 4. output as a file

### current result

```js
(function (modules) {
  function require(moduleId) {
    var module = { exports: {} };
    modules[moduleId](module, module.exports, require);
    return module.exports;
  }
  require(0);
})({
  0: function (module, exports, require) {
    export function greet() {
      console.log("Hello, Webpack in Rust!");
    }
  },
  1: function (module, exports, require) {
    import { greet } from "./greet.js";
    greet();
  },
});
```

### todo -> import && export
