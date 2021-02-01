import { h, render } from 'preact';
import * as wasm from 'logic-circuit-simulator';
import { App } from './App';

console.log(wasm.encode(new Uint8Array([1, 1, 1])));
(window as any).r = wasm;

render(h(App, null), document.body);
