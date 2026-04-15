//! Developer-facing JSON bridge driver for the first `emboss-r` method surface.

use std::io::{self, Read};

use emboss_r_bridge::protocol::{BridgeRequest, BridgeResponse};
use emboss_r_bridge::{
    charge_profile, new_sequence, not_sequence, nth_sequence, sequence_count, skip_sequences,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let request: BridgeRequest = serde_json::from_str(&input)?;

    let response = match request {
        BridgeRequest::NewSequence { record } => BridgeResponse::NewSequence {
            record: new_sequence(record)?,
        },
        BridgeRequest::SequenceCount { records } => BridgeResponse::SequenceCount {
            count: sequence_count(&records)?,
        },
        BridgeRequest::NthSequence { records, index } => BridgeResponse::NthSequence {
            record: nth_sequence(&records, index)?,
        },
        BridgeRequest::SkipSequences { records, count } => BridgeResponse::SkipSequences {
            records: skip_sequences(&records, count)?,
        },
        BridgeRequest::NotSequence { records, index } => BridgeResponse::NotSequence {
            records: not_sequence(&records, index)?,
        },
        BridgeRequest::ChargeProfile {
            record,
            window,
            step,
        } => BridgeResponse::ChargeProfile {
            profile: charge_profile(record, window, step)?,
        },
    };

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}
