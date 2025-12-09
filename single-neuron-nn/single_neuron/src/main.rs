use crate::neuron::{HiddenLayer, InputLayer, OutputLayer};

mod neuron;

fn train_epoch(neuron: &mut neuron::Neuron::<InputLayer>) -> f32 {

    let inputs = [1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0];
    let ouptuts = [1.0,4.0,6.0,8.0,10.0,12.0,14.0,16.0];

    let mut output = 0.0;
    let mut loss = 0.0;

    for n in 0..8 {
        let input = inputs[n];
        let real_value = ouptuts[n];

        output = neuron.train(input);
        loss = neuron.compute_loss(output, real_value);

        neuron.backward_pass(loss, input); 
    }

    loss
}

fn main() {

    // initialize 5 neurons: 2 first layer, 2 second layer, 1 third layer
    let mut neuron_one = neuron::Neuron::<InputLayer> {
        weight: 0.1,
        bias: 0.1,
        _layer: std::marker::PhantomData,
    };

    let mut output_neuron = neuron:: Neuron::<OutputLayer> {
        weight: 0.1,
        bias: 0.1,
        _layer: std::marker::PhantomData,
    };

    for n in 0..50 {
        
        let loss_one = train_epoch(&mut neuron_one);

        println!("---------- EPOCH {} ----------", n);
        println!("{}, loss", loss_one);
        println!("{}, bias", neuron_one.bias);
        println!("{}, weight", neuron_one.weight);
    }

    // predict something
    let input = 100.0;
    let prediction = input * neuron_one.weight + neuron_one.bias;
    println!("prediction 1: {}", prediction);


    let input = 200.0;
    let prediction = input * neuron_one.weight + neuron_one.bias;
    println!("prediction 2: {}", prediction);
}
