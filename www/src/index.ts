import * as wasm from 'logic-circuit-simulator';

console.log(wasm.encode(new Uint8Array([1, 1, 1])));
(window as any).r = wasm;
