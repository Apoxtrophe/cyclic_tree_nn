use crate::{convert_f32_to_id, convert_id_to_f32, Gene, Genome, Neuron};
use rand::prelude::*;

// MUTATION FUNCTIONS
impl Genome {
    // Finds parents that may have children and gives their ids
    // Beside that result it also gives the appropriate id of each possible child

    pub fn create_rand_neuron_child(&mut self) {
        let parent_child_pairs = self.get_neuron_candidates();
        if let Some(selected_pair) = self.create_rand_neuron(&parent_child_pairs) {
            self.create_neuron(selected_pair);
            self.create_synapse(selected_pair);
        }
        self.genes.sort_by_key(|k| k.id)
    }
    // Select a random input or hidden neuron to add a synapse connection to
    pub fn create_rand_synapse(&mut self) {
        let mut rng = thread_rng();
        let mut possible_neuron_source_ids = Vec::new();
        for gene in &mut self.genes {
            if gene.flag[0] == 1 || gene.flag[0] == 2 {
                possible_neuron_source_ids.push(gene.id);
            }
        }
        println!("Possible Source Id's {:?}", possible_neuron_source_ids);
        if possible_neuron_source_ids.is_empty() {
            return;
        }
        let selected = rng.gen_range(0..possible_neuron_source_ids.len());
        let source_id = possible_neuron_source_ids[selected];
        println!("source_id: {:?}", source_id);
        
        let possible_neuron_target_ids = self.get_synapse_candidates(source_id);
        println!("Possible Target Id's {:?}", possible_neuron_target_ids);
        if possible_neuron_target_ids.is_empty() {
            return;
        }
        
        
        let target_selected = rng.gen_range(0..possible_neuron_target_ids.len());
        let target_id = possible_neuron_target_ids[target_selected];
        
        println!("FROM_ID: {:?} TO_ID: {:?}", source_id, target_id);
        self.create_synapse((source_id, target_id));
        self.genes.sort_by_key(|k| k.id)
    }
}

// HELPER FUNCTIONS

impl Genome {
    // Create neuron from parent-child pair
    fn create_neuron(&mut self, selected_pair: ([u8; 2], [u8; 2])) {
        for gene in &mut self.genes {
            if gene.id == selected_pair.0 && gene.flag[0] <= 3 {
                gene.flag[1] += 1;
            }
        }

        let mut parent_index = 0;
        for i in 0..self.genes.len() {
            if self.genes[i].id == selected_pair.0 {
                parent_index = i;
                break;
            }
        }

        self.genes.insert(
            parent_index + 1,
            Gene {
                id: selected_pair.1,
                flag: [2, 0],
                local_data: 0.0,
                extern_data: 0.0,
            },
        );
    }
    // Find a valid, random parent-child pair
    fn create_rand_neuron(
        &self,
        parent_child_pairs: &Vec<([u8; 2], [u8; 2])>,
    ) -> Option<([u8; 2], [u8; 2])> {
        if parent_child_pairs.is_empty() {
            return None;
        }
        let mut rng = thread_rng();
        let selected = rng.gen_range(0..parent_child_pairs.len());
        Some(parent_child_pairs[selected])
    }
    // Find all possible parent-child pairs
    pub fn get_neuron_candidates(&self) -> Vec<([u8; 2], [u8; 2])> {
        let mut available_ids: Vec<([u8; 2], [u8; 2])> = Vec::new();
        for gene in &self.genes {
            let mut pair: ([u8; 2], [u8; 2]) = ([0, 0], [0, 0]);
            if gene.flag[0] <= 2 && gene.flag[1] < 2 {
                pair.0 = gene.id;
                let child_id = match gene.flag[1] {
                    0 => [gene.id[0], 2 * gene.id[1] + 1],
                    1 => [gene.id[0], 2 * gene.id[1] + 2],
                    _ => continue,
                };
                pair.1 = child_id;
                available_ids.push(pair);
            }
        }
        available_ids
    }
    // Create a synapse between two neurons
    pub fn create_synapse(&mut self, selected_pair: ([u8; 2], [u8; 2])) {
        let from_id = selected_pair.0;
        let to_id = selected_pair.1;
        let mut neuron_index = 0;
        for i in 0..self.genes.len() {
            if self.genes[i].id == from_id {
                neuron_index = i;
            }
        }
        let new_synapse = Gene {
            id: [from_id[0], from_id[1]],
            flag: [10, 10],
            local_data: convert_id_to_f32(to_id),
            extern_data: 0.0,
        };
        self.genes.insert(neuron_index + 1 , new_synapse);
    }

    // For any given neuron id, find all possible synapse candidates
    pub fn get_synapse_candidates(
        &self,
        neuron_id: [u8; 2]
    ) -> Vec<[u8; 2]> {
        let neuron_height = get_neuron_height(neuron_id[1]);
        
        let mut available_ids: Vec<[u8; 2]> = Vec::new();
        for gene in &self.genes {
            if gene.flag[0] == 3 {
                available_ids.push(gene.id);
            }
            
            let height = get_neuron_height(gene.id[1]);
            if self.are_connected((neuron_id, gene.id)) {
                println!("ALREADY CONNECTED");
                continue;
            }
            if height <= neuron_height {
                println!("TOO HIGH");
                continue;
            }
            if gene.id == neuron_id {
                println!("SAME ID");
                continue;
            }
            if gene.flag[0] == 10 {
                println!("ALREADY SYNAPSE");
                continue;
            }

            available_ids.push(gene.id);
        }
        println!("AVAILABLE IDS: {:?}", available_ids);
        available_ids
    }
    // Are any two given nodes connected via a synapse???
    pub fn are_connected (
        &self,
        selected_pair: ([u8; 2], [u8; 2]),
    ) -> bool {
        for gene in &self.genes {
            if gene.id == selected_pair.0 {
                if gene.flag[0] == 10 {
                    return true;
                }
            }
        }
        return false;
    }
}

// Debug display for genome
impl Genome {
    pub fn display(&self) {
        println!("################ GENOME DISPLAY ################");
        for gene in &self.genes {
            let seed = gene.id[0];
            let id = gene.id[1];
            let flag1 = gene.flag[0];
            let flag2 = gene.flag[1];
            let lcl_data = gene.local_data;
            let ext_data = gene.extern_data;
            if flag1 == 1 {
                println!(
                    "INPUT NEURON - - - - - # ID: {:?} # CHILDREN: {} # BIAS: {}",
                    gene.id, flag2, lcl_data
                );
            }
            if flag1 == 2 {
                println!(
                    "L HIDDEN NEURON- - - - # ID: {:?} # CHILDREN: {} # BIAS: {}",
                    gene.id, flag2, lcl_data
                );
            }
            if flag1 == 3 {
                println!(
                    "OUTPUT NEURON- - - - - # ID: {:?} # CHILDREN: {} # BIAS: {}",
                    gene.id, flag2, lcl_data
                );
            }
            if flag1 == 10 {
                let mut enabled = "True";
                if flag2 == 11 {
                    enabled = "False";
                }
                let to_id = convert_f32_to_id(lcl_data);
                println!("  L SYNAPSE- - - - - - # ID_FROM {:?} # ID_TO {:?} # WEIGHT {} # ENABLED: [{}]", gene.id, to_id, ext_data, enabled)
            }
        }
        println!("################################################\n");
    }
    pub fn debug_display(&self) {
        println!("DEBUG DISPLAY");
        for gene in &self.genes {
            println!("{:?}", gene);
        }
    }
}

pub fn get_neuron_height(
    neuron_flag1 : u8
    ) -> u8 {
    let height_limits = [
        1,    // Height 0: 1 node
        3,    // Height 1: 3 nodes total
        7,    // Height 2: 7 nodes total
        15,   // Height 3: 15 nodes total
        31,   // Height 4: 31 nodes total
        63,   // Height 5: 63 nodes total
        127,  // Height 6: 127 nodes total
        255,  // Height 7: 255 nodes total
    ];
    
    for (height, &limit) in height_limits.iter().enumerate() {
        if neuron_flag1 < limit {
            return height as u8;
        }
    }
    height_limits.len() as u8 - 1
}