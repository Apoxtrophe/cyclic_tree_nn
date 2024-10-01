extern crate aster_ml;
use aster_ml::*;
use std::*;

fn main() {        
    let mut network: Network = Network::new(1, 1);
    for i in 0..16{
        network.genome.rand_connected_child();
    }

    //network.genome.display();
    let complexity = network.genome.get_complexity();
    println!("Neuron Complexity: {}", complexity);
    network.parse_genome();
    
    network.genome.visualize();
    network.genome.remove_random_disabled_synapse();
    network.genome.statistics();
    network.genome.display();
}
