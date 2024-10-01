extern crate aster_ml;
use aster_ml::*;
use std::*;

fn main() {        
    let mut individual1 = Network::new(2, 2);
    let mut individual2 = Network::new(2, 2);
    

    let mut geneome1 = individual1.genome;
    let mut geneome2 = individual2.genome;
    
    for i in 0..14 {
        geneome1.rand_connected_child();
        geneome2.rand_connected_child();
    }
    println!("GENOME 1");
    geneome1.display();
    geneome1.visualize();
    println!("GENOME 2");
    geneome2.display();
    geneome2.visualize();
    println!("GENOME NEW");
    geneome1.crossover(&geneome2);
    
    geneome1.statistics();
}