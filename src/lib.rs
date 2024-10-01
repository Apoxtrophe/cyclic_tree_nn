use core::f64;
use std::collections::HashMap;
pub mod genome;
pub use genome::*;

pub mod visuals;
pub use visuals::*;

use plotly::common::Mode;
use plotly::{Plot, Scatter};


// Enums and constants for better readability
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GeneType {
    Input = 1,
    Hidden = 2,
    Output = 3,
    Synapse = 10,
}



#[derive(Debug, PartialEq, Clone, Copy)]
enum SynapseStatus {
    Enabled = 10,
    Disabled = 11,
}

impl SynapseStatus {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            10 => Some(SynapseStatus::Enabled),
            11 => Some(SynapseStatus::Disabled),
            _ => None,
        }
    }
}

// Helper functions to convert between GeneType and u8
impl GeneType {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(GeneType::Input),
            2 => Some(GeneType::Hidden),
            3 => Some(GeneType::Output),
            10 => Some(GeneType::Synapse),
            _ => None,
        }
    }

    fn as_u8(&self) -> u8 {
        *self as u8
    }
}

// The Gene struct represents neurons and synapses in the genome
#[derive(Debug, Clone)]
pub struct Gene {
    pub id: [u8; 2],    // id[0]: seed (input/output neuron number), id[1]: position in the tree
    pub flag: [u8; 2],  // flag[0]: GeneType, flag[1]: additional info (e.g., child count)
    pub local_data: f32,
    pub extern_data: f32,
}

// Genome is a blueprint for the network, later parsed into neurons and synapses
#[derive(Debug)]
pub struct Genome {
    pub genes: Vec<Gene>,
}

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
    pub destination: [u8; 2],
    pub weight: f32,
}

// Main network structure with genome, neurons, and synapses
pub struct Network {
    pub genome: Genome,
    pub neurons: HashMap<[u8;2], Neuron>,
    pub synapses: HashMap<([u8;2],[u8;2]), Synapse>,
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

