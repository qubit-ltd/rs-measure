// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Text parsing helpers for typed measurements.

use rust_decimal::Decimal;

use super::compact_candidate::CompactCandidate;
use super::parse_decimal_text_exact;
use crate::measure::MeasurementError;
use crate::measure::MeasurementParseOptions;
use crate::measure::Unit;

/// Parses a measurement with canonical-only or lenient unit matching.
///
/// # Parameters
///
/// * `input` - Measurement text in compact or whitespace-separated form.
/// * `strict` - Whether compact aliases are excluded and the final unit is
///   parsed canonically.
/// * `options` - Resource limits applied before parsing.
///
/// # Returns
///
/// The exact Decimal value and typed unit.
///
/// # Errors
///
/// Returns a measurement parsing error for malformed numeric text, unknown or
/// non-canonical units, or multiple valid compact suffixes.
pub(in crate::measure) fn parse_measurement_text<U>(
    input: &str,
    strict: bool,
    options: &MeasurementParseOptions,
) -> Result<(Decimal, U), MeasurementError>
where
    U: Unit,
{
    if input.len() > options.max_text_bytes() {
        return Err(MeasurementError::MeasurementTextLimitExceeded {
            maximum: options.max_text_bytes(),
        });
    }
    if let Some((value_text, unit_text)) = split_spaced_measurement_parts(input) {
        return parse_measurement_parts::<U>(value_text, unit_text, strict);
    }

    let trimmed = input.trim();
    let mut first_candidate = None;
    let mut ambiguous_units = Vec::new();
    for (unit_index, unit) in U::all().iter().copied().enumerate() {
        collect_compact_candidate(
            trimmed,
            unit_index,
            unit.symbol(),
            &mut first_candidate,
            &mut ambiguous_units,
        );
        if !strict {
            for alias in unit.aliases() {
                collect_compact_candidate(
                    trimmed,
                    unit_index,
                    alias,
                    &mut first_candidate,
                    &mut ambiguous_units,
                );
            }
        }
    }

    if let Some((value, unit_index)) =
        resolve_compact_candidates(input, first_candidate, ambiguous_units)?
    {
        return Ok((value, U::all()[unit_index]));
    }
    if has_malformed_scientific_suffix(trimmed) {
        return Err(MeasurementError::InvalidMeasurementSyntax);
    }

    let (value_text, unit_text) =
        split_measurement_parts(input).ok_or(MeasurementError::InvalidMeasurementSyntax)?;
    parse_measurement_parts::<U>(value_text, unit_text, strict)
}

/// Detects an exponent marker whose required digits are visibly absent.
///
/// Known compact unit suffixes are resolved before this check, preserving unit
/// symbols such as `eV` and explicitly declared exponent-like suffixes.
///
/// # Parameters
///
/// * `input` - Trimmed measurement text with no recognized compact suffix.
///
/// # Returns
///
/// `true` when an `e` or `E` immediately after a valid coefficient is followed
/// by the end of input, whitespace, or a sign without exponent digits.
fn has_malformed_scientific_suffix(input: &str) -> bool {
    let Some(value_len) = decimal_prefix_len(input) else {
        return false;
    };
    let suffix = &input[value_len..];
    let Some(rest) = suffix
        .strip_prefix('e')
        .or_else(|| suffix.strip_prefix('E'))
    else {
        return false;
    };
    if rest.is_empty() || rest.starts_with(char::is_whitespace) {
        return true;
    }
    let Some(after_sign) = rest.strip_prefix(['+', '-']) else {
        return false;
    };
    !matches!(after_sign.as_bytes().first(), Some(b'0'..=b'9'))
}

/// Retains one valid compact numeric-prefix and unit-suffix interpretation.
///
/// # Parameters
///
/// * `input` - Trimmed compact measurement text.
/// * `unit_index` - Index of the typed unit owning `symbol`.
/// * `symbol` - Canonical symbol or accepted alias candidate.
/// * `first_candidate` - Slot retaining the first interpretation without heap
///   allocation.
/// * `ambiguous_units` - Symbol list whose backing storage is allocated only
///   for the second and later interpretations.
fn collect_compact_candidate(
    input: &str,
    unit_index: usize,
    symbol: &'static str,
    first_candidate: &mut Option<CompactCandidate>,
    ambiguous_units: &mut Vec<String>,
) {
    if symbol.starts_with(['.', '+', '-']) {
        return;
    }
    let Some(value_text) = input.strip_suffix(symbol) else {
        return;
    };
    if value_text.is_empty()
        || value_text.ends_with(char::is_whitespace)
        || value_text.ends_with('.')
    {
        return;
    }
    if let Ok(value) = parse_decimal_text_exact(value_text) {
        retain_compact_candidate(
            CompactCandidate {
                value,
                unit_index,
                symbol,
            },
            first_candidate,
            ambiguous_units,
        );
    }
}

/// Retains one candidate and allocates unit names only after ambiguity exists.
///
/// # Parameters
///
/// * `candidate` - Newly discovered compact interpretation.
/// * `first_candidate` - Slot retaining the first interpretation without heap
///   allocation.
/// * `ambiguous_units` - Symbol list whose backing storage is allocated only
///   for the second and later interpretations.
fn retain_compact_candidate(
    candidate: CompactCandidate,
    first_candidate: &mut Option<CompactCandidate>,
    ambiguous_units: &mut Vec<String>,
) {
    if let Some(first) = first_candidate.as_ref() {
        if ambiguous_units.is_empty() {
            ambiguous_units.push(first.symbol.to_owned());
        }
        ambiguous_units.push(candidate.symbol.to_owned());
    } else {
        *first_candidate = Some(candidate);
    }
}

/// Resolves zero, one, or multiple compact suffix interpretations.
///
/// # Parameters
///
/// * `input` - Original measurement text retained for an ambiguity error.
/// * `first_candidate` - First compact interpretation, if one exists.
/// * `ambiguous_units` - Owned symbols when multiple interpretations exist, or
///   an empty zero-capacity vector otherwise.
///
/// # Returns
///
/// The exact Decimal and unit index for one interpretation, or `None` when no
/// compact interpretation exists.
///
/// # Errors
///
/// Returns [`MeasurementError::AmbiguousMeasurement`] with every matched symbol
/// when multiple compact interpretations exist.
fn resolve_compact_candidates(
    input: &str,
    first_candidate: Option<CompactCandidate>,
    ambiguous_units: Vec<String>,
) -> Result<Option<(Decimal, usize)>, MeasurementError> {
    if ambiguous_units.is_empty() {
        Ok(first_candidate.map(|candidate| (candidate.value, candidate.unit_index)))
    } else {
        Err(MeasurementError::AmbiguousMeasurement {
            input: input.to_owned(),
            units: ambiguous_units,
        })
    }
}

/// Parses already separated Decimal and unit text.
///
/// # Parameters
///
/// * `value_text` - Exact Decimal candidate.
/// * `unit_text` - Canonical symbol or lenient alias candidate.
/// * `strict` - Whether aliases must be rejected.
///
/// # Returns
///
/// The exact Decimal value and parsed unit.
///
/// # Errors
///
/// Returns invalid-measurement, unknown-unit, or non-canonical-unit errors.
fn parse_measurement_parts<U>(
    value_text: &str,
    unit_text: &str,
    strict: bool,
) -> Result<(Decimal, U), MeasurementError>
where
    U: Unit,
{
    let value = parse_decimal_text_exact(value_text)?;
    let unit = if strict {
        U::parse_strict(unit_text)?
    } else {
        U::parse_lenient(unit_text)?
    };
    Ok((value, unit))
}

/// Splits only measurement text with whitespace before the unit suffix.
///
/// # Parameters
///
/// * `input` - Candidate measurement text.
///
/// # Returns
///
/// The Decimal and unit slices when whitespace explicitly separates them.
#[inline]
fn split_spaced_measurement_parts(input: &str) -> Option<(&str, &str)> {
    let trimmed = input.trim();
    let value_len = decimal_prefix_len(trimmed)?;
    let (value_text, unit_suffix) = trimmed.split_at(value_len);
    (unit_suffix.trim_start().len() != unit_suffix.len())
        .then(|| (value_text, unit_suffix.trim()))
        .filter(|(_, unit_text)| !unit_text.is_empty())
}

/// Splits a measurement string into decimal value text and trimmed unit text.
///
/// # Parameters
///
/// * `input` - Candidate measurement text.
///
/// # Returns
///
/// `Some((value, unit))` when a syntactically valid Decimal prefix and a
/// non-empty plausible unit suffix are present. Space-separated suffixes may
/// start with `.`, `+`, or `-`; compact suffixes starting with those reserved
/// characters return `None` to avoid accepting malformed Decimal text.
#[inline]
fn split_measurement_parts(input: &str) -> Option<(&str, &str)> {
    let trimmed = input.trim();
    let value_len = decimal_prefix_len(trimmed)?;
    let (value_text, unit_suffix) = trimmed.split_at(value_len);
    let is_separated = unit_suffix.trim_start().len() != unit_suffix.len();
    let unit_text = unit_suffix.trim();
    if unit_text.is_empty()
        || (!is_separated && (value_text.ends_with('.') || unit_text.starts_with(['.', '+', '-'])))
    {
        None
    } else {
        Some((value_text, unit_text))
    }
}

/// Returns the byte length of the leading decimal value.
///
/// # Parameters
///
/// * `input` - Text beginning with an optional signed Decimal.
///
/// # Returns
///
/// `Some(length)` for a valid Decimal prefix, including a valid exponent when
/// present; otherwise, `None`.
fn decimal_prefix_len(input: &str) -> Option<usize> {
    let bytes = input.as_bytes();
    let mut index = 0;
    if matches!(bytes.first(), Some(b'+' | b'-')) {
        index += 1;
    }

    let mut has_digit = false;
    let mut has_dot = false;
    while let Some(byte) = bytes.get(index) {
        match byte {
            b'0'..=b'9' => {
                has_digit = true;
                index += 1;
            }
            b'.' if !has_dot => {
                has_dot = true;
                index += 1;
            }
            b'e' | b'E' if has_digit => {
                if let Some(end) = exponent_end(bytes, index + 1) {
                    return Some(end);
                }
                break;
            }
            b'.' | b'+' | b'-' => return None,
            _ => break,
        }
    }

    has_digit.then_some(index)
}

/// Returns the end offset of a valid exponent suffix.
///
/// # Parameters
///
/// * `bytes` - Complete measurement input as bytes.
/// * `index` - Offset immediately after the exponent marker.
///
/// # Returns
///
/// `Some(end)` after at least one exponent digit, or `None` for an invalid
/// suffix.
fn exponent_end(bytes: &[u8], mut index: usize) -> Option<usize> {
    if matches!(bytes.get(index), Some(b'+' | b'-')) {
        index += 1;
    }

    let digits_start = index;
    while matches!(bytes.get(index), Some(b'0'..=b'9')) {
        index += 1;
    }
    (index > digits_start).then_some(index)
}
