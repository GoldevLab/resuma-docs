//! Counter-page sizes from `resuma/benchmark/results.json`.
//!
//! Regenerated 2026-10-24 from `resuma/benchmark/results.json` (Resuma 2.0.1).
//! First interaction = `loader.js` + `core.js` + Counter handler chunk.

pub const RESUMA_INITIAL: &str = "1006 B";
pub const RESUMA_FIRST: &str = "10.30 KiB";
pub const RESUMA_STATIC: &str = "0 B";

pub const LOADER_RAW: &str = "2.05 KiB";
pub const LOADER_GZIP: &str = "1006 B";
pub const LOADER_BROTLI: &str = "840 B";

pub const CORE_RAW: &str = "26.81 KiB";
pub const CORE_GZIP: &str = "9.19 KiB";
pub const CORE_BROTLI: &str = "8.27 KiB";

pub const NEXT_GZIP: &str = "142.43 KiB";
/// 145851 B Next first-load gzip / 1006 B Resuma loader gzip.
pub const SMALLER_THAN_NEXT: &str = "145×";
