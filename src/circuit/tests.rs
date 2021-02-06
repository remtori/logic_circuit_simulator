use super::*;

const NAND_GATE: TID = 3;
const OR_GATE: TID = 4;

fn register_nand_gate(simulator: &mut CircuitSimulator) {
    let mut nand_gate = LogicCircuit::new(NAND_GATE, 2, 1);
    nand_gate.add(CircuitSimulator::AND_GATE);
    nand_gate.add(CircuitSimulator::NOT_GATE);

    nand_gate.add_connection(1, 0, 2, 0);
    nand_gate.add_connection(1, 1, 2, 1);
    nand_gate.add_connection(2, 2, 3, 0);
    nand_gate.add_connection(3, 1, 1, 2);

    simulator.register_circuit(nand_gate);
}

fn register_or_gate(simulator: &mut CircuitSimulator) {
    register_nand_gate(simulator);

    let mut or_gate = LogicCircuit::new(OR_GATE, 2, 1);
    or_gate.add(NAND_GATE);
    or_gate.add(NAND_GATE);
    or_gate.add(NAND_GATE);
    or_gate.add_connection(1, 0, 2, 0);
    or_gate.add_connection(1, 0, 2, 1);
    or_gate.add_connection(1, 1, 3, 0);
    or_gate.add_connection(1, 1, 3, 1);
    or_gate.add_connection(2, 2, 4, 0);
    or_gate.add_connection(3, 2, 4, 1);
    or_gate.add_connection(4, 2, 1, 2);

    simulator.register_circuit(or_gate);
}

#[test]
fn default_and_gate() {
    let simulator = CircuitSimulator::default();
    let and_gate = simulator.get_circuit(CircuitSimulator::AND_GATE).unwrap();

    let mut pins = [false, false, true];
    simulator.simulate(&and_gate, &mut pins, 1);
    assert!(pins.iter().eq([false, false, false].iter()));

    pins = [true, false, true];
    simulator.simulate(&and_gate, &mut pins, 1);
    assert!(pins.iter().eq([true, false, false].iter()));

    pins = [false, true, true];
    simulator.simulate(&and_gate, &mut pins, 1);
    assert!(pins.iter().eq([false, true, false].iter()));

    pins = [true, true, false];
    simulator.simulate(&and_gate, &mut pins, 1);
    assert!(pins.iter().eq([true, true, true].iter()));
}

#[test]
fn default_not_gate() {
    let simulator = CircuitSimulator::default();
    let not_gate = simulator.get_circuit(CircuitSimulator::NOT_GATE).unwrap();

    let mut pins = [false, false];
    simulator.simulate(&not_gate, &mut pins, 1);
    assert!(pins.iter().eq([false, true].iter()));

    pins = [true, false];
    simulator.simulate(&not_gate, &mut pins, 1);
    assert!(pins.iter().eq([true, false].iter()));
}

#[test]
fn test_nand_gate() {
    let mut simulator = CircuitSimulator::default();
    register_nand_gate(&mut simulator);

    let mut pins = [false, false, false];
    simulator.simulate_by_tid(NAND_GATE, &mut pins, 2);
    assert!(pins.iter().eq([false, false, true].iter()));

    let mut pins = [false, true, false];
    simulator.simulate_by_tid(NAND_GATE, &mut pins, 2);
    assert!(pins.iter().eq([false, true, true].iter()));

    let mut pins = [true, false, false];
    simulator.simulate_by_tid(NAND_GATE, &mut pins, 2);
    assert!(pins.iter().eq([true, false, true].iter()));

    let mut pins = [true, true, false];
    simulator.simulate_by_tid(NAND_GATE, &mut pins, 2);
    assert!(pins.iter().eq([true, true, false].iter()));
}

#[test]
fn test_or_gate() {
    let mut simulator = CircuitSimulator::default();
    register_or_gate(&mut simulator);

    let mut pins = [false, false, true];
    simulator.simulate_by_tid(OR_GATE, &mut pins, 4);
    assert!(pins.iter().eq([false, false, false].iter()));

    let mut pins = [false, true, false];
    simulator.simulate_by_tid(OR_GATE, &mut pins, 4);
    assert!(pins.iter().eq([false, true, true].iter()));

    let mut pins = [true, false, false];
    simulator.simulate_by_tid(OR_GATE, &mut pins, 4);
    assert!(pins.iter().eq([true, false, true].iter()));

    let mut pins = [true, true, false];
    simulator.simulate_by_tid(OR_GATE, &mut pins, 4);
    assert!(pins.iter().eq([true, true, true].iter()));
}
