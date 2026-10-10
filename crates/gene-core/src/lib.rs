//! Gene M0 simulation core. See docs/SPEC.md for the normative rules.
//!
//! The core has no dependency on rendering or observation consumers.
//! Observation never feeds back into the world.

pub mod agent;
pub mod behavior;
pub mod bonds;
pub mod contact;
pub mod effects;
pub mod env;
pub mod genome;
pub mod intent;
pub mod isa;
pub mod media;
pub mod motion;
pub mod mutation;
pub mod observer;
pub mod parcels;
pub mod records;
pub mod relations;
pub mod repro;
pub mod rng;
pub mod sim;
pub mod species;
pub mod vm;
pub mod world;

pub use sim::{Config, Sim, SimState};

/// Founder-4 assembly source (public ancestor).
pub const FOUNDER_ASM: &str = include_str!("../../../assets/founder4.asm");

/// Founder-5 assembly source: energy-aware revision of Founder-4 (SPEC §16b).
pub const FOUNDER5_ASM: &str = include_str!("../../../assets/founder5.asm");

/// Founder-6 assembly source: morning-only reproduction (SPEC §16c).
pub const FOUNDER6_ASM: &str = include_str!("../../../assets/founder6.asm");
pub const FOUNDER7_ASM: &str = include_str!("../../../assets/founder7.asm");
