pub mod circuit {

    use std::collections::{HashMap, VecDeque};

    pub type TID = u32; // Type Identifier
    pub type UID = u32; // Unique Identifier

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

    #[derive(Debug)]
    pub struct Interpreter {
        circuit_library: HashMap<TID, LogicCircuit>,
    }

    impl Interpreter {
        pub const AND_GATE: TID = 1;
        pub const NOT_GATE: TID = 2;

        pub fn new() -> Interpreter {
            Interpreter {
                circuit_library: HashMap::new(),
            }
        }

        pub fn default() -> Interpreter {
            let mut circuit_library: HashMap<TID, LogicCircuit> = HashMap::new();

            circuit_library.insert(
                Interpreter::AND_GATE,
                LogicCircuit::from_truth_table(Interpreter::AND_GATE, 2, 1, vec![0, 0, 0, 1]),
            );

            circuit_library.insert(
                Interpreter::NOT_GATE,
                LogicCircuit::from_truth_table(Interpreter::NOT_GATE, 1, 1, vec![1, 0]),
            );

            Interpreter { circuit_library }
        }

        pub fn register_circuit(&mut self, circuit: LogicCircuit) {
            self.circuit_library.insert(circuit.tid, circuit);
        }

        pub fn get_circuit(&self, id: u32) -> Option<&LogicCircuit> {
            self.circuit_library.get(&id)
        }

        pub fn execute_by_tid(&self, circuit_tid: TID, pins: &mut [bool]) -> Context {
            self.execute(self.get_unwrapped_circuit(circuit_tid), pins)
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
                let connections_iter = match context.connection_map.get(&current_uid) {
                    Some(connections) => connections,
                    None => break,
                };
                let offset_map = &context.offset_map;
                let type_map = &context.type_map;
                let pins = &mut context.pins;
                let inp_offset = offset as u32;

                for conn in connections_iter {
                    let out_offset = offset_map.get(&conn.output_uid).unwrap();
                    pins[(out_offset + conn.output_offset) as usize] =
                        pins[(inp_offset + conn.input_offset) as usize];

                    // println!(
                    //     "Assign pin {} to pin {}",
                    //     inp_offset + conn.input_offset,
                    //     out_offset + conn.output_offset
                    // );

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
                    .or_insert_with(Vec::new)
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
        fn new(
            input_uid: UID,
            input_offset: u32,
            output_uid: UID,
            output_offset: u32,
        ) -> Connection {
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
        next_uid: u32,
        pin_count: u32,
        out_offset: u32,
        truth_table: Option<Vec<usize>>,
        children: HashMap<UID, TID>,
        connections: Vec<Connection>,
    }

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
            type_id: u32,
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

        pub fn tick(&self, context: &Context) {}
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
}
