use ::audiobuffer::AudioBuffer;
use std::fs::File;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;
use std::str;
use std::convert;
use std::io::SeekFrom;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Sample{
    pub data: AudioBuffer,
    pub freq: f32, // which frequency is this sound in, for example 440.0 for A4
    pub samplerate: f32, // whats the sampe rate
}

#[derive(Debug)]
pub enum WavError{
    RIFFHeaderMissing,
    WAVEHeaderMissing,
    FmtHeaderMissing,
    DataHeaderMissing,
    IO(::std::io::Error)
}

impl convert::From<::std::io::Error> for WavError{
    fn from(f: ::std::io::Error) -> Self{
        WavError::IO(f)
    }
}

impl convert::From<str::Utf8Error> for WavError{
    fn from(_f: str::Utf8Error) -> Self{
        WavError::RIFFHeaderMissing
    }
}

// Checks an str is now at the file. str of 4 bytes.
fn file_check_str4(file: &mut File, s: &str, error: WavError) -> Result<(), WavError>{
    let mut data:[u8; 4] = [0,0,0,0];
    file.read(&mut data)?;
    if str::from_utf8(&data)? != s {
        return Err(error);
    }
    Ok(())
}

impl Sample{
    pub fn read_wav(filename: &str, freq: f32) -> Result<Self, WavError>{
        let mut f = File::open(filename)?;

        file_check_str4(&mut f, "RIFF", WavError::RIFFHeaderMissing)?;
        let file_size = f.read_u32::<LittleEndian>()?;

        file_check_str4(&mut f, "WAVE", WavError::WAVEHeaderMissing)?;
        file_check_str4(&mut f, "fmt ", WavError::FmtHeaderMissing)?;
        let fmt_size = f.read_u32::<LittleEndian>()?;
        let fmt_size_position = 20;

        let format = f.read_u16::<LittleEndian>()?;
        let channels = f.read_u16::<LittleEndian>()?;
        let sample_rate = f.read_u16::<LittleEndian>()?;
        let bytes = f.read_u32::<LittleEndian>()?;
        let block_align = f.read_u16::<LittleEndian>()?;
        let bits_per_sample = f.read_u16::<LittleEndian>()?;

        println!("{} has {} bytes of data // {} format {} channels {} sample_rate {} {} {} bits_per_sample {}", filename, file_size, fmt_size, format, channels, sample_rate, bytes, block_align, bits_per_sample);

        // Might have longer header, not my problem
        f.seek(SeekFrom::Start((fmt_size_position + fmt_size) as u64))?;

        file_check_str4(&mut f, "data", WavError::DataHeaderMissing)?;
        let _data_size = f.read_u32::<LittleEndian>()?;

        let mut tmpdata = Vec::<f32>::new( ); // Could pre allocate size, but I dont trust the file at all
        loop{
            match f.read_i16::<LittleEndian>() {
                Err(_) => break,
                Ok(sample) => {
                    let fsample: f32 = (sample as f32) / 32000.0;
                    tmpdata.push(fsample)
                }
            }
        }
        let ab = AudioBuffer::from(tmpdata);

        Ok(Sample{
            data: ab,
            freq: freq,
            samplerate: sample_rate as f32,
        })
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn read_audio_file(){
        let _wav = Sample::read_wav("synth/004/A5.wav", 880.0 ).expect("Could not read A5.wav");


    }
}
