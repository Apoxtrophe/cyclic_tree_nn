extern crate aster_ml;
use aster_ml::*;

fn main() {        
    // UH OH! DUPLICATE SYNAPSES!
    let mut network: Network = Network::new(1, 1);
    for i in 0..1{
        network.genome.rand_connected_child();
    }
    //network.genome.display();
    let complexity = network.genome.get_complexity();
    println!("Neuron Complexity: {}", complexity);

    
    network.genome.statistics();
    
    //network.genome.disable_random_synapse();
    network.genome.disable_random_synapse();
    network.parse_genome();
    
    
    network.genome.visualize();
    network.genome.remove_random_disabled_synapse();
    network.genome.statistics();
    network.genome.visualize();
    network.genome.statistics();
}
