//! # SPI
//! The `spi` module includes structures and functions to utilize the Serial Peripheral Interface
//! (SPI) protocol.

use crate::Edge;

//
// Represents different SPI typestate configurations.
//

/// Represents a SPI Clock Polarity configuration.
pub trait ClockPolarity: private::Sealed {
    /// Gets the idle logical state.
    ///
    /// # Returns
    /// `true` if idle state is logical high.
    /// `false` if idle state is logical low.
    fn get_idle_state() -> bool;
}

/// A zero-sized typestate indicating a logic low clock idle state.
pub struct LowIdle;
impl ClockPolarity for LowIdle {
    /// Gets the idle logical state.
    ///
    /// # Returns
    /// `true` if idle state is logical high.
    /// `false` if idle state is logical low.
    fn get_idle_state() -> bool {
        false
    }
}

/// A zero-sized typestate indicating a logic high clock idle state.
pub struct HighIdle;
impl ClockPolarity for HighIdle {
    /// Gets the idle logical state.
    ///
    /// # Returns
    /// `true` if idle state is logical high.
    /// `false` if idle state is logical low.
    fn get_idle_state() -> bool {
        true
    }
}

/// Represents a SPI Clock Phase Configuration.
pub trait ClockPhase: private::Sealed {
    /// Gets the data sampling clock edge.
    ///
    /// # Returns
    /// `RisingEdge` if data is sampled at the rising edge.
    /// `FallingEdge if data is sampled at the falling edge.
    fn get_sample_edge() -> Edge;
}

/// A zero-size typedtate indicating data sampling on rising edge.
pub struct RisingEdgeSample;
impl ClockPhase for RisingEdgeSample {
    /// Gets the data sampling clock edge.
    ///
    /// # Returns
    /// `RisingEdge` if data is sampled at the rising edge.
    /// `FallingEdge if data is sampled at the falling edge.
    fn get_sample_edge() -> Edge {
        Edge::RisingEdge
    }
}

/// A zero-size typedtate indicating data sampling on falling edge.
pub struct FallingEdgeSample;
impl ClockPhase for FallingEdgeSample {
    /// Gets the data sampling clock edge.
    ///
    /// # Returns
    /// `RisingEdge` if data is sampled at the rising edge.
    /// `FallingEdge if data is sampled at the falling edge.
    fn get_sample_edge() -> Edge {
        Edge::FallingEdge
    }
}

/// A zero-sized typestate indicating the SPI instance configuration.
pub struct SpiMode<Polarity: ClockPolarity, Phase: ClockPhase> {
    _polarity: Polarity,
    _phase: Phase,
}

//
// For sealed traits.
//

mod private {
    pub trait Sealed {}
}

impl private::Sealed for LowIdle {}
impl private::Sealed for HighIdle {}
impl private::Sealed for RisingEdgeSample {}
impl private::Sealed for FallingEdgeSample {}
