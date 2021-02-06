use log::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

pub type TID = u32; // Type Identifier
pub type UID = u32; // Unique Identifier

fn encode(bits: &[u8]) -> usize {
    let mut result: usize = 0;
    for bit in bits {
        result = (result << 1) + (*bit as usize);
    }

    result
}

fn decode(mut value: usize, bits: &mut [u8]) {
    let mut bit_idx: usize = bits.len() - 1;
    loop {
        bits[bit_idx] = (value & 1) as u8;

        if bit_idx == 0 {
            break;
        }

        bit_idx -= 1;
        value >>= 1;

        if value == 0 {
            break;
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Default)]
pub struct CircuitSimulator {
    circuit_library: HashMap<TID, LogicCircuit>,
}

pub const AND_GATE: TID = 1;
pub const NOT_GATE: TID = 2;

#[wasm_bindgen]
impl CircuitSimulator {
    pub fn default() -> CircuitSimulator {
        let mut circuit_library: HashMap<TID, LogicCircuit> = HashMap::new();

        circuit_library.insert(
            AND_GATE,
            LogicCircuit::from_truth_table(AND_GATE, 2, 1, vec![0, 0, 0, 1]),
        );

        circuit_library.insert(
            NOT_GATE,
            LogicCircuit::from_truth_table(NOT_GATE, 1, 1, vec![1, 0]),
        );

        CircuitSimulator { circuit_library }
    }

    pub fn register_circuit(&mut self, circuit: LogicCircuit) {
        self.circuit_library.insert(circuit.tid, circuit);
    }

    pub fn simulate_by_tid(&self, tid: TID, pins: &mut [u8], tick_count: u32) -> Context {
        self.simulate(self.get_unwrapped_circuit(tid), pins, tick_count)
    }

    pub fn simulate_by_tid_with_context(
        &self,
        context: &mut Context,
        tid: TID,
        pins: &mut [u8],
        tick_count: u32,
    ) {
        self.simulate_with_context(context, self.get_unwrapped_circuit(tid), pins, tick_count)
    }

    pub fn simulate(&self, circuit: &LogicCircuit, pins: &mut [u8], tick_count: u32) -> Context {
        assert_eq!(circuit.pin_count as usize, pins.len());
        let mut context = self.create_execution_context(&circuit);
        self.simulate_with_context(&mut context, circuit, pins, tick_count);
        context
    }

    pub fn simulate_with_context(
        &self,
        context: &mut Context,
        circuit: &LogicCircuit,
        pins: &mut [u8],
        tick_count: u32,
    ) {
        // TODO: A lot of duplicated code, clean up required
        assert_eq!(circuit.pin_count as usize, pins.len());
        debug!("({:p}) before {:?}", context, pins);

        // copy io to internal state
        for i in 0..circuit.pin_count {
            context.pins[i as usize] = pins[i as usize];
        }

        // connection flow
        for connection in circuit.connections.iter() {
            let input_offset =
                (context.offset_of(connection.input_uid) + connection.input_offset) as usize;
            let output_offset =
                (context.offset_of(connection.output_uid) + connection.output_offset) as usize;

            context.pins[output_offset] = context.pins[input_offset];
        }

        for tick in 0..tick_count {
            debug!("({:p}) tick {}:\n {:#?}", context, tick + 1, context);

            // is native circuit
            if circuit.children.is_empty() {
                if let Some(truth_table) = &circuit.truth_table {
                    let offset = context.offset_of(1) as usize;
                    let out_offset = offset + circuit.out_offset as usize;
                    let end_offset = offset + circuit.pin_count as usize;

                    decode(
                        truth_table[encode(&context.pins[offset..out_offset])],
                        &mut context.pins[out_offset..end_offset],
                    );

                    debug!("({:p}) evaluate using truth table", context);
                }

                continue;
            }

            debug!(
                "({:p}) evaluate using simulation using {} child",
                context,
                circuit.children.len()
            );
            for (child_uid, child_tid) in circuit.children.iter() {
                let child_circuit = self.get_unwrapped_circuit(*child_tid);
                let offset = context.offset_of(*child_uid) as usize;
                let out_offset = offset + child_circuit.out_offset as usize;
                let end_offset = offset + child_circuit.pin_count as usize;

                match &child_circuit.truth_table {
                    Some(truth_table) => {
                        decode(
                            truth_table[encode(&context.pins[offset..out_offset])],
                            &mut context.pins[out_offset..end_offset],
                        );
                    }
                    None => {
                        let child_context = context
                            .child_context
                            .entry(*child_uid)
                            .or_insert_with(|| self.create_execution_context(child_circuit));

                        self.simulate_with_context(
                            child_context,
                            child_circuit,
                            &mut context.pins[offset..end_offset],
                            1,
                        );
                    }
                }
            }

            // connection flow
            for connection in circuit.connections.iter() {
                let input_offset =
                    (context.offset_of(connection.input_uid) + connection.input_offset) as usize;
                let output_offset =
                    (context.offset_of(connection.output_uid) + connection.output_offset) as usize;

                context.pins[output_offset] = context.pins[input_offset];
            }
        }

        // Copy io from internal state
        for i in 0..circuit.pin_count {
            pins[i as usize] = context.pins[i as usize];
        }

        debug!("({:p}) after {:?}", context, pins);
    }

    fn create_execution_context(&self, circuit: &LogicCircuit) -> Context {
        let mut total_pin_count: u32 = circuit.pin_count;
        let mut offset_map: HashMap<UID, u32> = HashMap::new();

        // FIXME: Implied uid=1
        offset_map.insert(1, 0);

        for (uid, tid) in circuit.children.iter() {
            let child_circuit = self.circuit_library.get(tid).unwrap();
            offset_map.insert(*uid, total_pin_count);
            total_pin_count += child_circuit.pin_count;
        }

        Context::new(total_pin_count, offset_map)
    }

    fn get_unwrapped_circuit(&self, tid: TID) -> &LogicCircuit {
        match self.circuit_library.get(&tid) {
            Some(circuit) => circuit,
            None => panic!("Trying to get unregistered circuit type id: {}", tid),
        }
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Context {
    pins: Vec<u8>,
    offset_map: HashMap<UID, u32>,
    child_context: HashMap<UID, Context>,
}

impl Context {
    fn new(pin_count: u32, offset_map: HashMap<u32, u32>) -> Context {
        Context {
            pins: vec![0; pin_count as usize],
            offset_map,
            child_context: HashMap::new(),
        }
    }

    fn offset_of(&self, circuit_uid: u32) -> u32 {
        *self.offset_map.get(&circuit_uid).unwrap()
    }
}

#[wasm_bindgen]
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

#[wasm_bindgen]
#[derive(Debug)]
pub struct LogicCircuit {
    tid: TID,
    next_uid: UID,
    pin_count: u32,
    out_offset: u32,
    truth_table: Option<Vec<usize>>,
    children: HashMap<UID, TID>,
    connections: Vec<Connection>,
}

#[wasm_bindgen]
impl LogicCircuit {
    pub fn new(type_id: TID, input_pin_count: u32, output_pin_count: u32) -> LogicCircuit {
        LogicCircuit {
            tid: type_id,
            next_uid: 1,
            pin_count: input_pin_count + output_pin_count,
            out_offset: input_pin_count,
            truth_table: Option::None,
            children: HashMap::new(),
            connections: Vec::new(),
        }
    }

    pub fn from_truth_table(
        type_id: TID,
        input_pin_count: u32,
        output_pin_count: u32,
        truth_table: Vec<usize>,
    ) -> LogicCircuit {
        LogicCircuit {
            tid: type_id,
            next_uid: 1,
            pin_count: input_pin_count + output_pin_count,
            out_offset: input_pin_count,
            truth_table: Option::Some(truth_table),
            children: HashMap::new(),
            connections: Vec::new(),
        }
    }

    pub fn add(&mut self, circuit_type_id: TID) -> UID {
        self.next_uid += 1;
        self.children.insert(self.next_uid, circuit_type_id);
        self.next_uid
    }

    pub fn add_connection(
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

#[cfg(test)]
pub mod tests;
