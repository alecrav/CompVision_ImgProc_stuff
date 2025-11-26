mod neuron;
use neuron::compute_neuron;

fn main() {
    let inputs: Vec<i32> = (0..3).map(|v| v + 1000).collect();

    // initialize neuron
    let neuron = neuron::Neuron {
        inputs
    };
    
    // call compute neuron
    let res: i32 = neuron.returnOutput();
}