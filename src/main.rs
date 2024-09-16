
extern crate aster_ml;
use aster_ml::*;

fn main() {        

    // SYNAPSES ARE ERRONEOUSLY FALSE??? 
    let mut network: Network = Network::new(4, 2);
    for i in 0..10 {
        network.genome.create_rand_neuron_child();
    }
    
    network.parse_genome();
    network.display();
    network.genome.display();
    network.visualize();    
}
