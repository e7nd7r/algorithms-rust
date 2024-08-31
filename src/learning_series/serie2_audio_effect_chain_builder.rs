pub trait Effect {
    fn apply(&self, sample: f32) -> f32;
}

pub struct Reverb {
    pub intensity: f32,
}

impl Reverb {
    pub fn new(intensity: f32) -> Reverb {
        Reverb { intensity }
    }
}

impl Effect for Reverb {
    fn apply(&self, sample: f32) -> f32{
        sample * (1.0 + self.intensity)
    }
}

pub struct Delay {
    pub delay_time: f32,
}

impl Delay {
   pub fn new(delay_time: f32) -> Self {
        Delay { delay_time }
    }
}

impl Effect for Delay {
    fn apply(&self, sample: f32) -> f32 {
        sample * (1.0 - self.delay_time) // Simplified delay effect
    }
}

pub struct EQ {
    pub gain: f32,
}

impl EQ {
    pub fn new(gain: f32) -> Self {
        EQ { gain }
    }
}

impl Effect for EQ {
    fn apply(&self, sample: f32) -> f32 {
        sample * self.gain // Simplified EQ effect
    }
}

pub struct EffectChainBuilder {
    pub effects: Vec<Box<dyn Effect>>,
}

impl EffectChainBuilder {
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
        }
    }

    pub fn add_reverb(mut self, intensity: f32) -> Self {
        self.effects.push(Box::new(Reverb::new(intensity)));

        self
    }

    pub fn add_delay(mut self, delay_time: f32) -> Self {
        self.effects.push(Box::new(Delay::new(delay_time)));

        self
    }

    pub fn add_eq(mut self, gain: f32) -> Self {
        self.effects.push(Box::new(EQ::new(gain)));

        self
    }

    pub fn build(self) -> EffectChain{
        EffectChain {
            effects: self.effects,
        }
    }
}


pub struct EffectChain {
    pub effects: Vec<Box<dyn Effect>>,
}

impl EffectChain {
    pub fn apply(&self, sample: f32) -> f32 {
        self.effects
            .iter()
            .fold(sample, |acc, effect| effect.apply(acc))
    }
}

#[cfg(test)]
mod tests {
    use super::EffectChainBuilder;

    #[test]
    fn effect_chain_builder() {
        let effect_chain = EffectChainBuilder::new()
            .add_reverb(0.5)
            .add_delay(2.)
            .add_eq(2.)
            .build();

        let processed_sample = effect_chain.apply(1.0);

        println!("Processed Sample: {}", processed_sample);
    }

}