# Essence

Essence is a fully-local CLI tool to help you summarize meetings. It works by transcribing the audio (using Whisper) of a meeting and then summarizing the transcript (using Ollama).

It's currently made to be used primary with Apple Silicon Macs since that's what I use at work.

## Usage

```bash
essence transcribe -i <audio_path> -l <language> -m <model_path>
essence summarize -i <transcript_path> -m <model_name>
```

## Development

```bash
git clone https://github.com/orellazri/essence.git
brew install cmake
cargo build
```
