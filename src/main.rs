
extern crate aster_ml;
use aster_ml::*;

fn main() {        
    // UH OH! DUPLICATE SYNAPSES!
    let mut network: Network = Network::new(2, 2);
    for i in 0..10{
        network.genome.create_rand_neuron_child();
    }
    
    network.parse_genome();
    network.visualize();    
    network.debug_display();
    network.genome.debug_display();

    
    for i in 0..10{
        network.genome.create_rand_synapse();
    }
    network.parse_genome();
    network.visualize();   
    network.debug_display();
    network.genome.debug_display();

}
