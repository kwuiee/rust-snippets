const fs = require('fs');
const util = require('util');
var source = fs.readFileSync('target/wasm32-unknown-unknown/release/target_wasm_js_import.wasm');
var typedArray = new Uint8Array(source);
const env = {
    memoryBase: 0,
    tableBase: 0,
    memory: new WebAssembly.Memory({
      initial: 256
    }),
    table: new WebAssembly.Table({
      initial: 0,
      element: 'anyfunc'
    })
  }

WebAssembly.instantiate(typedArray, {
  env: env
}).then(result => {
  console.log(util.inspect(result, true, 0));
  console.log(result.instance.exports.add(9, 9));
}).catch(e => {
  // error caught
  console.log(e);
});
