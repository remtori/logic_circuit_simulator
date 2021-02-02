#[cfg(test)]
pub mod tests {

    use logic_circuit_simulator::circuit::*;
    const NAND_GATE: TID = 3;
    const OR_GATE: TID = 4;

    #[test]
    fn default_and_gate() {
        let interpreter = Interpreter::default();
        let and_gate = interpreter.get_circuit(Interpreter::AND_GATE).unwrap();

        let mut pins = [false, false, false];
        interpreter.execute(&and_gate, &mut pins);
        assert_eq!(pins.iter().eq([false, false, false].iter()), true);

        pins = [true, false, false];
        interpreter.execute(&and_gate, &mut pins);
        assert_eq!(pins.iter().eq([true, false, false].iter()), true);

        pins = [false, true, false];
        interpreter.execute(&and_gate, &mut pins);
        assert_eq!(pins.iter().eq([false, true, false].iter()), true);

        pins = [true, true, true];
        interpreter.execute(&and_gate, &mut pins);
        assert_eq!(pins.iter().eq([true, true, true].iter()), true);
    }

    #[test]
    fn default_not_gate() {
        let interpreter = Interpreter::default();
        let not_gate = interpreter.get_circuit(Interpreter::NOT_GATE).unwrap();

        let mut pins = [false, false];
        interpreter.execute(&not_gate, &mut pins);
        assert_eq!(pins.iter().eq([false, true].iter()), true);

        pins = [true, false];
        interpreter.execute(&not_gate, &mut pins);
        assert_eq!(pins.iter().eq([true, false].iter()), true);
    }

    fn register_nand_gate(interpreter: &mut Interpreter) {
        let mut nand_gate = LogicCircuit::new(NAND_GATE, 2, 1);
        nand_gate.add(Interpreter::AND_GATE);
        nand_gate.add(Interpreter::NOT_GATE);

        nand_gate.add_connection(1, 0, 2, 0);
        nand_gate.add_connection(1, 1, 2, 1);
        nand_gate.add_connection(2, 2, 3, 0);
        nand_gate.add_connection(3, 1, 1, 2);

        interpreter.register_circuit(nand_gate);
    }

    fn register_or_gate(interpreter: &mut Interpreter) {
        register_nand_gate(interpreter);

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

        interpreter.register_circuit(or_gate);
    }

    #[test]
    fn test_nand_gate() {
        let mut interpreter = Interpreter::default();
        register_nand_gate(&mut interpreter);

        let mut pins = [false, false, false];
        interpreter.execute_by_tid(NAND_GATE, &mut pins);
        assert_eq!(pins.iter().eq([false, false, true].iter()), true);

        let mut pins = [false, true, false];
        interpreter.execute_by_tid(NAND_GATE, &mut pins);
        assert_eq!(pins.iter().eq([false, true, true].iter()), true);

        let mut pins = [true, false, false];
        interpreter.execute_by_tid(NAND_GATE, &mut pins);
        assert_eq!(pins.iter().eq([true, false, true].iter()), true);

        let mut pins = [true, true, false];
        interpreter.execute_by_tid(NAND_GATE, &mut pins);
        assert_eq!(pins.iter().eq([true, true, false].iter()), true);
    }

    #[test]
    fn test_or_gate() {
        let mut interpreter = Interpreter::default();
        register_or_gate(&mut interpreter);

        let mut pins = [false, false, false];
        interpreter.execute_by_tid(OR_GATE, &mut pins);
        assert_eq!(pins.iter().eq([false, false, false].iter()), true);

        let mut pins = [false, true, false];
        interpreter.execute_by_tid(OR_GATE, &mut pins);
        assert_eq!(pins.iter().eq([false, true, true].iter()), true);

        let mut pins = [true, false, false];
        interpreter.execute_by_tid(OR_GATE, &mut pins);
        assert_eq!(pins.iter().eq([true, false, true].iter()), true);

        let mut pins = [true, true, false];
        interpreter.execute_by_tid(OR_GATE, &mut pins);
        assert_eq!(pins.iter().eq([true, true, true].iter()), true);
    }
}
