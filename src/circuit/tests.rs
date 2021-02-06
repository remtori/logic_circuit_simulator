use super::*;

const NAND_GATE: TID = 3;
const OR_GATE: TID = 4;

fn register_nand_gate(simulator: &mut CircuitSimulator) {
    let mut nand_gate = LogicCircuit::new(NAND_GATE, 2, 1);
    nand_gate.add(AND_GATE);
    nand_gate.add(NOT_GATE);

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

    let mut pins = [0, 0, 1];
    simulator.simulate_by_tid(AND_GATE, &mut pins, 1);
    assert!(pins.iter().eq([0, 0, 0].iter()));

    pins = [1, 0, 1];
    simulator.simulate_by_tid(AND_GATE, &mut pins, 1);
    assert!(pins.iter().eq([1, 0, 0].iter()));

    pins = [0, 1, 1];
    simulator.simulate_by_tid(AND_GATE, &mut pins, 1);
    assert!(pins.iter().eq([0, 1, 0].iter()));

    pins = [1, 1, 0];
    simulator.simulate_by_tid(AND_GATE, &mut pins, 1);
    assert!(pins.iter().eq([1, 1, 1].iter()));
}

#[test]
fn default_not_gate() {
    let simulator = CircuitSimulator::default();

    let mut pins = [0, 0];
    simulator.simulate_by_tid(NOT_GATE, &mut pins, 1);
    assert!(pins.iter().eq([0, 1].iter()));

    pins = [1, 0];
    simulator.simulate_by_tid(NOT_GATE, &mut pins, 1);
    assert!(pins.iter().eq([1, 0].iter()));
}

#[test]
fn test_nand_gate() {
    let mut simulator = CircuitSimulator::default();
    register_nand_gate(&mut simulator);

    let mut pins = [0, 0, 0];
    simulator.simulate_by_tid(NAND_GATE, &mut pins, 2);
    assert!(pins.iter().eq([0, 0, 1].iter()));

    let mut pins = [0, 1, 0];
    simulator.simulate_by_tid(NAND_GATE, &mut pins, 2);
    assert!(pins.iter().eq([0, 1, 1].iter()));

    let mut pins = [1, 0, 0];
    simulator.simulate_by_tid(NAND_GATE, &mut pins, 2);
    assert!(pins.iter().eq([1, 0, 1].iter()));

    let mut pins = [1, 1, 0];
    simulator.simulate_by_tid(NAND_GATE, &mut pins, 2);
    assert!(pins.iter().eq([1, 1, 0].iter()));
}

#[test]
fn test_or_gate() {
    let mut simulator = CircuitSimulator::default();
    register_or_gate(&mut simulator);

    let mut pins = [0, 0, 1];
    simulator.simulate_by_tid(OR_GATE, &mut pins, 4);
    assert!(pins.iter().eq([0, 0, 0].iter()));

    let mut pins = [0, 1, 0];
    simulator.simulate_by_tid(OR_GATE, &mut pins, 4);
    assert!(pins.iter().eq([0, 1, 1].iter()));

    let mut pins = [1, 0, 0];
    simulator.simulate_by_tid(OR_GATE, &mut pins, 4);
    assert!(pins.iter().eq([1, 0, 1].iter()));

    let mut pins = [1, 1, 0];
    simulator.simulate_by_tid(OR_GATE, &mut pins, 4);
    assert!(pins.iter().eq([1, 1, 1].iter()));
}
