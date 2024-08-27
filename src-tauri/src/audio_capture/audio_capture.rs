use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, Stream};
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub struct AudioCapture {
    stream: Option<Stream>,
    recorded_data: Arc<Mutex<Vec<u8>>>,
}

impl AudioCapture {
    pub fn new() -> Self {
        Self {
            stream: None,
            recorded_data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or("Failed to find a default input device")?;
        let config = device
            .default_input_config()
            .map_err(|e| e.to_string())?;

        let sample_format = config.sample_format();
        let config = config.into();

        let recorded_data = Arc::clone(&self.recorded_data);

        let stream = match sample_format {
            SampleFormat::F32 => self.build_stream::<f32>(&device, &config, recorded_data),
            SampleFormat::I16 => self.build_stream::<i16>(&device, &config, recorded_data),
            SampleFormat::U16 => self.build_stream::<u16>(&device, &config, recorded_data),
        }.map_err(|e| e.to_string())?;

        stream.play().map_err(|e| e.to_string())?;
        self.stream = Some(stream);
        Ok(())
    }

    pub fn stop(&mut self) {
        self.stream = None;
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<(), String> {
        let recorded_data = self.recorded_data.lock().unwrap();
        let path = Path::new(file_path);
        let file = File::create(path).map_err(|e| e.to_string())?;
        let mut writer = BufWriter::new(file);

        // Ajustar o cabeçalho WAV aqui, dependendo do formato de áudio
        let header = wav_header(recorded_data.len() as u32);
        writer.write_all(&header).map_err(|e| e.to_string())?;
        writer.write_all(&recorded_data).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn build_stream<T>(&self, device: &cpal::Device, config: &cpal::StreamConfig, recorded_data: Arc<Mutex<Vec<u8>>>) -> Result<cpal::Stream, cpal::BuildStreamError>
    where
        T: cpal::Sample + Send + 'static,
    {
        device.build_input_stream(
            config,
            move |data: &[T], _: &cpal::InputCallbackInfo| {
                let mut recorded = recorded_data.lock().unwrap();
                for &sample in data {
                    let sample_i16 = (sample.to_f32() * i16::MAX as f32) as i16;
                    recorded.extend_from_slice(&sample_i16.to_le_bytes());
                }
            },
            move |err| {
                eprintln!("Error occurred on stream: {}", err);
            },
        )
    }
}

fn wav_header(data_len: u32) -> [u8; 44] {
    let mut header = [0u8; 44];
    let file_size = data_len + 36;
    header[..4].copy_from_slice(b"RIFF");
    header[4..8].copy_from_slice(&file_size.to_le_bytes());
    header[8..12].copy_from_slice(b"WAVE");
    header[12..16].copy_from_slice(b"fmt ");
    header[16..20].copy_from_slice(&16u32.to_le_bytes());
    header[20..22].copy_from_slice(&1u16.to_le_bytes());
    header[22..24].copy_from_slice(&1u16.to_le_bytes());
    header[24..28].copy_from_slice(&44100u32.to_le_bytes());
    header[28..32].copy_from_slice(&(44100u32 * 2).to_le_bytes());
    header[32..34].copy_from_slice(&2u16.to_le_bytes());
    header[34..36].copy_from_slice(&16u16.to_le_bytes());
    header[36..40].copy_from_slice(b"data");
    header[40..44].copy_from_slice(&data_len.to_le_bytes());
    header
}
