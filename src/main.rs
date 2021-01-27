use std::collections::{HashMap, VecDeque};

type TID = u32; // Type Identifier
type UID = u32; // Unique Identifier

#[derive(Debug)]
pub struct Interpreter {
    circuit_library: HashMap<TID, LogicCircuit>,
}

impl Interpreter {
    const AND_GATE: TID = 1;
    const NOT_GATE: TID = 2;

    pub fn new() -> Interpreter {
        Interpreter {
            circuit_library: HashMap::new(),
        }
    }

    pub fn default() -> Interpreter {
        let mut circuit_library: HashMap<TID, LogicCircuit> = HashMap::new();

        circuit_library.insert(
            Interpreter::AND_GATE,
            LogicCircuit {
                tid: Interpreter::AND_GATE,
                pin_count: 3,
                out_offset: 2,
                truth_table: Option::Some(vec![0, 0, 0, 1]),
                children: HashMap::new(),
                connections: Vec::new(),
            },
        );

        circuit_library.insert(
            Interpreter::NOT_GATE,
            LogicCircuit {
                tid: Interpreter::NOT_GATE,
                pin_count: 2,
                out_offset: 1,
                truth_table: Option::Some(vec![1, 0]),
                children: HashMap::new(),
                connections: Vec::new(),
            },
        );

        Interpreter { circuit_library }
    }

    pub fn register_circuit(&mut self, circuit: LogicCircuit) {
        self.circuit_library.insert(circuit.tid, circuit);
    }

    pub fn get_circuit(&self, id: u32) -> Option<&LogicCircuit> {
        self.circuit_library.get(&id)
    }

    pub fn execute(&self, circuit: &LogicCircuit, pins: &mut [bool]) -> Context {
        assert_eq!(circuit.pin_count as usize, pins.len());

        let mut context = self.create_execution_context(&circuit);
        let mut circuit_desc_queue: VecDeque<(TID, UID)> = VecDeque::new();
        let mut current_circuit = circuit;
        let mut current_uid = 1; // FIXME: Implied uid=1
        for i in 0..circuit.pin_count {
            context.pins[i as usize] = pins[i as usize];
        }

        loop {
            // println!(
            //     "context={:p}, tid={}, uid={}, pins={:?}",
            //     &context, current_circuit.tid, current_uid, context.pins
            // );
            let offset = context.offset_of(current_uid) as usize;
            let out_offset = offset + current_circuit.out_offset as usize;
            let end_offset = offset + current_circuit.pin_count as usize;

            match &current_circuit.truth_table {
                Some(truth_table) => {
                    let encoded_value = encode(&context.pins[offset..out_offset]);
                    let mapped_value = truth_table[encoded_value];

                    decode(mapped_value, &mut context.pins[out_offset..end_offset]);
                    // println!(
                    //     "encoded={}, mapped={}, range={}..{}..{}",
                    //     encoded_value, mapped_value, offset, out_offset, end_offset
                    // );
                }
                None => {
                    if current_uid != 1 {
                        self.execute(current_circuit, &mut context.pins[offset..end_offset]);
                    }
                }
            }

            // FIXME: Make it so that we do not have to per field borrow
            let connections_iter = context.connection_map.get(&current_uid).unwrap().iter();
            let offset_map = &context.offset_map;
            let type_map = &context.type_map;
            let pins = &mut context.pins;
            let inp_offset = offset as u32;

            for conn in connections_iter {
                let out_offset = offset_map.get(&conn.output_uid).unwrap();
                pins[(out_offset + conn.output_offset) as usize] =
                    pins[(inp_offset + conn.input_offset) as usize];

                println!(
                    "Assign pin {} to pin {}",
                    inp_offset + conn.input_offset,
                    out_offset + conn.output_offset
                );

                circuit_desc_queue
                    .push_front((*type_map.get(&conn.output_uid).unwrap(), conn.output_uid));
            }

            let (maybe_circuit, maybe_uid) = match circuit_desc_queue.pop_back() {
                Some((tid, uid)) => {
                    if uid == 1 {
                        break;
                    }
                    (self.get_unwrapped_circuit(tid), uid)
                }
                None => break,
            };

            current_circuit = maybe_circuit;
            current_uid = maybe_uid;
        }

        for i in 0..circuit.pin_count {
            pins[i as usize] = context.pins[i as usize];
        }

        // println!(
        //     "context={:p}, tid={}, result={:?}",
        //     &context, current_circuit.tid, pins
        // );

        context
    }

    fn create_execution_context(&self, circuit: &LogicCircuit) -> Context {
        let mut total_pin_count: u32 = circuit.pin_count;
        let mut offset_map: HashMap<UID, u32> = HashMap::new();
        let mut type_map: HashMap<UID, TID> = HashMap::new();

        // FIXME: Implied uid=1
        type_map.insert(1, circuit.tid);
        offset_map.insert(1, 0);

        for (uid, tid) in circuit.children.iter() {
            let cc = self.circuit_library.get(tid).unwrap();
            type_map.insert(*uid, cc.tid);
            offset_map.insert(*uid, total_pin_count);
            total_pin_count += cc.pin_count;
        }

        let mut connection_map: HashMap<u32, Vec<Connection>> = HashMap::new();
        for conn in circuit.connections.iter() {
            connection_map
                .entry(conn.input_uid)
                .or_insert(Vec::new())
                .push(conn.clone());
        }

        Context::new(total_pin_count, connection_map, offset_map, type_map)
    }

    fn get_unwrapped_circuit(&self, id: u32) -> &LogicCircuit {
        match self.get_circuit(id) {
            Some(circuit) => circuit,
            None => panic!("Trying to get unregistered circuit type id: {}", id),
        }
    }
}

#[derive(Debug)]
pub struct Context {
    pins: Vec<bool>,
    connection_map: HashMap<UID, Vec<Connection>>,
    offset_map: HashMap<UID, u32>,
    type_map: HashMap<UID, TID>,
}

impl Context {
    fn new(
        pin_count: u32,
        connection_map: HashMap<u32, Vec<Connection>>,
        offset_map: HashMap<u32, u32>,
        type_map: HashMap<UID, TID>,
    ) -> Context {
        Context {
            pins: vec![false; pin_count as usize],
            connection_map,
            offset_map,
            type_map,
        }
    }

    fn offset_of(&self, circuit_uid: u32) -> u32 {
        *self.offset_map.get(&circuit_uid).unwrap()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Connection {
    input_uid: UID,
    input_offset: u32,
    output_uid: UID,
    output_offset: u32,
}

impl Connection {
    fn new(input_uid: UID, input_offset: u32, output_uid: UID, output_offset: u32) -> Connection {
        Connection {
            input_uid,
            input_offset,
            output_uid,
            output_offset,
        }
    }
}

#[derive(Debug)]
pub struct LogicCircuit {
    tid: u32,
    pin_count: u32,
    out_offset: u32,
    truth_table: Option<Vec<usize>>,
    children: HashMap<UID, TID>,
    connections: Vec<Connection>,
}

fn encode(bits: &[bool]) -> usize {
    let mut result: usize = 0;
    for bit in bits {
        result = (result << 1) + if *bit { 1 } else { 0 }
    }

    result
}

fn decode(mut value: usize, bits: &mut [bool]) {
    let mut bit_idx: usize = bits.len() - 1;
    while value > 0 {
        bits[bit_idx] = value & 1 == 1;
        if bit_idx == 0 {
            break;
        }

        bit_idx -= 1;
        value >>= 1;
    }
}

impl LogicCircuit {
    fn new(type_id: TID, input_pin_count: u32, output_pin_count: u32) -> LogicCircuit {
        LogicCircuit {
            tid: type_id,
            pin_count: input_pin_count + output_pin_count,
            out_offset: input_pin_count,
            truth_table: Option::None,
            children: HashMap::new(),
            connections: Vec::new(),
        }
    }

    fn add(&mut self, circuit_uid: UID, circuit_type_id: TID) {
        self.children.insert(circuit_uid, circuit_type_id);
    }

    fn add_connection(
        &mut self,
        input_uid: UID,
        input_offset: u32,
        output_uid: UID,
        output_offset: u32,
    ) {
        self.connections.push(Connection::new(
            input_uid,
            input_offset,
            output_uid,
            output_offset,
        ));
    }
}

impl std::cmp::PartialEq for LogicCircuit {
    fn eq(&self, other: &Self) -> bool {
        self.tid == other.tid
    }
}

impl std::cmp::Eq for LogicCircuit {}

impl std::hash::Hash for LogicCircuit {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tid.hash(state);
    }
}

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
