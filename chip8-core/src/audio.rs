use core::time::Duration;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(all(feature = "xo-chip", feature = "serde"))]
use serde_big_array::BigArray;

// defaults to a ~440Hz (444.44) square wave
const PITCH_BIAS: f32 = 64.0;
pub const SAMPLE_RATE: u32 = 48000;
const CHIP8_AUDIO_BUFFER_SIZE: usize = 128;
const DEFAULT_CHIP8_AUDIO_BUFFER: [f32; CHIP8_AUDIO_BUFFER_SIZE] = [
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
];

#[cfg(feature = "xo-chip")]
fn byte_expand(b: u8) -> [f32; 8] {
    [
        (b >> 7) as f32,
        ((b >> 6) & 1) as f32,
        ((b >> 5) & 1) as f32,
        ((b >> 4) & 1) as f32,
        ((b >> 3) & 1) as f32,
        ((b >> 2) & 1) as f32,
        ((b >> 1) & 1) as f32,
        (b & 1) as f32,
    ]
}

#[cfg(not(feature = "xo-chip"))]
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Audio {
    next: usize,
}
#[cfg(feature = "xo-chip")]
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Audio {
    pitch: Option<f32>,
    #[cfg_attr(feature = "serde", serde(with = "BigArray"))]
    buffer: [f32; CHIP8_AUDIO_BUFFER_SIZE],
    next: usize,
}

impl Audio {
    #[cfg(not(feature = "xo-chip"))]
    #[inline]
    pub fn new() -> Self {
        Audio { next: 0 }
    }
    #[cfg(feature = "xo-chip")]
    #[inline]
    pub fn new() -> Self {
        Audio {
            pitch: None,
            buffer: DEFAULT_CHIP8_AUDIO_BUFFER,
            next: 0,
        }
    }

    #[cfg(feature = "xo-chip")]
    #[inline]
    pub(crate) fn set_pitch(&mut self, pitch: f32) {
        self.pitch = Some(pitch);
    }

    #[cfg(feature = "xo-chip")]
    #[inline]
    pub(crate) fn write_pattern(&mut self, pat: &[u8]) {
        assert!(pat.len() >= 16);
        for (idx, byte) in pat.iter().take(16).copied().enumerate() {
            self.buffer[(idx * 8)..(idx * 8 + 8)].copy_from_slice(&byte_expand(byte));
        }
    }

    pub(crate) fn read_samples_to(&mut self, dur: Duration, buf: &mut [f32]) -> usize {
        #[cfg(feature = "xo-chip")]
        let pitch = self.pitch.unwrap_or(PITCH_BIAS);
        #[cfg(not(feature = "xo-chip"))]
        let pitch = PITCH_BIAS;
        #[cfg(feature = "xo-chip")]
        let pattern = &self.buffer;
        #[cfg(not(feature = "xo-chip"))]
        let pattern = &DEFAULT_CHIP8_AUDIO_BUFFER;

        let mut next = self.next;
        let freq = 4000.0 * 2.0f32.powf((pitch - PITCH_BIAS) / 48.0);
        let samples = (SAMPLE_RATE as f64 * dur.as_secs_f64()) as usize;
        assert!(buf.len() >= samples);
        for sample in buf.iter_mut().take(samples) {
            *sample = pattern[next] * freq / SAMPLE_RATE as f32;
            next = (next + 1) % CHIP8_AUDIO_BUFFER_SIZE;
        }
        self.next = next;
        samples
    }

    #[cfg(feature = "alloc")]
    #[inline]
    pub(crate) fn get_samples(&mut self, dur: Duration) -> Vec<f32> {
        let mut out = alloc::vec![0.0; (SAMPLE_RATE as f64 * dur.as_secs_f64()) as usize];
        self.read_samples_to(dur, &mut out);
        out
    }
}

impl Default for Audio {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
