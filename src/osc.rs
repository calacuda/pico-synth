use crate::{env::ADSR, synth::WAVE_TABLE_SIZE, SAMPLE_RATE};
use libm::powf;

#[derive(Clone, Copy, Debug)]
pub struct Overtone {
    /// the frequency of the overtone relative to the fundimental
    pub overtone: f64,
    /// how loud this over tone is relative to the total volume (ie, 1.0)
    pub volume: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct WavetableOscillator {
    sample_rate: f32,
    index: f32,
    index_increment: f32,
}

impl WavetableOscillator {
    pub fn new() -> Self {
        Self {
            sample_rate: SAMPLE_RATE as f32,
            // wave_table: Self::build_wave_table(overtones),
            index: 0.0,
            index_increment: 0.0,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * WAVE_TABLE_SIZE as f32 / self.sample_rate;
    }

    pub fn get_sample(&mut self, wave_table: &[f32]) -> f32 {
        let sample = self.lerp(wave_table);
        self.index += self.index_increment;
        self.index %= WAVE_TABLE_SIZE as f32;
        sample
    }

    fn lerp(&self, wave_table: &[f32]) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % WAVE_TABLE_SIZE;

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        truncated_index_weight * wave_table[truncated_index]
            + next_index_weight * wave_table[next_index]
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Oscillator {
    wt_osc: WavetableOscillator,
    env_filter: ADSR,
}

impl Oscillator {
    pub fn new() -> Self {
        Self {
            wt_osc: WavetableOscillator::new(),
            env_filter: ADSR::new(),
        }
    }

    pub fn is_pressed(&self) -> bool {
        self.env_filter.pressed()
    }

    pub fn press(&mut self, midi_note: u8) {
        self.env_filter.press();
        let frequency = Self::get_freq(midi_note);

        self.wt_osc.set_frequency(frequency);
    }

    fn get_freq(midi_note: u8) -> f32 {
        let exp = (f32::from(midi_note) + 36.376_316) / 12.0;
        // 2_f32.powf(exp)

        powf(2.0, exp)
    }

    pub fn release(&mut self) {
        self.env_filter.release();
    }

    pub fn get_sample(&mut self, wave_table: &[f32]) -> f32 {
        self.wt_osc.get_sample(wave_table) * self.env_filter.get_samnple()
    }
}
