pub struct Neuron {
    pub inputs: Vec<i32>
}

pub trait compute_neuron {
    fn returnOutput(&self) -> i32;
}

impl compute_neuron for Neuron {
    fn returnOutput(&self) -> i32 {
        // take vector and do the stuff has to be done
        print!("compute stuff");
        3
    }
}