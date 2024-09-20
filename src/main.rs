
extern crate aster_ml;
use aster_ml::*;

fn main() {        
    // UH OH! DUPLICATE SYNAPSES!
    let mut network: Network = Network::new(5, 2);
    for i in 0..80{
        network.genome.create_rand_neuron_child();
    }
    
    network.parse_genome();
    network.display();
    network.genome.display();
    network.visualize();    
}
