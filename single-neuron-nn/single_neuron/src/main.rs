mod neuron;
use std::cell::Cell;

use neuron::ComputeNeuron;

fn main() {

    // initialize neuron
    let neuron = neuron::Neuron {
        weight: 0.1,
        bias: 0.1
    };

    let input = 2.0;
    let real_value = 5.0;

    let output = neuron.train(input);

    neuron.compute_loss(output, real_value);
}