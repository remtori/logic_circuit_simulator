use logic_circuit_simulator::circuit::*;

fn main() {
    logic_circuit_simulator::initialize();

    let mut simulator = CircuitSimulator::default();
    let mut pins = [true, true, false];

    // let and_gate = simulator.get_circuit(CircuitSimulator::AND_GATE).unwrap();
    // simulator.simulate(&and_gate, &mut pins, 1);
    // assert!(pins.iter().eq([false, false, false].iter()));

    // pins = [true, false, true];
    // simulator.simulate(&and_gate, &mut pins, 1);
    // assert!(pins.iter().eq([true, false, false].iter()));

    // pins = [false, true, true];
    // simulator.simulate(&and_gate, &mut pins, 1);
    // assert!(pins.iter().eq([false, true, false].iter()));

    // pins = [true, true, false];
    // simulator.simulate(&and_gate, &mut pins, 1);
    // assert!(pins.iter().eq([true, true, true].iter()));

    const NAND_GATE: TID = 3;
    let mut nand_gate = LogicCircuit::new(NAND_GATE, 2, 1);
    nand_gate.add(CircuitSimulator::AND_GATE);
    nand_gate.add(CircuitSimulator::NOT_GATE);
    nand_gate.add_connection(1, 0, 2, 0);
    nand_gate.add_connection(1, 1, 2, 1);
    nand_gate.add_connection(2, 2, 3, 0);
    nand_gate.add_connection(3, 1, 1, 2);

    simulator.simulate(&nand_gate, &mut pins, 2);
    simulator.register_circuit(nand_gate);

    const OR_GATE: TID = 4;
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

    // pins = [true, false, false];
    // simulator.simulate(&or_gate, &mut pins, 1);

    // pins = [true, false, true];
    // simulator.simulate_by_tid(CircuitSimulator::AND_GATE, &mut pins, 1);
}
