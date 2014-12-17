use std::borrow::Cow;
use std::mem;

/// Format that each sample has.
#[deriving(Clone, Copy, Show, PartialEq, Eq)]
pub enum SampleFormat {
    /// The value 0 corresponds to 0.
    I16,
    /// The value 0 corresponds to 32768.
    U16,
    F32,
}

impl SampleFormat {
    pub fn get_sample_size(&self) -> uint {
        match self {
            &SampleFormat::I16 => mem::size_of::<i16>(),
            &SampleFormat::U16 => mem::size_of::<u16>(),
            &SampleFormat::F32 => mem::size_of::<f32>(),
        }
    }
}

/// Trait for containers that contain PCM data.
#[unstable = "Will be rewritten with associated types"]
pub trait Sample: Copy {
    fn get_format(Option<Self>) -> SampleFormat;

    /// Turns the data into a `Vec<i16>` where each element is a sample.
    fn to_vec_i16(&[Self]) -> Cow<Vec<i16>, [i16]>;
    /// Turns the data into a `Vec<u16>` where each element is a sample.
    fn to_vec_u16(&[Self]) -> Cow<Vec<u16>, [u16]>;
    /// Turns the data into a `Vec<f32>` where each element is a sample.
    fn to_vec_f32(&[Self]) -> Cow<Vec<f32>, [f32]>;
}

impl Sample for u16 {
    fn get_format(_: Option<u16>) -> SampleFormat {
        SampleFormat::U16
    }

    fn to_vec_i16(input: &[u16]) -> Cow<Vec<i16>, [i16]> {
        Cow::Owned(input.iter().map(|&value| {
            if value >= 32768 {
                (value - 32768) as i16
            } else {
                (value as i16) - 32767
            }
        }).collect())
    }

    fn to_vec_u16(input: &[u16]) -> Cow<Vec<u16>, [u16]> {
        Cow::Borrowed(input)
    }

    fn to_vec_f32(input: &[u16]) -> Cow<Vec<f32>, [f32]> {
        Cow::Owned(Sample::to_vec_f32(Sample::to_vec_i16(input).as_slice()).to_vec())
    }
}

impl Sample for i16 {
    fn get_format(_: Option<i16>) -> SampleFormat {
        SampleFormat::I16
    }

    fn to_vec_i16(input: &[i16]) -> Cow<Vec<i16>, [i16]> {
        Cow::Borrowed(input)
    }

    fn to_vec_u16(input: &[i16]) -> Cow<Vec<u16>, [u16]> {
        Cow::Owned(input.iter().map(|&value| {
            if value < 0 {
                (value + 32767) as u16
            } else {
                (value as u16) + 32768
            }
        }).collect())
    }

    fn to_vec_f32(input: &[i16]) -> Cow<Vec<f32>, [f32]> {
        Cow::Owned(input.iter().map(|&value| {
            value as f32 / 32768.0
        }).collect())
    }
}

impl Sample for f32 {
    fn get_format(_: Option<f32>) -> SampleFormat {
        SampleFormat::F32
    }

    fn to_vec_i16(input: &[f32]) -> Cow<Vec<i16>, [i16]> {
        unimplemented!()
    }

    fn to_vec_u16(input: &[f32]) -> Cow<Vec<u16>, [u16]> {
        unimplemented!()
    }

    fn to_vec_f32(input: &[f32]) -> Cow<Vec<f32>, [f32]> {
        Cow::Borrowed(input)
    }
}