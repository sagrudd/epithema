//! JSON request and response types for the developer-facing bridge driver.

use serde::{Deserialize, Serialize};

use crate::types::{BridgeChargeProfile, BridgeSequenceInput, BridgeSequenceRecord};

/// JSON bridge request envelope.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "method", content = "params", rename_all = "snake_case")]
pub enum BridgeRequest {
    /// Create one validated sequence record.
    NewSequence {
        /// Input sequence payload.
        record: BridgeSequenceInput,
    },
    /// Count sequence records in a supplied in-memory collection.
    SequenceCount {
        /// Ordered in-memory sequence records.
        records: Vec<BridgeSequenceInput>,
    },
    /// Select the 1-based Nth sequence record.
    NthSequence {
        /// Ordered in-memory sequence records.
        records: Vec<BridgeSequenceInput>,
        /// 1-based index to select.
        index: usize,
    },
    /// Skip the first N sequence records.
    SkipSequences {
        /// Ordered in-memory sequence records.
        records: Vec<BridgeSequenceInput>,
        /// Number of leading records to skip.
        count: usize,
    },
    /// Return all sequence records except the supplied 1-based index.
    NotSequence {
        /// Ordered in-memory sequence records.
        records: Vec<BridgeSequenceInput>,
        /// 1-based index to exclude.
        index: usize,
    },
    /// Compute a sliding-window protein charge profile.
    ChargeProfile {
        /// Input protein sequence.
        record: BridgeSequenceInput,
        /// Sliding-window length.
        window: usize,
        /// Sliding-window step size.
        step: usize,
    },
}

/// JSON bridge response envelope.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "method", content = "result", rename_all = "snake_case")]
pub enum BridgeResponse {
    /// New sequence response.
    NewSequence {
        /// Created validated record.
        record: BridgeSequenceRecord,
    },
    /// Sequence count response.
    SequenceCount {
        /// Counted sequence records.
        count: usize,
    },
    /// Nth sequence response.
    NthSequence {
        /// Selected record.
        record: BridgeSequenceRecord,
    },
    /// Skip sequence response.
    SkipSequences {
        /// Remaining records after skipping.
        records: Vec<BridgeSequenceRecord>,
    },
    /// Not sequence response.
    NotSequence {
        /// Remaining records after exclusion.
        records: Vec<BridgeSequenceRecord>,
    },
    /// Charge-profile response.
    ChargeProfile {
        /// Charge-profile payload.
        profile: BridgeChargeProfile,
    },
}
