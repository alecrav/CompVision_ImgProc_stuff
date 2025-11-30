use std::cell::Cell;

pub struct Neuron {
    pub weight: f32,
    pub bias: f32,
}

impl Neuron {
    pub fn train(&self, input: f32) -> f32 {
        let output = (input * self.weight) + self.bias;
        output
    }

    pub fn compute_loss(&self, output: f32, real_value: f32) -> f32 {
        let loss = real_value - output;
        loss
    }

    /// update weight and bias according to calculaed loss
    pub fn backward_pass(&mut self, loss: f32, input: f32) {
        self.weight = self.weight + (input * 0.01 * loss);
        self.bias = self.bias + (0.01 * loss);
    }
}
