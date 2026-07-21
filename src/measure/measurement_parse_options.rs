// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Options controlling measurement text parsing.

/// Options controlling resource limits for measurement text parsing.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeasurementParseOptions {
    /// Inclusive byte limit applied to supplied measurement text.
    max_text_bytes: usize,
}

impl MeasurementParseOptions {
    /// Default maximum measurement text length in bytes.
    pub const DEFAULT_MAX_TEXT_BYTES: usize = 1_048_576;

    /// Returns the maximum accepted measurement text length in bytes.
    ///
    /// # Returns
    ///
    /// The configured inclusive byte limit.
    #[must_use]
    #[inline(always)]
    pub const fn max_text_bytes(&self) -> usize {
        self.max_text_bytes
    }

    /// Returns these options with a different measurement text byte limit.
    ///
    /// # Parameters
    ///
    /// * `maximum` - Inclusive maximum number of UTF-8 bytes.
    ///
    /// # Returns
    ///
    /// The updated options value.
    #[inline(always)]
    pub const fn with_max_text_bytes(mut self, maximum: usize) -> Self {
        self.max_text_bytes = maximum;
        self
    }
}

impl Default for MeasurementParseOptions {
    /// Creates options using [`Self::DEFAULT_MAX_TEXT_BYTES`].
    #[inline(always)]
    fn default() -> Self {
        Self {
            max_text_bytes: Self::DEFAULT_MAX_TEXT_BYTES,
        }
    }
}
