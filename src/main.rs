extern crate aster_ml;
use aster_ml::*;

fn main() {        
    // UH OH! DUPLICATE SYNAPSES!
    let mut network: Network = Network::new(10, 1);
    for i in 0..120{
        network.genome.create_rand_neuron_child();
        println!("{}",i);
        network.genome.display();
    }
    //network.genome.display();
    network.parse_genome();
    network.genome.visualize();   
}
