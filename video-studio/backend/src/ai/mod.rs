/// Professional AI-powered video editing tools
///
/// This module contains implementations of advanced AI features
/// for professional video editing, comparable to Adobe Premiere Pro
/// and DaVinci Resolve capabilities.

pub mod auto_reframe;
pub mod match_color;
pub mod track_object;
pub mod remove_background;
pub mod enhance_audio;
pub mod rough_cut;
pub mod motion_analysis;
pub mod cut_points;
pub mod upscale;
pub mod interpolation;
pub mod beat_detection;

pub use auto_reframe::*;
pub use match_color::*;
pub use track_object::*;
pub use remove_background::*;
pub use enhance_audio::*;
pub use rough_cut::*;
pub use motion_analysis::*;
pub use cut_points::*;
pub use upscale::*;
pub use interpolation::*;
pub use beat_detection::*;
