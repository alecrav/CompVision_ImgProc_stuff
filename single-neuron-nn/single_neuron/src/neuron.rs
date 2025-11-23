pub struct Neuron {
    inputs: Vec<i32>,
}

pub trait computeNeuron {
    fn returnOutput(&self) -> i32;
}

impl computeNeuron for Neuron {
    fn returnOutput(&self) -> i32 {
        // take vector and do the stuff has to be done
    }
}