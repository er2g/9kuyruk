use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

/// AI Agent for intelligent video editing
/// Supports: OpenAI GPT-4, Gemini Pro
#[derive(Clone)]
pub struct AIAgent {
    provider: AIProvider,
    api_key: String,
    system_prompt: String,
    available_tools: Vec<AITool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AIProvider {
    OpenAI { model: String },
    Gemini { model: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AITool {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
    pub handler: String, // Function name to call
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AIRequest {
    pub prompt: String,
    pub context: Option<ProjectContext>,
    pub chain_of_thought: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectContext {
    pub timeline_clips: Vec<ClipInfo>,
    pub current_time: f64,
    pub selected_clips: Vec<String>,
    pub video_metadata: VideoMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClipInfo {
    pub id: String,
    pub track: String,
    pub start: f64,
    pub duration: f64,
    pub clip_type: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub duration: f64,
    pub codec: String,
}

impl AIAgent {
    pub fn new(provider: AIProvider, api_key: String) -> Self {
        let system_prompt = Self::create_system_prompt();
        let available_tools = Self::register_tools();

        Self {
            provider,
            api_key,
            system_prompt,
            available_tools,
        }
    }

    /// Professional video editing AI system prompt
    fn create_system_prompt() -> String {
        r#"You are an expert video editing assistant with deep knowledge of professional post-production workflows, similar to Adobe Premiere Pro and DaVinci Resolve.

## Your Capabilities

### Scene Understanding & Analysis
- Detect scene boundaries and shot changes
- Analyze scene composition (rule of thirds, leading lines, etc.)
- Identify objects, people, and actions in frames
- Track motion and camera movement
- Analyze audio (speech, music, ambient, silence)
- Detect color grading and lighting
- Understand narrative flow and pacing

### Professional Editing Operations
- Timeline manipulation (cut, trim, slip, slide, ripple, roll)
- Multi-camera editing and sync
- Color correction and grading (ACES workflow)
- Audio mixing and ducking
- Effect application and keyframing
- Transition creation
- Speed ramping and time remapping
- Rotoscoping and masking

### AI-Powered Tools
- Auto-reframe: Content-aware aspect ratio conversion
- Smart conform: Intelligent cropping for social media
- Auto-color match: Match color between clips
- Scene detection: Automatic cut point identification
- Speech-to-text: Professional subtitle generation
- Object tracking: Track subjects across frames
- Background removal: AI-powered rotoscoping
- Audio enhancement: Noise reduction, EQ, dynamics
- Upscaling: AI super-resolution
- Frame interpolation: Optical flow slow motion

### Workflow Understanding
- Proxy workflow for 4K/8K editing
- Color management (Rec.709, DCI-P3, Rec.2020)
- LUT application and creation
- Render queue optimization
- Export settings for different platforms
- Collaboration and version control

## Your Approach

When given a task, you should:

1. **Analyze Context**
   - Understand the current project state
   - Identify relevant clips and timeline structure
   - Consider user's intent and creative direction

2. **Chain of Thought**
   - Break down complex tasks into steps
   - Explain your reasoning
   - Consider multiple approaches
   - Choose optimal solution

3. **Execute with Tools**
   - Use available function calls
   - Apply professional best practices
   - Maintain non-destructive editing
   - Preserve quality throughout pipeline

4. **Validate Results**
   - Check technical correctness
   - Ensure creative coherence
   - Verify no artifacts or issues

## Communication Style

- Be concise and professional
- Use industry terminology correctly
- Explain technical decisions when needed
- Ask clarifying questions if ambiguous
- Suggest alternatives when appropriate

## Available Tools

You have access to all video editing functions through tool calls. Always use the appropriate tool rather than just describing what to do.

Remember: You are editing in a professional environment. Prioritize quality, maintain proper color management, and follow broadcast-safe standards."#.to_string()
    }

    /// Register all available AI tools (function calling)
    fn register_tools() -> Vec<AITool> {
        vec![
            // Scene Analysis
            AITool {
                name: "detect_scenes".to_string(),
                description: "Detect scene boundaries and shot changes in video".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "video_id": { "type": "string" },
                        "threshold": { "type": "number", "default": 0.3 },
                        "min_scene_length": { "type": "number", "default": 1.0 }
                    },
                    "required": ["video_id"]
                }),
                handler: "scene::detect_scenes".to_string(),
            },

            // Auto-Reframe (Content-Aware)
            AITool {
                name: "auto_reframe".to_string(),
                description: "Intelligently reframe video to different aspect ratio (e.g., 16:9 to 9:16) using AI to track subjects".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "clip_id": { "type": "string" },
                        "target_aspect": { "type": "string", "enum": ["16:9", "9:16", "1:1", "4:5"] },
                        "track_subject": { "type": "boolean", "default": true },
                        "motion_priority": { "type": "string", "enum": ["center", "action", "faces"], "default": "action" }
                    },
                    "required": ["clip_id", "target_aspect"]
                }),
                handler: "ai::auto_reframe".to_string(),
            },

            // Color Match
            AITool {
                name: "match_color".to_string(),
                description: "Match color and look between clips using AI".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "source_clip": { "type": "string" },
                        "target_clips": { "type": "array", "items": { "type": "string" } },
                        "match_type": { "type": "string", "enum": ["color", "tone", "full"], "default": "full" }
                    },
                    "required": ["source_clip", "target_clips"]
                }),
                handler: "ai::match_color".to_string(),
            },

            // Object Tracking
            AITool {
                name: "track_object".to_string(),
                description: "Track an object or person across frames for masking/effects".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "clip_id": { "type": "string" },
                        "start_frame": { "type": "number" },
                        "bounding_box": { "type": "object" },
                        "track_type": { "type": "string", "enum": ["point", "mask", "3d"] }
                    },
                    "required": ["clip_id", "start_frame", "bounding_box"]
                }),
                handler: "ai::track_object".to_string(),
            },

            // Background Removal
            AITool {
                name: "remove_background".to_string(),
                description: "AI-powered background removal (rotoscoping)".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "clip_id": { "type": "string" },
                        "quality": { "type": "string", "enum": ["draft", "preview", "final"], "default": "preview" },
                        "edge_refinement": { "type": "boolean", "default": true }
                    },
                    "required": ["clip_id"]
                }),
                handler: "ai::remove_background".to_string(),
            },

            // Speech to Text
            AITool {
                name: "generate_subtitles".to_string(),
                description: "Generate accurate subtitles from speech using Whisper".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "clip_id": { "type": "string" },
                        "language": { "type": "string", "default": "auto" },
                        "style": { "type": "string", "enum": ["simple", "karaoke", "word-by-word"] },
                        "speaker_detection": { "type": "boolean", "default": false }
                    },
                    "required": ["clip_id"]
                }),
                handler: "ai::generate_subtitles".to_string(),
            },

            // Audio Enhancement
            AITool {
                name: "enhance_audio".to_string(),
                description: "AI-powered audio enhancement (noise reduction, EQ, dynamics)".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "clip_id": { "type": "string" },
                        "profile": { "type": "string", "enum": ["voice", "music", "ambient", "auto"] },
                        "noise_reduction": { "type": "number", "min": 0, "max": 100 },
                        "eq_preset": { "type": "string" }
                    },
                    "required": ["clip_id"]
                }),
                handler: "ai::enhance_audio".to_string(),
            },

            // Timeline Operations
            AITool {
                name: "create_rough_cut".to_string(),
                description: "Create a rough cut assembly from script/transcript".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "script": { "type": "string" },
                        "available_clips": { "type": "array" },
                        "target_duration": { "type": "number" },
                        "pacing": { "type": "string", "enum": ["fast", "medium", "slow"] }
                    },
                    "required": ["script", "available_clips"]
                }),
                handler: "ai::create_rough_cut".to_string(),
            },

            // Motion Analysis
            AITool {
                name: "analyze_motion".to_string(),
                description: "Analyze camera and subject motion in clip".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "clip_id": { "type": "string" },
                        "detect_camera_movement": { "type": "boolean", "default": true },
                        "detect_subject_movement": { "type": "boolean", "default": true }
                    },
                    "required": ["clip_id"]
                }),
                handler: "ai::analyze_motion".to_string(),
            },

            // Smart Cut Detection
            AITool {
                name: "suggest_cut_points".to_string(),
                description: "AI suggests optimal cut points based on action, audio, and composition".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "clip_id": { "type": "string" },
                        "criteria": { "type": "array", "items": { "type": "string" } },
                        "min_clip_length": { "type": "number", "default": 2.0 }
                    },
                    "required": ["clip_id"]
                }),
                handler: "ai::suggest_cut_points".to_string(),
            },

            // Upscaling
            AITool {
                name: "upscale_video".to_string(),
                description: "AI super-resolution upscaling".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "clip_id": { "type": "string" },
                        "target_resolution": { "type": "string", "enum": ["1080p", "4K", "8K"] },
                        "model": { "type": "string", "enum": ["fast", "balanced", "quality"] }
                    },
                    "required": ["clip_id", "target_resolution"]
                }),
                handler: "ai::upscale_video".to_string(),
            },

            // Frame Interpolation
            AITool {
                name: "interpolate_frames".to_string(),
                description: "Optical flow frame interpolation for slow motion".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "clip_id": { "type": "string" },
                        "target_fps": { "type": "number" },
                        "quality": { "type": "string", "enum": ["draft", "preview", "final"] }
                    },
                    "required": ["clip_id", "target_fps"]
                }),
                handler: "ai::interpolate_frames".to_string(),
            },

            // Music Beat Detection
            AITool {
                name: "detect_beats".to_string(),
                description: "Detect music beats for syncing cuts to music".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "audio_clip_id": { "type": "string" },
                        "sensitivity": { "type": "number", "min": 0, "max": 1, "default": 0.5 }
                    },
                    "required": ["audio_clip_id"]
                }),
                handler: "ai::detect_beats".to_string(),
            },
        ]
    }

    /// Execute AI request with chain of thought
    pub async fn execute(&self, request: AIRequest) -> Result<AIResponse> {
        match &self.provider {
            AIProvider::OpenAI { model } => {
                self.execute_openai(model, request).await
            }
            AIProvider::Gemini { model } => {
                self.execute_gemini(model, request).await
            }
        }
    }

    async fn execute_openai(&self, model: &str, request: AIRequest) -> Result<AIResponse> {
        let client = reqwest::Client::new();

        let mut messages = vec![
            serde_json::json!({
                "role": "system",
                "content": self.system_prompt
            }),
        ];

        // Add project context if available
        if let Some(context) = &request.context {
            messages.push(serde_json::json!({
                "role": "system",
                "content": format!("Current project context:\n{}", serde_json::to_string_pretty(context)?)
            }));
        }

        messages.push(serde_json::json!({
            "role": "user",
            "content": request.prompt
        }));

        let body = serde_json::json!({
            "model": model,
            "messages": messages,
            "tools": self.available_tools.iter().map(|t| {
                serde_json::json!({
                    "type": "function",
                    "function": {
                        "name": t.name,
                        "description": t.description,
                        "parameters": t.parameters
                    }
                })
            }).collect::<Vec<_>>(),
            "tool_choice": "auto"
        });

        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;

        // Parse response and handle tool calls
        self.parse_ai_response(result).await
    }

    async fn execute_gemini(&self, model: &str, request: AIRequest) -> Result<AIResponse> {
        let client = reqwest::Client::new();

        // Gemini API format
        let body = serde_json::json!({
            "contents": [{
                "role": "user",
                "parts": [{ "text": request.prompt }]
            }],
            "systemInstruction": {
                "parts": [{ "text": self.system_prompt }]
            },
            "tools": [{
                "functionDeclarations": self.available_tools.iter().map(|t| {
                    serde_json::json!({
                        "name": t.name,
                        "description": t.description,
                        "parameters": t.parameters
                    })
                }).collect::<Vec<_>>()
            }]
        });

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            model, self.api_key
        );

        let response = client
            .post(&url)
            .json(&body)
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;

        self.parse_gemini_response(result).await
    }

    async fn parse_ai_response(&self, response: serde_json::Value) -> Result<AIResponse> {
        // Parse OpenAI response
        let choice = &response["choices"][0];
        let message = &choice["message"];

        let mut ai_response = AIResponse {
            text: message["content"].as_str().unwrap_or("").to_string(),
            tool_calls: Vec::new(),
            thinking_steps: Vec::new(),
        };

        // Extract tool calls
        if let Some(tool_calls) = message["tool_calls"].as_array() {
            for call in tool_calls {
                ai_response.tool_calls.push(ToolCall {
                    name: call["function"]["name"].as_str().unwrap().to_string(),
                    arguments: call["function"]["arguments"].as_str().unwrap().to_string(),
                });
            }
        }

        Ok(ai_response)
    }

    async fn parse_gemini_response(&self, response: serde_json::Value) -> Result<AIResponse> {
        // Parse Gemini response
        let candidate = &response["candidates"][0];
        let content = &candidate["content"];

        let mut ai_response = AIResponse {
            text: String::new(),
            tool_calls: Vec::new(),
            thinking_steps: Vec::new(),
        };

        if let Some(parts) = content["parts"].as_array() {
            for part in parts {
                if let Some(text) = part["text"].as_str() {
                    ai_response.text.push_str(text);
                }

                if let Some(function_call) = part.get("functionCall") {
                    ai_response.tool_calls.push(ToolCall {
                        name: function_call["name"].as_str().unwrap().to_string(),
                        arguments: serde_json::to_string(&function_call["args"])?,
                    });
                }
            }
        }

        Ok(ai_response)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AIResponse {
    pub text: String,
    pub tool_calls: Vec<ToolCall>,
    pub thinking_steps: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCall {
    pub name: String,
    pub arguments: String,
}
