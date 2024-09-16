use std::collections::HashMap;
pub mod genome;
pub use genome::*;

#[derive(Debug, Clone)]
pub struct Neuron {
    pub id: [u8; 2],
    pub flag: [u8; 2],
    pub bias: f32,
    pub activation: f32,
}

#[derive(Debug, Clone)]
pub struct Synapse {
    pub id: [u8; 2],
    pub flag: [u8; 2],
    pub destination: [u8; 2], // Converted to and from f32 :(
    pub weight: f32,
}

#[derive(Debug, Clone)]
pub struct Gene {
    pub id: [u8; 2],
    pub flag: [u8; 2],
    pub local_data: f32,
    pub extern_data: f32,
}

// Main network structure with genome, neurons, and synapses
pub struct Network {
    pub genome: Genome,
    pub neurons: HashMap<[u8;2], Neuron>,
    pub synapses: HashMap<([u8;2],[u8;2]), Synapse>,
}

// Genome structure with Vec of Genes for easy search and manipulation
#[derive(Debug)]
pub struct Genome {
    pub genes: Vec<Gene>,
}

impl Genome {
    // Create a new Genome with inputs and outputs
    pub fn new(inputs: u16, outputs: u16) -> Self {
        let mut genes = Vec::new();
        for i in 0..inputs {
            genes.push(Gene {
                id: [i as u8, 0],
                flag: [1, 0],
                local_data: 0.0,
                extern_data: 0.0,
            });
        }
        for i in 0..outputs {
            genes.push(Gene {
                id: [255 - i as u8, 0],
                flag: [3, 0],
                local_data: 0.0,
                extern_data: 0.0,
            });
        }

        Genome { genes }

    }    
}



impl Network {
    pub fn new(
        inputs: u16,
        outputs: u16,
    ) -> Self {
        Network {
            genome: Genome::new(inputs, outputs),
            neurons: HashMap::new(),
            synapses: HashMap::new(),
        }
    }
    // Parse genome to create neurons and synapses
    pub fn parse_genome(
        &mut self,
    ) {
        for gene in &self.genome.genes {
            if gene.flag[0] <= 3 {

                let new_neuron = Neuron {
                    id: gene.id,
                    flag: gene.flag.clone(),
                    bias: gene.local_data,
                    activation: gene.extern_data,
                };
                self.neurons.insert(gene.id, new_neuron);
            }
            if gene.flag[0] == 10 {
                let new_synapse = Synapse {
                    id: gene.id,
                    flag: gene.flag.clone(),
                    destination: convert_f32_to_id(gene.local_data),
                    weight: gene.extern_data,
                };
                self.synapses.insert((gene.id, new_synapse.destination), new_synapse);
            }
        }
    }
    pub fn display(&self) {
        println!("################ NETWORK DISPLAY ################");

        // Collect and sort neurons by their IDs
        let mut neurons: Vec<&Neuron> = self.neurons.values().collect();
        neurons.sort_by_key(|n| n.id);

        // Display neurons
        for neuron in neurons {
            let id = neuron.id;
            let flag1 = neuron.flag[0];
            let bias = neuron.bias;
            let activation = neuron.activation;

            // Determine neuron type based on flag
            match flag1 {
                1 => println!(
                    "INPUT NEURON - - - - - # ID: {:?} # BIAS: {} # ACTIVATION: {}",
                    id, bias, activation
                ),
                2 => println!(
                    "HIDDEN NEURON - - - - - # ID: {:?} # BIAS: {} # ACTIVATION: {}",
                    id, bias, activation
                ),
                3 => println!(
                    "OUTPUT NEURON - - - - - # ID: {:?} # BIAS: {} # ACTIVATION: {}",
                    id, bias, activation
                ),
                _ => println!(
                    "UNKNOWN NEURON - - - - # ID: {:?} # BIAS: {} # ACTIVATION: {}",
                    id, bias, activation
                ),
            }
        }

        // Collect and sort synapses by their source and destination IDs
        let mut synapses: Vec<&Synapse> = self.synapses.values().collect();
        synapses.sort_by_key(|s| (s.id, s.destination));

        // Display synapses
        for synapse in synapses {
            let id_from = synapse.id;
            let id_to = synapse.destination;
            let weight = synapse.weight;
            let enabled = if synapse.flag[1] == 11 { "False" } else { "True" };

            println!(
                "SYNAPSE - - - - - - - - # FROM: {:?} # TO: {:?} # WEIGHT: {} # ENABLED: [{}]",
                id_from, id_to, weight, enabled
            );
        }

        println!("################################################\n");
    }
    pub fn debug_display(&self) {
        println!("################ NETWORK DEBUG DISPLAY ################");
        println!("Neurons:");
        for neuron in self.neurons.values() {
            println!("{:?}", neuron);
        }
        println!("Synapses:");
        for synapse in self.synapses.values() {
            println!("{:?}", synapse);
        }
        println!("########################################################\n");
    }
}

pub fn convert_id_to_f32 (
    id: [u8; 2],
) -> f32 {
    let combined_value: u16 = u16::from_le_bytes(id); // Combine into u16
    let float_value: f32 = combined_value as f32; // Convert to f32
    float_value
}

pub fn convert_f32_to_id (
    float: f32,
) -> [u8;2] {
    let combined_value_back: u16 = float as u16; // Convert back to u16
    let byte_array_back: [u8; 2] = combined_value_back.to_le_bytes(); // Split back to [u8; 2]
    byte_array_back
}