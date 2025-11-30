use std::cell::Cell;

pub struct Neuron {
    pub weight: f32,
    pub bias: f32,
}

pub trait ComputeNeuron {
    fn train(&self, input: f32) -> f32;
    fn compute_loss(&self, output: f32, real_value: f32) -> f32;
}

impl ComputeNeuron for Neuron {
    fn train(&self, input: f32) -> f32 {
        let output = (input * self.weight) + self.bias;
        output
    }

    fn compute_loss(&self, output: f32, real_value: f32) -> f32 {
        let loss = real_value - output;
        loss
    }
}
