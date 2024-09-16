use core::f64;
use std::collections::HashMap;
pub mod genome;
pub use genome::*;
use plotly::common::Mode;
use plotly::{Plot, Scatter};


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
    pub fn visualize (
        &self,
    ) {
        let mut plot = Plot::new();
        
        let mut neuron_positions: HashMap<[u8;2], (f64, f64)> = HashMap::new();
        let horizontal_spacing = 1.0;
        let vertical_spacing = 1.0;
        for neuron in self.neurons.values() {
                    let neuron_type = neuron.flag[0];
                    let mut x_pos = 0.0;
                    let mut y_pos = 0.0;
        
                    // Convert id[0] and id[1] to u16 for calculation
                    let tree_id = neuron.id[0] as u16;
                    let node_id1 = neuron.id[1] as u16;
        
                    if neuron_type == 1 { // Input neuron (root of a tree)
                        x_pos = tree_id as f64 * 1.0; // Separate trees horizontally
                        y_pos = 0.0;
                    } else if neuron_type == 2 { // Hidden neuron
                        let depth = get_neuron_height(node_id1 as u8) as f64;
                        y_pos = depth * vertical_spacing;
        
                        let inorder_pos = get_inorder_position(node_id1) as f64;
                        let max_nodes_at_depth = 2u32.pow(depth as u32) as f64;
        
                        // Adjust x_pos within the tree
                        x_pos = (inorder_pos + 0.5) * (horizontal_spacing / max_nodes_at_depth);
                        // Shift x_pos based on the tree's position
                        x_pos += tree_id as f64 * 1.0;
                    } else if neuron_type == 3 { // Output neuron
                        x_pos = 255.0 - tree_id as f64;
                        y_pos = (7.0 + 1.0) * vertical_spacing;
                    }
        
                    // Store positions
                    neuron_positions.insert(neuron.id, (x_pos, y_pos));
        
                    // Debug print
                    println!("Neuron ID: {:?}, x: {}, y: {}", neuron.id, x_pos, y_pos);
                }
        
        let mut edge_x = Vec::new();
        let mut edge_y = Vec::new();
        
        for synapse in self.synapses.values() {
            let source_id = synapse.id;
            let destination_id = synapse.destination;
            
            if let (Some(&(x0, y0)), Some(&(x1, y1))) = 
                (neuron_positions.get(&source_id), neuron_positions.get(&destination_id)) {
                
                edge_x.push(x0);
                edge_x.push(x1);
                edge_x.push(f64::NAN);    
                edge_y.push(y0);
                edge_y.push(y1);
                edge_y.push(f64::NAN);
            }
        }
        let edge_trace = Scatter::new(edge_x, edge_y)
            .mode(Mode::Lines)
            .name("")
            .line(plotly::common::Line::new().color("#888").width(1.0))
            .hover_info(plotly::common::HoverInfo::None);
        
        plot.add_trace(edge_trace);
        
        
        let mut node_x = Vec::new();
        let mut node_y = Vec::new();
        let mut node_text = Vec::new();
        
        for neuron in self.neurons.values() {
            let (x , y) = neuron_positions[&neuron.id];
            node_x.push(x);
            node_y.push(y);
            node_text.push(format!("ID: {:?}", neuron.id));
        }
        
        let node_trace = Scatter::new(node_x, node_y)
                    .mode(Mode::MarkersText)
                    .name("")
                    .marker(plotly::common::Marker::new().size(10).color("skyblue"))
                    .hover_text_array(node_text);
                    
        plot.add_trace(node_trace);
        
        plot.show();
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

fn get_inorder_position(id1: u16) -> u16 {
    // Base case: root node
    if id1 == 0 {
        return 0;
    }

    // Recursive case
    let parent_id1 = (id1 - 1) / 2;
    let is_left_child = id1 % 2 == 1;

    let parent_position = get_inorder_position(parent_id1);

    if is_left_child {
        // Left child: position is parent's position * 2
        return parent_position * 2;
    } else {
        // Right child: position is parent's position * 2 + 1
        return parent_position * 2 + 1;
    }
}