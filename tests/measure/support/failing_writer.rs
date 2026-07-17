// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Deterministically failing writer used by serialization error-path tests.

use std::io;

/// Writer that fails after accepting a configured number of bytes.
pub(crate) struct FailingWriter {
    /// Number of bytes that may still be written successfully.
    remaining: usize,
}

impl FailingWriter {
    /// Creates a writer that fails after `remaining` bytes.
    ///
    /// # Parameters
    ///
    /// * `remaining` - Number of bytes accepted before the first error.
    ///
    /// # Returns
    ///
    /// A deterministic writer for exercising serializer failures.
    pub(crate) const fn new(remaining: usize) -> Self {
        Self { remaining }
    }
}

impl io::Write for FailingWriter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        if self.remaining == 0 {
            return Err(io::Error::other("injected write failure"));
        }
        let written = buffer.len().min(self.remaining);
        self.remaining -= written;
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
