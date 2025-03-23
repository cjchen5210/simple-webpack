(function(modules) {
  function require(moduleId) {
    var module = { exports: {} };
    modules[moduleId](module, module.exports, require);
    return module.exports;
  }
  require(0);
})({
  0: function(module, exports, require) {
    import { greet } from './greet.js';
greet();
  },
  1: function(module, exports, require) {
    export function greet() {
  console.log("Hello, Webpack in Rust!");
}
  },
});
