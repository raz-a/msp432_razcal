//! # Interrupt
//! The `interrupt` module includes structures and functions to configure interrupts.

use core::sync::atomic::{compiler_fence, Ordering};

pub struct SingleProcessorCriticalSectionToken {
    _unused: (),
}

/// Disables interrupts globally.
fn disable_interrupts() {
    unsafe { asm!("cpsid i") };
    compiler_fence(Ordering::SeqCst);
}

/// Enables interrupts globally.
fn enable_interrupts() {
    unsafe { asm!("cpsie i") };
    compiler_fence(Ordering::SeqCst);
}

/// Creates a single processor crtitical section.
///
/// # Arguments
/// `crtitical_section_function` - Provides a function to be executed in the context of a critical
///     section.
pub fn single_proc_critical_section<F: FnMut(SingleProcessorCriticalSectionToken)>(
    mut crtitical_section_function: F,
) {
    disable_interrupts();
    let critical_section_token = SingleProcessorCriticalSectionToken { _unused: () };
    crtitical_section_function(critical_section_token);
    enable_interrupts();
}
