use crate::{Gene, GeneType, Genome, SynapseStatus};
use rand::prelude::*;

// MUTATION FUNCTIONS
impl Genome {
    /// Creates a new Genome with specified numbers of input and output neurons
    pub fn new(inputs: u16, outputs: u16) -> Self {
        assert!(inputs <= 256, "Number of inputs must be <= 256");
        assert!(outputs <= 256, "Number of outputs must be <= 256");

        let mut genes = Vec::with_capacity((inputs + outputs) as usize);

        // Assign unique IDs to input neurons
        for i in 0..inputs {
            genes.push(Gene {
                id: [i as u8, 0],
                flag: [GeneType::Input.as_u8(), 0],
                local_data: 0.0,
                extern_data: 0.0,
            });
        }

        // Assign unique IDs to output neurons
        for i in 0..outputs {
            genes.push(Gene {
                id: [255 - i as u8, 0],
                flag: [GeneType::Output.as_u8(), 0],
                local_data: 0.0,
                extern_data: 0.0,
            });
        }

        Genome { genes }
    }
    
    /// Creates a random neuron child by selecting a parent that can have children
    pub fn random_child(&mut self) {
        let parent_child_pairs = self.get_neuron_candidates();
        if let Some(selected_pair) = self.select_random_pair(&parent_child_pairs) {
            self.create_neuron(selected_pair);
            self.create_synapse(selected_pair);
            self.sort_genes();
        }
    }
    
    pub fn rand_connected_child (
        &mut self,
    ) {
        let parent_child_pairs = self.get_neuron_candidates();
        if let Some(selected_pair) = self.select_random_pair(&parent_child_pairs) {
            self.create_neuron(selected_pair);
            self.create_synapse(selected_pair);
            let possible_targets = self.get_synapse_candidates(selected_pair.1);
            if let Some(target_id) = self.select_random(&possible_targets) {
                self.create_synapse((selected_pair.1, target_id));
            }
            self.sort_genes();

        }
    }
    
    /// Creates a random synapse between neurons
    pub fn random_synapse(&mut self) {
        if let Some(source_id) = self.select_random(&self.get_possible_synapse_sources()) {
            let possible_targets = self.get_synapse_candidates(source_id);
            if let Some(target_id) = self.select_random(&possible_targets) {
                self.create_synapse((source_id, target_id));
                self.sort_genes();
            }
        }
    }
    
    /// Disables a random enabled synapse
    pub fn disable_random_synapse(&mut self) {
        if let Some(index) = self.select_random_index(&self.get_synapse_indices(SynapseStatus::Enabled)) {
            self.genes[index].flag[1] = SynapseStatus::Disabled as u8;
            println!("Disabled synapse at index {}", index);
        } else {
            println!("No enabled synapses to disable.");
        }
    }
    /// Enables a random disabled synapse
    pub fn enable_random_synapse(&mut self) {
        if let Some(index) = self.select_random_index(&self.get_synapse_indices(SynapseStatus::Disabled)) {
            self.genes[index].flag[1] = SynapseStatus::Enabled as u8;
            println!("Enabled synapse at index {}", index);
        } else {
            println!("No disabled synapses to enable.");
        }
    }
    
    /// Removes a random synapse that is disabled. If the destination or source neuourns have 1 or 0 synapses after this occurs, they will be removed as well. 
    pub fn remove_random_disabled_synapse(
        &mut self,
    ) {
        let mut source_target_ids: Vec<([u8;2], [u8;2])> = Vec::new();
        for genes in self.genes.iter_mut() {
            if genes.flag[1] == 11 { // If synapse is disabled
                let source_id = genes.id;
                let target_id = convert_f32_to_id(genes.local_data);
                source_target_ids.push((source_id, target_id));
            }
        }
        if let Some(selected_pair) = self.select_random_pair(&source_target_ids) {
            self.remove_synapse(selected_pair.0, selected_pair.1);
        }
    }
    
    /// Helper function to sort genes by their IDs
    fn sort_genes(&mut self) {
        self.genes.sort_by_key(|k| k.id);
    }
}

// Helper functions for Genome
impl Genome {
    /// Removes a synapse between two neurons.
    /// If the source or destination neurons become isolated after removal, they are also removed.
    pub fn remove_synapse(&mut self, from_id: [u8; 2], to_id: [u8; 2]) -> bool {
        if let Some(index) = self.find_synapse_index(from_id, to_id) {
            self.genes.remove(index);
            self.update_parent_child_counts(from_id, to_id);
            self.remove_neuron_if_isolated(from_id);
            self.remove_neuron_if_isolated(to_id);
            println!("Removed synapse from {:?} to {:?}", from_id, to_id);
            true
        } else {
            println!("Synapse from {:?} to {:?} not found.", from_id, to_id);
            false
        }
    }

    /// Updates the child count of the parent neuron after synapse removal.
    fn update_parent_child_counts(&mut self, from_id: [u8; 2], _to_id: [u8; 2]) {
        if let Some((_, parent_gene)) = self.find_gene_by_id_and_type_mut(
            from_id,
            &[GeneType::Input, GeneType::Hidden],
        ) {
            if parent_gene.flag[1] > 0 {
                parent_gene.flag[1] -= 1;
            }
        }
    }

    /// Removes a neuron if it has no incoming or outgoing synapses and is not an Input or Output neuron.
    fn remove_neuron_if_isolated(&mut self, neuron_id: [u8; 2]) {
        if let Some(gene_type) = self.get_gene_type(neuron_id) {
            if gene_type == GeneType::Input || gene_type == GeneType::Output {
                return;
            }
        }

        let has_incoming = self.genes.iter().any(|gene| {
            GeneType::from_u8(gene.flag[0]) == Some(GeneType::Synapse)
                && convert_f32_to_id(gene.local_data) == neuron_id
        });

        let has_outgoing = self.genes.iter().any(|gene| {
            gene.id == neuron_id && GeneType::from_u8(gene.flag[0]) == Some(GeneType::Synapse)
        });

        if !has_incoming && !has_outgoing {
            self.genes.retain(|gene| gene.id != neuron_id);
            println!("Removed isolated neuron {:?}", neuron_id);
        }
    }

    /// Helper function to find the index of a synapse in the genes vector.
    fn find_synapse_index(&self, from_id: [u8; 2], to_id: [u8; 2]) -> Option<usize> {
        let to_id_f32 = convert_id_to_f32(to_id);
        self.genes.iter().position(|gene| {
            gene.id == from_id
                && GeneType::from_u8(gene.flag[0]) == Some(GeneType::Synapse)
                && gene.local_data == to_id_f32
        })
    }

    /// Helper function to get the gene type of a neuron.
    fn get_gene_type(&self, neuron_id: [u8; 2]) -> Option<GeneType> {
        self.genes
            .iter()
            .find(|gene| gene.id == neuron_id)
            .and_then(|gene| GeneType::from_u8(gene.flag[0]))
    }

    /// Helper function to find a gene by its ID and type (mutable version).
    fn find_gene_by_id_and_type_mut(
        &mut self,
        id: [u8; 2],
        types: &[GeneType],
    ) -> Option<(usize, &mut Gene)> {
        self.genes
            .iter_mut()
            .enumerate()
            .find(|(_, gene)| {
                gene.id == id && types.contains(&GeneType::from_u8(gene.flag[0]).unwrap())
            })
    }
    /// Creates a neuron from a parent-child pair
    fn create_neuron(&mut self, selected_pair: ([u8; 2], [u8; 2])) {
        if let Some((parent_index, parent_gene)) = self.find_gene_by_id_and_type(selected_pair.0, &[GeneType::Input, GeneType::Hidden, GeneType::Output]) {
            parent_gene.flag[1] += 1;
            self.genes.insert(
                parent_index + 1,
                Gene {
                    id: selected_pair.1,
                    flag: [GeneType::Hidden.as_u8(), 0],
                    local_data: 0.0,
                    extern_data: 0.0,
                },
            );
        } else {
            println!("Parent gene not found or invalid type.");
        }
    }

    /// Creates a synapse between two neurons
    fn create_synapse(&mut self, selected_pair: ([u8; 2], [u8; 2])) {
        let (from_id, to_id) = selected_pair;
        if let Some((index, _)) = self.find_gene_by_id(from_id) {
            let new_synapse = Gene {
                id: from_id,
                flag: [GeneType::Synapse.as_u8(), SynapseStatus::Enabled as u8],
                local_data: convert_id_to_f32(to_id),
                extern_data: 0.0,
            };
            self.genes.insert(index + 1, new_synapse);
        } else {
            println!("Source neuron not found.");
        }
    }

    /// Finds all possible parent-child pairs for neuron creation, also sort the return type by the height of the parent candidates
    fn get_neuron_candidates(&self) -> Vec<([u8; 2], [u8; 2])> {
        let mut candidates: Vec<([u8; 2], [u8; 2])> = self.genes
            .iter()
            .filter(|gene| {
                let gene_type = GeneType::from_u8(gene.flag[0]);
                (gene_type == Some(GeneType::Input) || gene_type == Some(GeneType::Hidden))
                    && gene.flag[1] < 2
                    && gene.id[1] <= 126  // Added condition to exclude genes with parent_id[1] > 127
            })
            .filter_map(|gene| {
                let parent_id = gene.id;
                let child_id = match gene.flag[1] {
                    0 => [gene.id[0], 2 * gene.id[1] + 1],
                    1 => [gene.id[0], 2 * gene.id[1] + 2],
                    _ => return None,
                };
                Some((parent_id, child_id))
            })
            .collect();
    
        // Sort the vector by the second value in the first tuple (parent_id[1])
        candidates.sort_by_key(|(parent_id, _)| get_neuron_height(parent_id[1]));
    
        candidates
    }
    
    /// Finds all possible synapse source neurons
    fn get_possible_synapse_sources(&self) -> Vec<[u8; 2]> {
        self.genes
            .iter()
            .filter(|gene| {
                let gene_type = GeneType::from_u8(gene.flag[0]);
                gene_type == Some(GeneType::Input) || gene_type == Some(GeneType::Hidden)
            })
            .map(|gene| gene.id)
            .collect()
    }

    /// Finds all possible synapse target neurons for a given source neuron
    fn get_synapse_candidates(&self, neuron_id: [u8; 2]) -> Vec<[u8; 2]> {
        let source_height = get_neuron_height(neuron_id[1]);

        self.genes
            .iter()
            .filter(|gene| {
                gene.id != neuron_id
                    && GeneType::from_u8(gene.flag[0]) != Some(GeneType::Synapse)
                    && !self.are_connected(neuron_id, gene.id)
                    && (get_neuron_height(gene.id[1]) > source_height || gene.flag[0] == 3)
            })
            .map(|gene| gene.id)
            .collect()
    }

    /// Checks if two neurons are already connected via a synapse
    fn are_connected(&self, from_id: [u8; 2], to_id: [u8; 2]) -> bool {
        self.find_synapse(from_id, to_id).is_some()
    }

    /// Selects a random element from a vector
    fn select_random<T: Clone>(&self, items: &[T]) -> Option<T> {
        let mut rng = thread_rng();
        items.choose(&mut rng).cloned()
    }

    /// Selects a random index from a vector of indices
    fn select_random_index(&self, indices: &[usize]) -> Option<usize> {
        let mut rng = thread_rng();
        indices.choose(&mut rng).cloned()
    }

    /// Selects a random parent-child pair
    fn select_random_pair(&self, pairs: &[([u8; 2], [u8; 2])]) -> Option<([u8; 2], [u8; 2])> {
        let mut rng = thread_rng();
        pairs.choose(&mut rng).copied()
    }

    /// Finds a gene by its ID
    fn find_gene_by_id(&mut self, id: [u8; 2]) -> Option<(usize, &mut Gene)> {
        self.genes.iter_mut().enumerate().find(|(_, gene)| gene.id == id)
    }

    /// Finds a gene by its ID and type
    fn find_gene_by_id_and_type(&mut self, id: [u8; 2], types: &[GeneType]) -> Option<(usize, &mut Gene)> {
        self.genes
            .iter_mut()
            .enumerate()
            .find(|(_, gene)| gene.id == id && types.contains(&GeneType::from_u8(gene.flag[0]).unwrap()))
    }

    /// Finds a synapse between two neurons
    fn find_synapse(&self, from_id: [u8; 2], to_id: [u8; 2]) -> Option<&Gene> {
        let to_id_f32 = convert_id_to_f32(to_id);
        self.genes.iter().find(|gene| {
            gene.id == from_id
                && GeneType::from_u8(gene.flag[0]) == Some(GeneType::Synapse)
                && gene.local_data == to_id_f32
        })
    }

    /// Gets indices of synapses based on their status (enabled or disabled)
    fn get_synapse_indices(&self, status: SynapseStatus) -> Vec<usize> {
        self.genes
            .iter()
            .enumerate()
            .filter(|(_, gene)| {
                GeneType::from_u8(gene.flag[0]) == Some(GeneType::Synapse)
                    && gene.flag[1] == status as u8
            })
            .map(|(index, _)| index)
            .collect()
    }
}

// Utility functions
/// Converts an ID ([u8; 2]) to f32
pub fn convert_id_to_f32(id: [u8; 2]) -> f32 {
    u16::from_be_bytes(id) as f32
}

/// Converts f32 back to an ID ([u8; 2])
pub fn convert_f32_to_id(value: f32) -> [u8; 2] {
    (value as u16).to_be_bytes()
}

/// Calculates the height of a neuron in the binary tree based on its position
pub fn get_neuron_height(position: u8) -> u32 {
    let mut height = 0;
    let mut index = position as u32;
    while index > 0 {
        index = (index - 1) / 2;
        height += 1;
    }
    height
}


pub fn get_inorder_position(id1: u16) -> u16 {
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