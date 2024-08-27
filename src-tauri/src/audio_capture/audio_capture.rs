use multichannel_audio::audio_class::AudioInstance;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use lame::Lame;

/// A struct to handle audio capture and processing.
pub struct AudioCapture {
    audio_instance: Option<AudioInstance>,
    recorded_data: Arc<Mutex<Vec<i32>>>, 
}

impl AudioCapture {
    pub fn new() -> Self {
        Self {
            audio_instance: None,
            recorded_data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        let sample_rate = 48_000;
        let audio_instance = AudioInstance::new(sample_rate)
            .map_err(|e| e.to_string())?;

        // Start recording for 5 seconds
        let recording_duration = 5.0;
        let recording = audio_instance.record(recording_duration)
            .map_err(|e| e.to_string())?;

        // Process and store the recorded data
        let mut data = self.recorded_data.lock().unwrap();
        data.clear(); 

        for sample_vec in recording {
            for sample in sample_vec {
                data.push(sample);
            }
        }

      

        self.audio_instance = Some(audio_instance);
        Ok(())
    }

    pub fn stop(&mut self) {
        self.audio_instance = None;
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<(), String> {
        let recorded_data = self.recorded_data.lock().unwrap();
        let path = Path::new(file_path);
        let file = File::create(path).map_err(|e| e.to_string())?;
        let mut writer = BufWriter::new(file);

        let mut encoder = Lame::new().ok_or("Failed to create Lame encoder".to_string())?;
        encoder.set_channels(2).map_err(|e| format!("Failed to set channels: {:?}", e))?;
        encoder.set_sample_rate(44_100).map_err(|e| format!("Failed to set sample rate: {:?}", e))?;
        encoder.set_quality(5).map_err(|e| format!("Failed to set quality: {:?}", e))?;

        // Convert raw data to PCM format
    let pcm_data: Vec<i16> = recorded_data
            .iter()
            .map(|&sample| sample as i16) // Convert each i32 sample to i16
            .collect();

        // Encode PCM data to MP3
        let mut mp3_buffer = vec![0u8; pcm_data.len() * 2];
        let num_bytes = encoder.encode(&pcm_data, &[], &mut mp3_buffer)
            .map_err(|e| format!("Failed to encode MP3: {:?}", e))?;

        writer.write_all(&mp3_buffer[..num_bytes]).map_err(|e| e.to_string())?;
        Ok(())
    }
}
