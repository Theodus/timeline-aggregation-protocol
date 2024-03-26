// Copyright 2023-, Semiotic AI, Inc.
// SPDX-License-Identifier: Apache-2.0

//! # Receipt State
//!
//! These are the implementation of the typestate pattern for tracking the
//! state of a receipt.
//! The `ReceiptState` trait represents the different states a receipt can be in.

use crate::receipt::ReceiptError;

/// Checking state represents a receipt that is currently being checked.
#[derive(Debug, Clone)]
pub struct Checking;

/// Failed state represents a receipt that has failed a check or validation.
#[derive(Debug, Clone)]
pub struct Failed {
    /// A list of checks to be completed for the receipt, along with their
    /// current result
    pub error: ReceiptError,
}

/// AwaitingReserve state represents a receipt that has passed all checks
/// and is awaiting escrow reservation.
#[derive(Debug, Clone)]
pub struct AwaitingReserve;

/// Reserved state represents a receipt that has successfully reserved escrow.
#[derive(Debug, Clone)]
pub struct Reserved;

/// Trait for the different states a receipt can be in.
pub trait ReceiptState {}
impl ReceiptState for Checking {}
impl ReceiptState for AwaitingReserve {}
impl ReceiptState for Reserved {}
impl ReceiptState for Failed {}
