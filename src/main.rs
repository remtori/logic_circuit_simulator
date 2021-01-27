mod circuit;
use circuit::circuit::*;

fn main() {
    const NAND_GATE: TID = 3;
    let mut nand_gate = LogicCircuit::new(NAND_GATE, 2, 1);
    nand_gate.add(2, Interpreter::AND_GATE);
    nand_gate.add(3, Interpreter::NOT_GATE);
    nand_gate.add_connection(1, 0, 2, 0);
    nand_gate.add_connection(1, 1, 2, 1);
    nand_gate.add_connection(2, 2, 3, 0);
    nand_gate.add_connection(3, 1, 1, 2);

    let mut pins = [false, true, false];
    let mut interpreter = Interpreter::default();

    // println!("Before: {:?}", pins);
    // interpreter.execute(&nand_gate, &mut pins);
    // println!("After: {:?}", pins);

    interpreter.register_circuit(nand_gate);

    const OR_GATE: TID = 4;
    let mut or_gate = LogicCircuit::new(OR_GATE, 2, 1);
    or_gate.add(2, NAND_GATE);
    or_gate.add(3, NAND_GATE);
    or_gate.add(4, NAND_GATE);
    or_gate.add_connection(1, 0, 2, 0);
    or_gate.add_connection(1, 0, 2, 1);
    or_gate.add_connection(1, 1, 3, 0);
    or_gate.add_connection(1, 1, 3, 1);
    or_gate.add_connection(2, 2, 4, 0);
    or_gate.add_connection(3, 2, 4, 1);
    or_gate.add_connection(4, 2, 1, 2);

    println!("Before: {:?}", pins);
    interpreter.execute(&or_gate, &mut pins);
    println!("After: {:?}", pins);
}
