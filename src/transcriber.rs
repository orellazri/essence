use hound::WavReader;
use log::info;
use whisper_rs::{FullParams, SamplingStrategy};
use whisper_rs::{WhisperContext, WhisperContextParameters, WhisperState};

use crate::error::Error;

pub struct Transcriber {
    state: WhisperState,
}

impl Transcriber {
    pub fn new(model_path: &str) -> Result<Self, Error> {
        let ctx = WhisperContext::new_with_params(&model_path, WhisperContextParameters::default())
            .map_err(|e| Error::new(&format!("Failed to load model: {}", e)))?;

        let state = ctx
            .create_state()
            .map_err(|e| Error::new(&format!("Failed to create state: {}", e)))?;

        Ok(Self { state })
    }

    pub fn transcribe(&mut self, audio_path: &str, language: &str) -> Result<String, Error> {
        info!("Opening wav file");

        let samples: Vec<i16> = WavReader::open(audio_path)
            .map_err(|e| Error::new(&format!("Failed to open wav file: {}", e)))?
            .into_samples::<i16>()
            .map(|x| x.map_err(|e| Error::new(&format!("Failed to read wav file: {}", e))))
            .collect::<Result<Vec<i16>, Error>>()?;

        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(Some(&language));
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);

        info!("Converting audio to 16KHz mono f32 samples");

        // We need to convert the audio to 16KHz mono f32 samples for the model
        let mut inter_samples = vec![Default::default(); samples.len()];
        whisper_rs::convert_integer_to_float_audio(&samples, &mut inter_samples)
            .map_err(|e| Error::new(&format!("Failed to convert audio data: {}", e)))?;
        let samples = whisper_rs::convert_stereo_to_mono_audio(&inter_samples)
            .map_err(|e| Error::new(&format!("Failed to convert audio data: {}", e)))?;

        info!("Transcribing audio");

        self.state
            .full(params, &samples[..])
            .map_err(|e| Error::new(&format!("Failed to run model: {}", e)))?;

        let num_segments = self
            .state
            .full_n_segments()
            .map_err(|e| Error::new(&format!("Failed to get number of segments: {}", e)))?;

        info!("Collecting transcript from segments");

        let transcript = (0..num_segments)
            .map(|i| {
                self.state
                    .full_get_segment_text(i)
                    .map_err(|e| Error::new(&format!("Failed to get segment: {}", e)))
            })
            .collect::<Result<Vec<String>, Error>>()?
            .join("\n");

        info!("Transcription complete");

        Ok(transcript)
    }
}
