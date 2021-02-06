import { h, render } from 'preact';
import { App } from './App';

const wasm = import('logic-circuit-simulator');
wasm.then((m) => {
	m.initialize('DEBUG');
	(window as any).r = m;
});

render(h(App, null), document.body);
