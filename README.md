# Logic Circuit Simulator

## Specifications

- [ ] Circuit update each tick

- [ ] Circuit can connect to each other and transfer signal

- [ ] Persistent context

    + Each circuit will hold its own pin state

- [ ] Package circuit

    + Independent state and reusable

- [ ] Save & Load

- [ ] Recursive check and optimize

    + Only recursively connecting circuit can preserve its own state

    + Other straight-forward circuit can optimize by using truth table

## Optimize

- `BitVec` instead of `Vec<bool>`

- Cache input as `u32` and only simulate internal if input changed

## Commands

- Build `wasm`

```sh
wasm-pack build -- --features wasm
```
