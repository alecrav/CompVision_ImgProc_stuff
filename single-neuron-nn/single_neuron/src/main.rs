mod neuron;

fn train_epoch(neuron: &mut neuron::Neuron) -> f32 {

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
    let mut neuron = neuron::Neuron {
        weight: 0.1,
        bias: 0.1
    };
    
    for n in 0..50 {
        
        let loss = train_epoch(&mut neuron);
        println!("---------- EPOCH {} ----------", n);
        println!("{}, loss", loss);
        println!("{}, bias", neuron.bias);
        println!("{}, weight", neuron.weight);
    }

    // predict something
    let input = 100.0;
    let prediction = input * neuron.weight + neuron.bias;
    println!("prediction 1: {}", prediction);


    let input = 200.0;
    let prediction = input * neuron.weight + neuron.bias;
    println!("prediction 2: {}", prediction);
}
