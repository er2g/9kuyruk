#!/usr/bin/env python3
"""
Optimized Whisper Service for VPS (8GB RAM + 6 core E5)
Uses faster-whisper with int8 quantization for RAM efficiency
"""

from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from faster_whisper import WhisperModel
import uvicorn
import os

app = FastAPI(title="Whisper Transcription Service")

# Load model once at startup (int8 quantized, ~5GB RAM)
model = WhisperModel(
    "large-v3",
    device="cpu",
    compute_type="int8",  # Quantized for RAM efficiency
    num_workers=4,        # E5 6 core için optimal
    cpu_threads=4,
)

class TranscribeRequest(BaseModel):
    audio_path: str
    language: str = "auto"

class Segment(BaseModel):
    start: float
    end: float
    text: str

@app.post("/transcribe", response_model=list[Segment])
async def transcribe(request: TranscribeRequest):
    """Transcribe audio file to text with timestamps"""

    if not os.path.exists(request.audio_path):
        raise HTTPException(status_code=404, detail="Audio file not found")

    try:
        # Transcribe with optimizations
        segments, info = model.transcribe(
            request.audio_path,
            language=None if request.language == "auto" else request.language,
            beam_size=5,
            best_of=5,
            temperature=0.0,
            vad_filter=True,          # Skip silent parts
            vad_parameters=dict(
                min_silence_duration_ms=500,
            ),
            condition_on_previous_text=True,
        )

        result = []
        for segment in segments:
            result.append(Segment(
                start=segment.start,
                end=segment.end,
                text=segment.text.strip(),
            ))

        return result

    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/health")
async def health():
    return {"status": "ok", "model": "large-v3", "device": "cpu", "compute_type": "int8"}

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8001, workers=1)
