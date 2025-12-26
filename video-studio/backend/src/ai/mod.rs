/// Professional AI-powered video editing tools (VPS-optimized)
///
/// Lightweight implementations optimized for 8GB RAM + 6 core E5 VPS
/// Heavy ML models removed, FFmpeg-based alternatives used

pub mod auto_reframe;
pub mod track_object;
pub mod chroma_key;
pub mod enhance_audio;
pub mod rough_cut;
pub mod motion_analysis;
pub mod cut_points;
pub mod beat_detection;

pub use auto_reframe::*;
pub use track_object::*;
pub use chroma_key::*;
pub use enhance_audio::*;
pub use rough_cut::*;
pub use motion_analysis::*;
pub use cut_points::*;
pub use beat_detection::*;
