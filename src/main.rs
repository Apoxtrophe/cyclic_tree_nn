
extern crate aster_ml;
use aster_ml::*;

fn main() {        

    // SYNAPSES ARE ERRONEOUSLY FALSE??? 
    let mut network: Network = Network::new(2, 2);
    for i in 0..4 {
        network.genome.create_rand_neuron_child();
    }
    
    network.genome.display();
    //network.genome.debug_display();
    network.parse_genome();
    
    network.display();  
    

}
